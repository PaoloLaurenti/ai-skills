#!/bin/sh
# Runs one scenario with the plugin disabled and the instructions naming it cut, which is
# what makes a baseline a baseline.
#
#   baseline.sh <state> <model> <prompt> [<next prompt>...]
#
# Takes the same arguments as run.sh and hands them straight over, so a baseline and the
# run it is compared against differ in one thing only.
#
# Not with an empty CLAUDE_CONFIG_DIR. That removes the plugin and the credentials
# together, since credentials live in the keychain keyed to the real configuration
# directory: `claude plugin list` answers without them and passes the check, then the run
# dies with "Not logged in".
#
# Assumes the plugin is enabled to begin with, which is the normal state, since the
# symlink loads it in every session on this machine.
set -eu

[ "$#" -ge 3 ] || {
    echo "usage: baseline.sh <state> <model> <prompt> [<next prompt>...]" >&2
    exit 2
}

here="$(cd "$(dirname "$0")" && pwd)"
plugin=myai@skills-dir

# Disabling the plugin is not enough when the user-level instructions name the skill: a
# run told to "always use myai:developing-with-tdd" gets "Unknown skill", then finds
# SKILL.md through the symlink and reads it by hand. So for the length of the run every
# line naming the skill leaves the instructions: the "Test-driven development" section of
# CLAUDE.md, and any line naming it in code-principles.md. Both are copied first and put
# back from the trap.
instructions="$HOME/.claude/CLAUDE.md"
principles="$HOME/.claude/code-principles.md"
saved="$(mktemp -d)"
cp "$instructions" "$saved/CLAUDE.md"
cp "$principles" "$saved/code-principles.md"

restore() {
    cp "$saved/CLAUDE.md" "$instructions"
    cp "$saved/code-principles.md" "$principles"
    claude plugin enable "$plugin" >/dev/null 2>&1 || true
    if cmp -s "$saved/CLAUDE.md" "$instructions" && cmp -s "$saved/code-principles.md" "$principles"; then
        rm -rf "$saved"
    else
        echo "baseline: the instructions were not restored; the originals are in $saved" >&2
    fi
}

# From a trap rather than a line after the run: a baseline that is killed must not leave
# the skill switched off or the instructions cut.
trap restore EXIT
trap 'exit 130' INT TERM
claude plugin disable "$plugin" >/dev/null 2>&1
awk '/^### Test-driven development/ {skip = 1; next} skip && /^#/ {skip = 0} !skip' \
    "$saved/CLAUDE.md" > "$instructions"
grep -v 'myai:developing-with-tdd' "$saved/code-principles.md" > "$principles" || true

# The verification, printed rather than assumed. A baseline where the skill loaded after
# all is worse than no baseline, because it reads like the rules holding on their own.
# Read from the Status line of this plugin's own entry, found by name, since how many
# lines separate it from the section header is the listing's layout to change.
loaded=$(claude plugin list | awk -v p="$plugin" '$0 ~ p {f=1} f && /Status:/ {print; exit}' \
    | grep -c 'loaded' || true)
echo "$plugin reporting loaded: $loaded (expected 0)"
[ "$loaded" -eq 0 ] || {
    echo "baseline: the plugin is still loaded, so this would not be a baseline" >&2
    exit 1
}

named=$(cat "$instructions" "$principles" | grep -c 'myai:developing-with-tdd' || true)
echo "instructions naming myai:developing-with-tdd: $named (expected 0)"
[ "$named" -eq 0 ] || {
    echo "baseline: the instructions still name the skill, so this would not be a baseline" >&2
    exit 1
}

"$here/run.sh" "$@"
