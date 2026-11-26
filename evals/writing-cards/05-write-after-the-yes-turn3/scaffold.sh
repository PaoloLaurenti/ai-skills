#!/bin/sh
# The workspace as the second turn left it: cards.md with the cycle the requester chose.
set -eu
"$(dirname "$0")/../fixture/setup.sh" approved
cp "$(dirname "$0")/cards.md" .
