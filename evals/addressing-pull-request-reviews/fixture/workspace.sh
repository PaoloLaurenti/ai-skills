#!/bin/sh
# Builds a case's workspace in the current directory, which is where a case's scaffold
# starts: the run's empty workspace.
#
# - The fixture's repo from `files.sh`, checked out on `main` as a fresh clone would be,
#   or on the branch named by the one argument, where a recorded turn left it.
# - `origin.git`, a bare repo beside it that stands for GitHub: the sandbox has no
#   network, so a push lands there and a grader reads the branch from its files. Git is
#   told to ignore it, so it never shows as untracked.
# - The pull request as `tools/fake-gh/record.sh` recorded it, in `.git/fake-gh/`, for
#   the stand-in `gh` to answer from. Its head is set to this repo's own, since the
#   recorded one names a commit made on GitHub at another time. The log of calls starts
#   empty, so a grader reading it finds a file even when a turn calls nothing.
set -eu

here="$(cd "$(dirname "$0")" && pwd)"
branch=walkthrough

git init -q -b main
git config user.name "eval fixture"
git config user.email "eval@example.invalid"
# Fixed dates give the commits the same ids on every run, so a recorded turn naming one
# still names it when a later case resumes.
GIT_AUTHOR_DATE="2026-09-23T10:00:00Z"
GIT_COMMITTER_DATE="$GIT_AUTHOR_DATE"
export GIT_AUTHOR_DATE GIT_COMMITTER_DATE
"$here/files.sh" "$branch"
git checkout -q main

git init -q --bare origin.git
# A bare repo keeps no log of its branches unless told to, and a grader counts the pushes
# in it.
git -C origin.git config core.logAllRefUpdates true
echo "origin.git/" >>.git/info/exclude
git remote add origin "$(pwd)/origin.git"
git push -q origin main "$branch"
git fetch -q origin
git branch -q --set-upstream-to origin/main main
git branch -q --set-upstream-to "origin/$branch" "$branch"
git checkout -q "${1:-main}"

mkdir -p .git/fake-gh
jq --arg head "$(git rev-parse "$branch")" '.head_sha = $head' \
    "$here/pull-request.json" >.git/fake-gh/pull-request.json
: >.git/fake-gh/calls.jsonl
