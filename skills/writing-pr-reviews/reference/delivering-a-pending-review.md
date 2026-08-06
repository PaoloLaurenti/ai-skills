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

## Changing a draft this run created

Only one this run created. Delete it and recreate it, since a second `POST` fails while it
exists:

```bash
gh api --method DELETE repos/OWNER/REPO/pulls/N/reviews/REVIEW_ID
```

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
```

## Two things the API reports misleadingly

Reading a pending review back shows `position` and `original_position` only, with `line`,
`start_line` and `side` null whatever the payload sent. That is how the API reports a
draft, not evidence the anchor was lost. Grade an anchor from the payload rather than from
the read-back.

Inline comments only attach to lines inside a diff hunk, and line numbers come from the
head version of the file rather than from the diff.
