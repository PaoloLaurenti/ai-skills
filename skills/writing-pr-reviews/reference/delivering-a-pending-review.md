# Delivering a pending review

The commands behind the delivery rules in `SKILL.md`. Read this before posting anything:
the rules say what must happen, this says how, and two of the calls here are ones that
come out wrong when reconstructed from memory.

## Creating a draft, when none is open

`gh pr review` cannot produce a draft: it requires `--comment`, `--approve`, or
`--request-changes`, all of which publish immediately. Use the API and omit `event`:

```bash
gh api --method POST repos/OWNER/REPO/pulls/N/reviews --input review.json
```

`review.json` holds `commit_id`, `body`, and a `comments` array of `{path, line, side}`,
or `{path, start_line, start_side, line, side}` for a range. No `event` key means the
whole review, general comment included, stays `PENDING`.

## Checking for an open draft first

GitHub allows one pending review per user per pull request, so the `POST` above fails with
`422 User can only have one pending review per pull request` while one exists:

```bash
gh api repos/OWNER/REPO/pulls/N/reviews -q '.[] | select(.state=="PENDING") | .id'
```

Nothing back means the `POST` is the right call. An id means that draft is appended to,
per non-negotiable 2, and never deleted to get past the `422`.

## Appending to a draft that is already open

REST has no append, so this part is GraphQL and works off the node id:

```bash
node=$(gh api repos/OWNER/REPO/pulls/N/reviews/REVIEW_ID -q .node_id)
```

Inline comments go one at a time, anchored exactly as `review.json` would have anchored
them:

```bash
gh api graphql -f query='
mutation($rid:ID!,$body:String!,$path:String!,$line:Int!){
  addPullRequestReviewThread(input:{pullRequestReviewId:$rid, body:$body,
    path:$path, line:$line, side:RIGHT}){ thread { id } }
}' -f rid="$node" -f body="..." -f path="..." -F line=N
```

It takes `startLine` and `startSide` too, so a range anchor survives the append. Reach for
it rather than `addPullRequestReviewComment`, which reads like the obvious name and
rejects `line` and `side` outright, because it still speaks the older `position` model.

The general comment is added to rather than overwritten, so read the existing body and
send it back with the new text after it:

```bash
old=$(gh api repos/OWNER/REPO/pulls/N/reviews/REVIEW_ID -q .body)
gh api graphql -f query='
mutation($rid:ID!,$body:String!){
  updatePullRequestReview(input:{pullRequestReviewId:$rid, body:$body}){
    pullRequestReview { state } }
}' -f rid="$node" -f body="$old
...new text..."
```

Both mutations leave the review `PENDING` and publish nothing.

**A draft whose body is empty cannot be given one.** `updatePullRequestReview`, and the
REST `PUT` on the review alike, answer `Could not edit a review with a missing body` when
the draft was created with `""`. So a draft gets its general comment in the `POST` that
creates it: a review built up over several turns, as a guided walkthrough builds one, is
collected locally and created once, after the general comment is settled. A draft that was
already open with an empty body cannot take a general comment at all: say so in the reply
and hand over its text.

## Changing a draft this run created

Only one this run created. Delete it and recreate it, since a second `POST` fails while it
exists. Read its comment ids first, because the `DELETE` leaves every inline comment
behind:

```bash
ids=$(gh api repos/OWNER/REPO/pulls/N/reviews/REVIEW_ID/comments -q '.[].id')
gh api --method DELETE repos/OWNER/REPO/pulls/N/reviews/REVIEW_ID
echo "$ids" | while read -r id; do
  gh api --method DELETE "repos/OWNER/REPO/pulls/comments/$id"
done
```

The review leaves `pulls/N/reviews`, but its inline comments stay `PENDING` with no
review, at once and every time, and the next draft does not pick them up. A `404` on one
of them means it is already gone. Only the ids read here are deleted: an orphan from
anywhere else is reported, never deleted. The loop reads with `while read` because in zsh
`for id in $ids` does not split on newlines.

A draft that was already open when the review started is not this. It belongs to whoever
wrote it, and appending is the only thing that may happen to it.

## Reporting back

State the submit command rather than leaving it to be reconstructed. The review id is part
of it, and reconstructing it produces an endpoint that does not exist:

```bash
gh api --method POST repos/OWNER/REPO/pulls/N/reviews/REVIEW_ID/events -f event=COMMENT
```

Confirm nothing leaked first:

```bash
gh api repos/OWNER/REPO/pulls/N/reviews -q '.[] | "\(.id) \(.state)"'  # expect PENDING
gh api repos/OWNER/REPO/pulls/N/comments -q 'length'                   # expect 0
gh api graphql -f query='query($owner:String!,$name:String!,$n:Int!){
  repository(owner:$owner,name:$name){ pullRequest(number:$n){
    reviewThreads(first:100){ nodes{ comments(first:1){ nodes{
      databaseId state pullRequestReview{ databaseId } } } } } } } }' \
  -f owner=OWNER -f name=REPO -F n=N \
  -q '.data.repository.pullRequest.reviewThreads.nodes[].comments.nodes[0]
      | select(.state=="PENDING" and .pullRequestReview==null) | .databaseId'  # expect nothing
```

`pulls/N/comments` returns submitted comments only, so orphans left by a deleted draft
show up in the last query alone. Any id it prints goes in the reply, as an orphan this run
did not create.

## Two things the API reports misleadingly

Reading a pending review back shows `position` and `original_position` only, with `line`,
`start_line` and `side` null whatever the payload sent. That is how the API reports a
draft, not evidence the anchor was lost. Grade an anchor from the payload rather than from
the read-back.

Inline comments only attach to lines inside a diff hunk, and line numbers come from the
head version of the file rather than from the diff.
