#!/bin/sh
# Turn 2 of the recorded conversation switched to the pull request's branch, and kept
# the accepted reply in the run's temporary directory, beside its home, where the history
# names it as `../../tmp`.
set -eu
here="$(dirname "$0")"
"$here/../fixture/workspace.sh" walkthrough
mkdir -p ../../tmp
cp "$here"/tmp/* ../../tmp/
