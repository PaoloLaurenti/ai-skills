#!/bin/sh
# Rewrites, in place, every trace of the account that recorded them out of the given
# files: nothing in this repo names its owner, and `tools/check.sh` fails on any login
# that slips through.
#
#   scrub.sh <repo owner> <file>...
#
# The login and the repo's owner become `fixture-owner`, the display name `Fixture
# Owner`. The account's number and its GraphQL id lead back to it as surely as its
# login, so they become those of account 1, `MDQ6VXNlcjE=` being `04:User1` in base64.
set -eu

owner="$1"
shift
login="$(gh api user -q .login)"
account="$(gh api user -q .id)"
account_node="$(gh api user -q .node_id)"
# Which `gh pr view --json author` carries.
account_name="$(gh api user -q .name)"

for file in "$@"; do
    sed -E \
        -e "s/$owner/fixture-owner/g" \
        -e "s/$login/fixture-owner/g" \
        -e "s|$account_node|MDQ6VXNlcjE=|g" \
        -e "s/$account_name/Fixture Owner/g" \
        -e "s/(^|[^0-9])$account([^0-9]|$)/\\11\\2/g" \
        "$file" >"$file.scrubbed"
    mv "$file.scrubbed" "$file"
done
