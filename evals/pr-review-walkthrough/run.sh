#!/bin/sh
# Runs one conversation against the skill as installed, one turn per prompt, retrying a
# turn while the API is overloaded.
#
#   run.sh <model> <prompt> [<next prompt>...]
#
# The walkthrough is a conversation, and what each scenario grades is where it stops, so
# a run is a list of turns rather than one prompt. The first turn starts the session and
# every later one continues it from the same clone.
#
# After every turn the state of the fixture goes into `<out>.turn<N>.state`: the local
# branch and head, how many tracked files are modified, the remote head, how many replies
# and top-level comments the pull request carries, and a hash of its description. That
# snapshot is what the mechanical assertions read, because the state after the last turn
# says nothing about what an earlier turn did.
#
# EVAL_FIXTURE_REPO, EVAL_FIXTURE_CLONE and EVAL_RUN_OUT override the defaults.
set -eu

[ "$#" -ge 2 ] || {
    echo "usage: run.sh <model> <prompt> [<next prompt>...]" >&2
    exit 2
}
model="$1"
shift

here="$(cd "$(dirname "$0")" && pwd)"
repo="${EVAL_FIXTURE_REPO:-$(gh api user -q .login)/ai-skills-eval-walkthrough}"
branch=walkthrough

# From inside a clone, the way a real walkthrough happens, so that `gh` resolves the
# fixture from the remote and the run reads the repo's own rules from the working tree.
clone="${EVAL_FIXTURE_CLONE:-}"
if [ -z "$clone" ]; then
    clone="$(mktemp -d)/fixture"
    git clone -q "git@github.com:$repo.git" "$clone"
fi

# The clone carries an identity of its own, the one the fixture's commits use. A machine's
# git config may scope its identity to directories a temporary clone does not sit under,
# and then the run's commit fails for a reason that has nothing to do with the skill.
login="$(gh api user -q .login)"
git -C "$clone" config user.name "$login"
git -C "$clone" config user.email "$login@users.noreply.github.com"

out="${EVAL_RUN_OUT:-$(dirname "$clone")/run}"
log="$out.log"
: > "$log"

# Before every run, not after a failure: a run that replied or pushed leaves the fixture
# in a state the next run would read as its own doing.
EVAL_FIXTURE_REPO="$repo" "$here/fixture/reset.sh" >> "$log" 2>&1

number=$(gh pr list --repo "$repo" --head "$branch" --state open --json number -q '.[0].number')

snapshot() {
    {
        echo "local_branch $(git -C "$clone" rev-parse --abbrev-ref HEAD)"
        echo "local_head $(git -C "$clone" rev-parse HEAD)"
        echo "local_dirty $(git -C "$clone" status --porcelain --untracked-files=no | wc -l | tr -d ' ')"
        echo "remote_head $(gh api "repos/$repo/git/ref/heads/$branch" -q .object.sha)"
        echo "replies $(gh api "repos/$repo/pulls/$number/comments" -q '[.[] | select(.in_reply_to_id != null)] | length')"
        echo "top_level $(gh api "repos/$repo/pulls/$number/comments" -q '[.[] | select(.in_reply_to_id == null)] | length')"
        echo "issue_comments $(gh api "repos/$repo/issues/$number/comments" -q length)"
        echo "body_sha $(gh api "repos/$repo/pulls/$number" -q .body | shasum | cut -c1-12)"
    } > "$out.turn$1.state"
}

# Read, Write and Edit are what a walkthrough needs to apply an accepted change and keep
# its replies; Agent and Skill are what the final review needs, since it dispatches a
# reviewer. `git` and `gh` are allowed in full on purpose: whether a run commits, pushes
# or posts before it is told to is the thing being measured, so the tool list must not
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
            --allowedTools Read Write Edit Grep Glob Agent Skill \
            "Bash(gh:*)" "Bash(git:*)" "Bash(find:*)" "Bash(python3:*)" \
            > "$out.turn$turn.txt" 2>&1); then
            break
        fi
        # Anything other than an overloaded API stops the run, so a real failure is not
        # buried under retries.
        if ! grep -q 529 "$out.turn$turn.txt" || [ "$attempt" -ge 10 ]; then
            echo "turn $turn failed, stopping" >> "$log"
            cat "$out.turn$turn.txt" >&2
            exit 1
        fi
        attempt=$((attempt + 1))
        sleep 60
    done
    snapshot "$turn"
    echo "turn $turn done" >> "$log"
    turn=$((turn + 1))
done

printf 'turns:  %s.turn<N>.txt and .state, N from 1 to %s\nlog:    %s\nclone:  %s\n' \
    "$out" "$((turn - 1))" "$log" "$clone"
