#!/bin/sh
# Runs one scenario against the skill as installed, one turn per prompt, retrying a turn
# while the API is overloaded.
#
#   run.sh <state> <model> <prompt> [<next prompt>...]
#
# Builds a fresh fixture in the given state (see fixture/setup.sh), then drives the
# conversation from inside it: the first turn starts the session, each later one
# continues it with `--continue`. After every turn it writes the reply to
# `<out>.turn<N>.txt` and the state of the crate to `<out>.turn<N>.state`, since the state
# after the last turn says nothing about what an earlier turn did.
#
# A state file holds:
#   src_changed    files under src/ changed or added since the fixture commit
#   tests_changed  the same under tests/
#   other_changed  anything else changed or added, `target/` being ignored
#   build          ok or failed
#   passed failed ignored   summed over every test binary `cargo test` ran
#   tests          the names of the test functions under tests/, one line
#
# EVAL_RUN_DIR overrides where the fixture and the output go.
set -eu

[ "$#" -ge 3 ] || {
    echo "usage: run.sh <state> <model> <prompt> [<next prompt>...]" >&2
    exit 2
}
state="$1"
model="$2"
shift 2

here="$(cd "$(dirname "$0")" && pwd)"
base="${EVAL_RUN_DIR:-$(mktemp -d)}"
crate="$base/shop"
out="$base/run"
log="$out.log"

"$here/fixture/setup.sh" "$state" "$crate"
: > "$log"

snapshot() {
    {
        echo "src_changed $(git -C "$crate" status --porcelain -- src | wc -l | tr -d ' ')"
        echo "tests_changed $(git -C "$crate" status --porcelain -- tests | wc -l | tr -d ' ')"
        echo "other_changed $(git -C "$crate" status --porcelain -- . ':!src' ':!tests' | wc -l | tr -d ' ')"
        results="$(cd "$crate" && cargo test 2>&1 || true)"
        if printf '%s\n' "$results" | grep -q '^error'; then
            echo "build failed"
        else
            echo "build ok"
        fi
        printf '%s\n' "$results" | awk '/^test result:/ {
                for (i = 1; i <= NF; i++) {
                    if ($(i) ~ /^passed/) p += $(i - 1)
                    if ($(i) ~ /^failed/) f += $(i - 1)
                    if ($(i) ~ /^ignored/) g += $(i - 1)
                }
            }
            END { printf "passed %d\nfailed %d\nignored %d\n", p, f, g }'
        echo "tests $(awk '/#\[test\]/ { t = 1; next }
            t && /fn / { sub(/.*fn /, ""); sub(/\(.*/, ""); printf "%s ", $0; t = 0 }' \
            "$crate"/tests/*.rs)"
    } > "$out.turn$1.state"
}

# The tools a TDD session needs, and no more: reading and editing files, cargo to build
# and test, git to read what changed. `git` is allowed in full on purpose: whether a run
# commits on its own is one of the things a scenario grades.
turn=1
for prompt in "$@"; do
    continue_flag=""
    [ "$turn" -eq 1 ] || continue_flag="--continue"
    attempt=1
    while :; do
        echo "turn $turn, attempt $attempt at $(date -u +%H:%M:%S)" >> "$log"
        # shellcheck disable=SC2086
        if (cd "$crate" && claude -p $continue_flag "$prompt" --model "$model" \
            --allowedTools Read Write Edit Grep Glob Agent Skill \
            "Bash(cargo:*)" "Bash(git:*)" "Bash(ls:*)" "Bash(mkdir:*)" "Bash(rm:*)" \
            > "$out.turn$turn.txt" 2>&1); then
            break
        fi
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

printf 'turns:  %s.turn<N>.txt and .state, N from 1 to %s\nlog:    %s\ncrate:  %s\n' \
    "$out" "$((turn - 1))" "$log" "$crate"
