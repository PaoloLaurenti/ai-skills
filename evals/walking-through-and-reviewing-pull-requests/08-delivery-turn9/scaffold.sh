#!/bin/sh
# The workspace, and the collected review as the recorded turn 8 left it in the run's
# home, where the skill keeps it until the review is delivered.
here="$(dirname "$0")"
"$here/../fixture/workspace.sh"
mkdir -p "$HOME/.local/state/reviews"
cp "$here"/collected/*.json "$HOME/.local/state/reviews/"
