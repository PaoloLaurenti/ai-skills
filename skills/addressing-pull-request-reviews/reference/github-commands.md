# GitHub commands for a review walkthrough

The commands behind the steps in `SKILL.md`, in the order the walkthrough reaches them.
Set `R` to `owner/repo` and `N` to the pull request number.

## Collecting the comments

The review threads, with their replies and whether they are resolved. REST returns the
same comments flat, and the thread structure has to be rebuilt from `in_reply_to_id`:
GraphQL gives it directly.

```bash
gh api graphql -f query='query($owner:String!,$name:String!,$number:Int!){
  repository(owner:$owner,name:$name){ pullRequest(number:$number){
    author{login}
    reviewThreads(first:100){ nodes{ isResolved path line
      comments(first:50){ nodes{ databaseId author{login} body url } } } }
    reviews(first:100){ nodes{ author{login} state body } }
  } } }' -f owner="${R%/*}" -f name="${R#*/}" -F number="$N"
```

A thread is walked when no comment after its first one is by the pull request's author.
`databaseId` of a thread's first comment is the id the reply goes to.

The general conversation of the pull request, which holds comments that are not attached
to a line:

```bash
gh api "repos/$R/issues/$N/comments" --paginate -q '.[] | {id, user: .user.login, body}'
```

When the pull request has nothing to walk, the requester's open pull requests with review
comments:

```bash
for n in $(gh pr list --repo "$R" --author @me --state open --json number -q '.[].number'); do
  echo "#$n $(gh api "repos/$R/pulls/$n/comments" -q length) inline comments"
done
```

## Finding the checkout

```bash
git worktree list
git fetch origin
git status -sb                                   # ahead and behind the remote
git range-diff @{u}...HEAD                       # whether a diverged branch holds the same patches
```

When the range-diff shows every local commit as equal to a remote one, the local branch
only needs to move to the remote. That move discards local commits, so it is the
requester's to approve, and `git reset --keep @{u}` refuses to run over uncommitted work.

## Publishing

A reply inside its thread. The id is the thread's first comment, and this endpoint is the
one that keeps the reply attached to the thread:

```bash
gh api --method POST "repos/$R/pulls/$N/comments/$COMMENT_ID/replies" -f body="$REPLY"
```

`gh pr comment` and `repos/$R/issues/$N/comments` post to the general conversation
instead, which detaches the answer from the line it is about.

The description, from a file, so that the text reaches GitHub exactly as written:

```bash
gh pr edit "$N" --repo "$R" --body-file description.md
gh pr view "$N" --repo "$R" --json body -q .body | diff - description.md
```

A new review request. After a review with requested changes, GitHub does not ask the
reviewer again by itself:

```bash
gh api --method POST "repos/$R/pulls/$N/requested_reviewers" -f 'reviewers[]=LOGIN'
```

Requesting the pull request's author returns `422`, since GitHub does not let the author
review their own pull request.

The final check:

```bash
gh api "repos/$R/pulls/$N/comments" -q '[.[] | select(.in_reply_to_id != null)] | length'
gh api "repos/$R/pulls/$N/reviews" -q '[.[] | select(.state == "PENDING")] | length'
```
