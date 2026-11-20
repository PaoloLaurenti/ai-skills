#!/bin/sh
# Builds a case's workspace in the current directory, which is where a case's scaffold
# starts: the run's empty workspace.
#
#   workspace.sh <branch> [<thread to resolve>...]
#
# - The fixture's repo from `files.sh`, checked out on the pull request's branch, as a
#   checkout of the branch under review would be. The follow-up's branch carries the
#   author's fix too, from `follow-up-fix.sh`, as the seeded pull request does.
# - `origin.git`, a bare repo beside it that stands for GitHub: the sandbox has no
#   network, so a fetch reaches it, and a push would land there. Git is told to ignore
#   it, so it never shows as untracked.
# - The pull request as `photograph.sh` took it, in `.git/fake-gh/`, for the stand-in
#   `gh` to answer from. The commits it names were made on GitHub at another time, so
#   each is replaced by this repo's own: the head, the base, and the commit a submitted
#   review was written against, which is the head's parent. The log of calls starts
#   empty, so a grader reading it finds a file even when a run calls nothing.
# - On `open-draft`, the draft someone left open, `open-draft.json`, created through the
#   stand-in as a run would create one, before the log is emptied.
# - The threads named after the branch resolved through the stand-in, as a recorded turn
#   of the follow-up resolved them, before the log is emptied.
set -eu

here="$(cd "$(dirname "$0")" && pwd)"
branch="$1"
shift
photograph="$here/pull-requests/$branch.json"

git init -q -b main
git config user.name "eval fixture"
git config user.email "eval@example.invalid"
# Fixed dates give the commits the same ids on every run, so a grader or a recorded turn
# naming the head still names it.
GIT_AUTHOR_DATE="2026-11-20T10:00:00Z"
GIT_COMMITTER_DATE="$GIT_AUTHOR_DATE"
export GIT_AUTHOR_DATE GIT_COMMITTER_DATE
"$here/files.sh" "$branch"
[ "$branch" != follow-up ] || "$here/follow-up-fix.sh"

git init -q --bare origin.git
echo "origin.git/" >>.git/info/exclude
git remote add origin "$(pwd)/origin.git"
git push -q origin main "$branch"
git fetch -q origin
git branch -q --set-upstream-to origin/main main
git branch -q --set-upstream-to "origin/$branch" "$branch"

mkdir -p .git/fake-gh
jq -r '[.head_sha, .viewed.baseRefOid] + [.reviews[].commit_id] | unique | .[]' "$photograph" |
    while read -r photographed; do
        if [ "$photographed" = "$(jq -r .head_sha "$photograph")" ]; then
            echo "s/$photographed/$(git rev-parse "$branch")/g"
        elif [ "$photographed" = "$(jq -r .viewed.baseRefOid "$photograph")" ]; then
            echo "s/$photographed/$(git rev-parse main)/g"
        else
            echo "s/$photographed/$(git rev-parse "$branch~1")/g"
        fi
    done >.git/fake-gh/commits.sed
sed -f .git/fake-gh/commits.sed "$photograph" >.git/fake-gh/pull-request.json
rm .git/fake-gh/commits.sed

# Only the stand-in may answer below: a real `gh` would write to GitHub.
stand_in() {
    case "$(command -v gh)" in
    */evals/results/.bin/gh) gh "$@" >/dev/null ;;
    *)
        echo "workspace: gh is not the stand-in, so nothing was written through it" >&2
        exit 1
        ;;
    esac
}
pull_request="repos/$(jq -r .repository .git/fake-gh/pull-request.json)/pulls/$(jq -r .number .git/fake-gh/pull-request.json)"
if [ "$branch" = open-draft ]; then
    jq --arg head "$(git rev-parse "$branch")" '{commit_id: $head} + .' "$here/open-draft.json" |
        stand_in api --method POST "$pull_request/reviews" --input -
fi
for thread in "$@"; do
    stand_in api graphql -f query='mutation($t:ID!){ resolveReviewThread(input:{threadId:$t}){ thread{ isResolved } } }' -f t="$thread"
done
: >.git/fake-gh/calls.jsonl
