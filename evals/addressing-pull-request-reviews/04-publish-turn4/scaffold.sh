#!/bin/sh
# Turn 2 of the recorded conversation switched to the pull request's branch, turn 3
# applied the accepted change to the README, uncommitted, and the two turns kept the
# accepted replies in the run's temporary directory, beside its home, where the history
# names it as `../../tmp`.
set -eu
here="$(dirname "$0")"
"$here/../fixture/workspace.sh" walkthrough
cp "$here/README.md" README.md
mkdir -p ../../tmp
cp "$here"/tmp/* ../../tmp/
