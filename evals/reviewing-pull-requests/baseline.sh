#!/bin/sh
# Runs one scenario with the plugin disabled, which is what makes a baseline a baseline.
#
#   baseline.sh <model> <prompt> [<next prompt>...]
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

[ "$#" -ge 2 ] || {
    echo "usage: baseline.sh <model> <prompt> [<next prompt>...]" >&2
    exit 2
}

here="$(cd "$(dirname "$0")" && pwd)"
plugin=myai@skills-dir

# From a trap rather than a line after the run: a baseline that is killed must not leave
# the skill switched off.
trap 'claude plugin enable "$plugin" >/dev/null 2>&1' EXIT INT TERM
claude plugin disable "$plugin" >/dev/null 2>&1

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

"$here/run.sh" "$@"
