#!/bin/sh
# `04`'s workspace, with a link to the design frame in FEATURE.md for the card to keep.
set -eu
"$(dirname "$0")/../fixture/setup.sh" cached
awk '/^Design:/ { print "Design: [the \"Password reset\" page](https://www.figma.com/design/Rs7Mobile/App?node-id=42-7) of the app'"'"'s design file."; next } { print }' FEATURE.md >FEATURE.md.new
mv FEATURE.md.new FEATURE.md
