#!/bin/sh
# The settings noted, and FEATURE.md with three open decisions that do not depend on
# each other, so `grilling` alone would ask them in one round. The design line says
# there is none, or a run first asks for the design file's link, a missing source
# rather than an open decision.
set -eu
"$(dirname "$0")/../fixture/setup.sh" cached
awk '/^- How long a link stays valid/ {
    print
    print "- Whether a person who signed up with Google or Apple, and has no password, can"
    print "  ask for a reset link has not been decided yet."
    print "- Whether the reset email is written in the language of the app or always in"
    print "  English has not been decided yet."
    next
}
/^Design:/ { print "Design: none, the pages reuse the app'"'"'s existing components."; next }
{ print }' FEATURE.md >FEATURE.md.new
mv FEATURE.md.new FEATURE.md
