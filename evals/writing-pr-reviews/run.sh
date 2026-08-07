#!/bin/sh
# Runs one scenario against the skill as installed, retrying while the API is
# overloaded.
#
#   run.sh <model> <prompt>
#
# Prints where it left the output, the log and the clone. The clone matters after the
# run and is deliberately not cleaned up: the JSON the skill posted from sits in it, and
# a pending review's anchor can only be graded from there, since the API reports
# `position` alone for a draft.
#
# EVAL_FIXTURE_REPO, EVAL_FIXTURE_CLONE and EVAL_RUN_OUT override the defaults.
set -eu

[ "$#" -eq 2 ] || {
    echo "usage: run.sh <model> <prompt>" >&2
    exit 2
}
model="$1"
prompt="$2"

here="$(cd "$(dirname "$0")" && pwd)"
repo="${EVAL_FIXTURE_REPO:-$(gh api user -q .login)/ai-skills-eval-fixture}"

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
# EVAL_SKIP_RESET exists for scenario 07 alone, which needs a draft already open when the
# run starts and seeds one itself. Skipping the clearing anywhere else reintroduces the
# failure the clearing is here to prevent.
if [ -z "${EVAL_SKIP_RESET:-}" ]; then
    "$here/fixture/reset.sh" >> "$log" 2>&1
else
    echo "reset skipped: EVAL_SKIP_RESET set" >> "$log"
fi

attempt=1
while [ "$attempt" -le 10 ]; do
    echo "attempt $attempt at $(date -u +%H:%M:%S)" >> "$log"
    # Agent and Skill are what non-negotiable 7 needs: the findings come from
    # superpowers:requesting-code-review, which dispatches a reviewer subagent.
    # Without them a run reviews the diff itself and the rule goes unmeasured.
    if (cd "$clone" && claude -p "$prompt" --model "$model" \
        --allowedTools Read Write Grep Glob Agent Skill \
        "Bash(gh:*)" "Bash(find:*)" "Bash(git:*)" \
        > "$out" 2>&1); then
        echo "attempt $attempt succeeded" >> "$log"
        printf 'output: %s\nlog:    %s\nclone:  %s\n' "$out" "$log" "$clone"
        exit 0
    fi
    # Anything other than an overloaded API stops the loop. Retrying every failure would
    # have buried the run that died with "Not logged in" under ten identical attempts.
    if ! grep -q 529 "$out"; then
        echo "attempt $attempt failed for another reason, stopping" >> "$log"
        cat "$out" >> "$log"
        cat "$out" >&2
        exit 1
    fi
    echo "attempt $attempt got 529, waiting" >> "$log"
    attempt=$((attempt + 1))
    sleep 60
done

echo "gave up after 10 attempts" >> "$log"
exit 1
