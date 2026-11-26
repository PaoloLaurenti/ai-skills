#!/bin/sh
# Builds a case's workspace: the feature to split into cards and the team's Definition
# of Ready, in the run's empty workspace, the current directory.
#
#   setup.sh [cached | approved]
#
# With `cached`, the team's settings an earlier session noted are put in the run's home,
# where the skill keeps them: `~/.local/state/writing-cards/linear-mobile.md`. With
# `approved`, the settings too, and `cards.md`, the cards approved one at a time and the
# decisions behind them, ready to be written to the tracker.
set -eu

here="$(cd "$(dirname "$0")" && pwd)"
cp "$here/FEATURE.md" "$here/dor.md" .
case "${1:-}" in
cached | approved)
    mkdir -p "$HOME/.local/state/writing-cards"
    cp "$here/settings.md" "$HOME/.local/state/writing-cards/linear-mobile.md"
    ;;
esac
if [ "${1:-}" = approved ]; then
    cp "$here/cards.md" .
fi
