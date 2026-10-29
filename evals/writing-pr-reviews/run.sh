#!/bin/sh
# Runs one scenario against the skill as installed, one turn per prompt, retrying a turn
# while the API is overloaded.
#
#   run.sh <model> <prompt> [<next prompt>...]
#
# Prints where it left the output, the log and the clone. The clone matters after the
# run and is deliberately not cleaned up: the JSON the skill posted from sits in it, and
# a pending review's anchor can only be graded from there, since the API reports
# `position` alone for a draft.
#
# A single prompt writes its reply to `run.txt`, which is what every single-turn scenario
# grades. More than one makes a conversation: the first turn starts the session, each
# later one continues it from the same clone, and turn N writes `run.turn<N>.txt` plus
# `run.turn<N>.state`, the state of the pull request right after it. The state after the
# last turn says nothing about what an earlier one did, such as a thread resolved before
# its yes, so each turn gets its own.
#
# EVAL_FIXTURE_REPO, EVAL_FIXTURE_CLONE and EVAL_RUN_OUT override the defaults.
set -eu

[ "$#" -ge 2 ] || {
    echo "usage: run.sh <model> <prompt> [<next prompt>...]" >&2
    exit 2
}
model="$1"
shift
turns=$#

here="$(cd "$(dirname "$0")" && pwd)"
repo="${EVAL_FIXTURE_REPO:-$(gh api user -q .login)/ai-skills-eval-fixture}"

# The state needs the pull request, and every scenario names it in its prompt. A
# conversation without one stops here rather than writing empty states after each turn.
number=""
if [ "$turns" -gt 1 ]; then
    number=$(printf '%s' "$1" | grep -oE '#[0-9]+' | head -1 | tr -d '#' || true)
    [ -n "$number" ] || {
        echo "run.sh: a run of several turns needs '#<number>' in its first prompt" >&2
        exit 2
    }
fi

# A run happens from inside a clone, the way a real review does, so that `gh` resolves
# the fixture from the remote rather than whatever repo this was invoked from.
clone="${EVAL_FIXTURE_CLONE:-}"
if [ -z "$clone" ]; then
    clone="$(mktemp -d)/fixture"
    git clone -q "git@github.com:$repo.git" "$clone"
fi

out="${EVAL_RUN_OUT:-$(dirname "$clone")/run.txt}"
log="$out.log"
: > "$log"

# Before every run, not after a failure. GitHub allows one pending review per user per
# pull request, so the draft the previous run left behind makes this one's POST fail in
# a way that reads exactly like the skill getting delivery wrong.
#
# EVAL_SKIP_RESET exists for the scenarios that seed the pull request themselves, 07 and
# 09, which run reset.sh before seeding. Skipping the clearing anywhere else reintroduces
# the failure the clearing is here to prevent.
if [ -z "${EVAL_SKIP_RESET:-}" ]; then
    "$here/fixture/reset.sh" >> "$log" 2>&1
else
    echo "reset skipped: EVAL_SKIP_RESET set" >> "$log"
fi

collected="$HOME/.claude/reviews/${repo%/*}-${repo#*/}-$number.json"

snapshot() {
    reviews=$(gh api "repos/$repo/pulls/$number/reviews")
    pending=$(printf '%s' "$reviews" | jq -r '.[] | select(.state == "PENDING") | .id')
    {
        echo "pending_reviews $(printf '%s' "$reviews" | jq '[.[] | select(.state == "PENDING")] | length')"
        if [ -n "$pending" ]; then
            echo "pending_comments $(gh api "repos/$repo/pulls/$number/reviews/$pending/comments" -q length)"
        else
            echo "pending_comments -"
        fi
        echo "submitted_reviews $(printf '%s' "$reviews" | jq '[.[] | select(.state != "PENDING")] | length')"
        echo "published_comments $(gh api "repos/$repo/pulls/$number/comments" -q length)"
        # shellcheck disable=SC2016 # the $ are GraphQL variables, not shell ones
        echo "resolved_threads $(gh api graphql -F p="$number" -f o="${repo%/*}" -f n="${repo#*/}" \
            -f query='query($o:String!,$n:String!,$p:Int!){repository(owner:$o,name:$n){
                pullRequest(number:$p){reviewThreads(first:100){nodes{isResolved}}}}}' \
            -q '[.data.repository.pullRequest.reviewThreads.nodes[] | select(.isResolved)] | length')"
        for key in comments held replies; do
            if [ -f "$collected" ]; then
                echo "collected_$key $(jq --arg k "$key" '.[$k] // [] | length' "$collected")"
            else
                echo "collected_$key -"
            fi
        done
    } > "$1"
}

# Agent and Skill are what non-negotiable 7 needs: the findings come from
# superpowers:requesting-code-review, which dispatches a reviewer subagent. Without them
# a run reviews the diff itself and the rule goes unmeasured. Edit and jq are what the
# collected review and the delivery of follow-up replies use; jq is named rather than left
# to whatever a machine's own settings happen to allow.
turn=1
for prompt in "$@"; do
    reply="$out"
    [ "$turns" -eq 1 ] || reply="${out%.txt}.turn$turn.txt"
    continue_flag=""
    [ "$turn" -eq 1 ] || continue_flag="--continue"
    attempt=1
    while :; do
        echo "turn $turn, attempt $attempt at $(date -u +%H:%M:%S)" >> "$log"
        # shellcheck disable=SC2086
        if (cd "$clone" && claude -p $continue_flag "$prompt" --model "$model" \
            --allowedTools Read Write Edit Grep Glob Agent Skill \
            "Bash(gh:*)" "Bash(find:*)" "Bash(git:*)" "Bash(jq:*)" \
            > "$reply" 2>&1); then
            break
        fi
        # Anything other than an overloaded API stops the run. Retrying every failure
        # would have buried the run that died with "Not logged in" under ten identical
        # attempts.
        if ! grep -q 529 "$reply" || [ "$attempt" -ge 10 ]; then
            echo "turn $turn failed, stopping" >> "$log"
            cat "$reply" >> "$log"
            cat "$reply" >&2
            exit 1
        fi
        echo "turn $turn got 529, waiting" >> "$log"
        attempt=$((attempt + 1))
        sleep 60
    done
    [ "$turns" -eq 1 ] || snapshot "${out%.txt}.turn$turn.state"
    echo "turn $turn done" >> "$log"
    turn=$((turn + 1))
done

if [ "$turns" -eq 1 ]; then
    printf 'output: %s\n' "$out"
else
    printf 'turns:  %s.turn<N>.txt and .state, N from 1 to %s\n' "${out%.txt}" "$turns"
fi
printf 'log:    %s\nclone:  %s\n' "$log" "$clone"
