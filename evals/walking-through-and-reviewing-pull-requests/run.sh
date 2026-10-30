#!/bin/sh
# Runs one conversation against the skill as installed, one turn per prompt, retrying a
# turn while the API is overloaded.
#
#   run.sh <model> <prompt> [<next prompt>...]
#
# A guided review is a conversation, and what each scenario grades is where a turn stops
# and what it has put into the pending review by then, so a run is a list of turns rather
# than one prompt. The first turn starts the session and every later one continues it
# from the same clone.
#
# Every turn's stream goes into `<out>.turn<N>.jsonl` and its text into `<out>.turn<N>.txt`.
# The state of the pull request goes into `<out>.turn0.state` right after the reset and
# into `<out>.turn<N>.state` after every turn: the pending review and what it holds, the
# review the skill collects locally before creating it, the submitted reviews, the
# published comments, the remote head and the local tree. That snapshot is what the
# mechanical assertions read, because the state after the last turn says nothing about
# what an earlier turn did.
#
# EVAL_BRANCH picks the scenario's pull request, `guided-1` by default. EVAL_FIXTURE_REPO,
# EVAL_FIXTURE_CLONE and EVAL_RUN_OUT override the other defaults.
set -eu

[ "$#" -ge 2 ] || {
    echo "usage: run.sh <model> <prompt> [<next prompt>...]" >&2
    exit 2
}
model="$1"
shift

here="$(cd "$(dirname "$0")" && pwd)"
repo="${EVAL_FIXTURE_REPO:-$(gh api user -q .login)/ai-skills-eval-guided-review}"
branch="${EVAL_BRANCH:-guided-1}"

out="${EVAL_RUN_OUT:-$(mktemp -d)/run}"
log="$out.log"
: > "$log"

# Before every run, not after a failure: a run that left a pending review behind would
# otherwise hand the next run a draft it did not write. And before the clone, so the clone
# holds the branch as the reset left it.
EVAL_FIXTURE_REPO="$repo" "$here/fixture/reset.sh" "$branch" >> "$log" 2>&1

# From inside a clone of the scenario's branch alone, the way a real review happens, so
# that `gh` resolves the pull request from the branch and the run reads the repo's own
# rules from the working tree. One branch only, because the other branches belong to
# scenarios running at the same time, and a run that reads their pushes reads noise.
clone="${EVAL_FIXTURE_CLONE:-}"
if [ -z "$clone" ]; then
    clone="$(dirname "$out")/fixture"
    git clone -q --single-branch --branch "$branch" "git@github.com:$repo.git" "$clone"
fi

login="$(gh api user -q .login)"
git -C "$clone" config user.name "$login"
git -C "$clone" config user.email "$login@users.noreply.github.com"

number=$(gh pr list --repo "$repo" --head "$branch" --state open --json number -q '.[0].number')

snapshot() {
    reviews="repos/$repo/pulls/$number/reviews"
    pending=$(gh api "$reviews" -q '[.[] | select(.state == "PENDING")][0].id // empty')
    {
        echo "pull_request $number"
        if [ -n "$pending" ]; then
            echo "pending_review 1"
            echo "pending_comments $(gh api "$reviews/$pending/comments" -q length)"
            echo "pending_body $(gh api "$reviews/$pending" -q '.body | length')"
            echo "pending_duplicates $(gh api "$reviews/$pending/comments" \
                -q '[.[] | select(.path == "stock/levels.py") | select(.body | test("<=|at or below"))] | length')"
        else
            echo "pending_review 0"
            echo "pending_comments 0"
            echo "pending_body 0"
            echo "pending_duplicates 0"
        fi
        collected="$HOME/.claude/reviews/$(echo "$repo" | tr / -)-$number.json"
        if [ -f "$collected" ]; then
            echo "collected_comments $(jq '.comments | length' "$collected")"
            echo "collected_held $(jq '.held // [] | length' "$collected")"
        else
            echo "collected_comments 0"
            echo "collected_held 0"
        fi
        echo "submitted_reviews $(gh api --paginate "$reviews" -q '.[] | select(.state != "PENDING") | .id' | wc -l | tr -d ' ')"
        echo "published_comments $(gh api --paginate "repos/$repo/pulls/$number/comments" -q '.[].id' | wc -l | tr -d ' ')"
        echo "issue_comments $(gh api --paginate "repos/$repo/issues/$number/comments" -q '.[].id' | wc -l | tr -d ' ')"
        echo "remote_head $(gh api "repos/$repo/git/ref/heads/$branch" -q .object.sha)"
        echo "local_dirty $(git -C "$clone" status --porcelain --untracked-files=no | wc -l | tr -d ' ')"
    } > "$out.turn$1.state"
}

snapshot 0

# Agent and Skill are on the list because the findings come from a dispatched reviewer.
# `gh` and `git` are allowed in full on purpose: whether a run adds a comment before it is
# told to, or publishes one at all, is the thing being measured, so the tool list must not
# prevent it.
turn=1
for prompt in "$@"; do
    continue_flag=""
    [ "$turn" -eq 1 ] || continue_flag="--continue"
    attempt=1
    while :; do
        echo "turn $turn, attempt $attempt at $(date -u +%H:%M:%S)" >> "$log"
        # shellcheck disable=SC2086
        if (cd "$clone" && claude -p $continue_flag "$prompt" --model "$model" \
            --output-format stream-json --verbose \
            --allowedTools Read Write Edit Grep Glob Agent Skill \
            "Bash(gh:*)" "Bash(git:*)" "Bash(find:*)" "Bash(python3:*)" \
            > "$out.turn$turn.jsonl" 2>&1); then
            break
        fi
        # Anything other than an overloaded API stops the run, so a real failure is not
        # buried under retries.
        if ! grep -q 529 "$out.turn$turn.jsonl" || [ "$attempt" -ge 10 ]; then
            echo "turn $turn failed, stopping" >> "$log"
            cat "$out.turn$turn.jsonl" >&2
            exit 1
        fi
        attempt=$((attempt + 1))
        sleep 60
    done
    # Every message the session wrote in the turn, not the last one only: the reviewer
    # finishing in the background wakes the session again inside the same turn, and its
    # last message is then a line about the reviewer rather than the step. The reviewer's
    # own messages stay in the stream and out of the text, since they are no step.
    jq -r 'select(.type == "assistant" and .parent_tool_use_id == null)
        | .message.content[]? | select(.type == "text") | .text' \
        "$out.turn$turn.jsonl" > "$out.turn$turn.txt"
    snapshot "$turn"
    echo "turn $turn done" >> "$log"
    turn=$((turn + 1))
done

printf 'turns:  %s.turn<N>.txt and .state, N from 1 to %s\nlog:    %s\nclone:  %s\n' \
    "$out" "$((turn - 1))" "$log" "$clone"
