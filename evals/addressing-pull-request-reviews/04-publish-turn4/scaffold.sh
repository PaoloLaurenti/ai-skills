#!/bin/sh
# Turn 2 of the recorded conversation switched to the pull request's branch, and turn 3
# applied the accepted change to the README, uncommitted.
set -eu
here="$(dirname "$0")"
"$here/../fixture/workspace.sh" walkthrough
cp "$here/README.md" README.md
