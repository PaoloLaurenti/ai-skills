# Following up on a review already submitted

The commands behind the follow-up section of `SKILL.md`. `OWNER`, `REPO` and `N` stand
for the pull request.

## What was reviewed, and what changed since

```bash
me=$(gh api user -q .login)
head=$(gh pr view N --json headRefOid -q .headRefOid)
sha=$(gh api repos/OWNER/REPO/pulls/N/reviews | jq -r --arg me "$me" \
  '[.[] | select(.user.login == $me and .state != "PENDING")] | last | .commit_id')
git fetch -q origin "$sha" "$head"
git log --oneline "${sha}..${head}"
git diff --stat "${sha}" "${head}"
```

`sha` is the head the last review submitted from this account was written against.
Fetching it by SHA works even after a force-push dropped it from the branch. A file as it
was then is `git show "${sha}:path"`, braces included, since in zsh `$sha:a` expands as a
path modifier. A diff limited to one path loses a rename, so a file renamed since goes into
the pathspec under both names.

## The threads

```bash
gh api graphql -f query='query($owner:String!,$name:String!,$n:Int!){
  repository(owner:$owner,name:$name){ pullRequest(number:$n){
    reviewThreads(first:100){ nodes{ id isResolved isOutdated path line originalLine
      comments(first:100){ nodes{ url body state author{ login } } } } } } } }' \
  -f owner=OWNER -f name=REPO -F n=N \
  | jq --arg me "$me" '[.data.repository.pullRequest.reviewThreads.nodes[]
      | select(.isResolved | not)
      | select(.comments.nodes[0].author.login == $me
               and .comments.nodes[0].state == "SUBMITTED")
      | {id, path, line, originalLine, isOutdated, url: .comments.nodes[0].url,
         comment: .comments.nodes[0].body,
         replies: [.comments.nodes[1:][] | select(.state == "SUBMITTED")
                   | {author: .author.login, body}]}]
    | sort_by(.path, .originalLine)'
```

`isOutdated` means the thread's lines changed since the comment, and `line` is then null:
read `originalLine` against `sha`. `id` is what resolving and replying take.

## Resolving a thread

```bash
gh api graphql -f query='mutation($t:ID!){
  resolveReviewThread(input:{threadId:$t}){ thread{ isResolved } } }' -f t=THREAD_ID
gh api graphql -f query='mutation($t:ID!){
  unresolveReviewThread(input:{threadId:$t}){ thread{ isResolved } } }' -f t=THREAD_ID
```

The second undoes the first, when the requester takes a yes back.

## Delivering the replies

The collected file holds the payload `guided-pr-review` builds, plus the replies:

```json
{"commit_id": "HEAD_SHA", "body": "", "comments": [],
 "replies": [{"thread": "THREAD_ID", "body": "..."}]}
```

```bash
f=~/.claude/reviews/OWNER-REPO-N.json
review=$(jq 'del(.replies)' "$f" \
  | gh api --method POST repos/OWNER/REPO/pulls/N/reviews --input - -q .node_id)
jq -c '.replies[]' "$f" | while read -r reply; do
  gh api graphql -f query='mutation($t:ID!,$r:ID!,$b:String!){
    addPullRequestReviewThreadReply(input:{pullRequestReviewThreadId:$t,
      pullRequestReviewId:$r, body:$b}){ comment{ databaseId state } } }' \
    -f t="$(jq -r .thread <<<"$reply")" -f r="$review" -f b="$(jq -r .body <<<"$reply")"
done
```

The REST payload has no `in_reply_to`, so `replies` comes off before the `POST` and each
reply goes in by GraphQL, coming back `PENDING`. With nothing else collected the `POST`
still creates the draft, from an empty body and no comments; a draft created with an empty
body cannot be given one later, so the body is settled first. A draft already open takes
the place of the `POST`, per non-negotiable 2: its id is
`gh api repos/OWNER/REPO/pulls/N/reviews -q '.[] | select(.state=="PENDING") | .node_id'`.
Never `POST pulls/N/comments/ID/replies`: it publishes the reply at once. Then run the
checks and state the submit command from the delivery reference.
