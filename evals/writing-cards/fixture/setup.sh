#!/bin/sh
# Builds a case's workspace: the feature to split into cards and the team's Definition
# of Ready, in the run's empty workspace, the current directory.
#
#   setup.sh [cached]
#
# With `cached`, the team's settings an earlier session noted are put in the run's home,
# where the skill keeps them: `~/.local/state/writing-cards/linear-mobile.md`.
set -eu

here="$(cd "$(dirname "$0")" && pwd)"
cp "$here/FEATURE.md" "$here/dor.md" .
if [ "${1:-}" = cached ]; then
    mkdir -p "$HOME/.local/state/writing-cards"
    cp "$here/settings.md" "$HOME/.local/state/writing-cards/linear-mobile.md"
fi
