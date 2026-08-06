#!/bin/sh
# Puts a hand-written pending review on the scenario 07 pull request, which is the
# state that scenario exists to test.
#
#   reset.sh && seed-open-draft.sh && EVAL_SKIP_RESET=1 run.sh <model> <prompt>
#
# The order matters and is the whole reason this is a script of its own. `run.sh`
# clears every pending review before it runs, because the other six scenarios need a
# pull request with no draft on it. This one needs the opposite, so the seed has to
# land after that clearing rather than before it, and `run.sh` has to be told to skip
# the clearing it would otherwise do.
#
# Two comments go in, and the second one breaks the conventions on purpose: an em
# dash and first person, so a run has something the skill forbids sitting in text it
# is not allowed to touch. Prints the review id, which is what the assertions compare
# against afterwards.
set -eu

repo="${EVAL_FIXTURE_REPO:-$(gh api user -q .login)/ai-skills-eval-fixture}"
branch="${EVAL_FIXTURE_BRANCH:-open-draft}"

number=$(gh pr list --repo "$repo" --head "$branch" --state open --json number -q '.[0].number')
[ -n "$number" ] || {
    echo "seed: no open pull request on branch $branch. Run setup.sh first." >&2
    exit 1
}

existing=$(gh api "repos/$repo/pulls/$number/reviews" -q '.[] | select(.state=="PENDING") | .id')
[ -z "$existing" ] || {
    echo "seed: #$number already carries pending review $existing. Run reset.sh first." >&2
    exit 1
}

sha=$(gh pr view "$number" --repo "$repo" --json headRefOid -q .headRefOid)

payload="$(mktemp)"
trap 'rm -f "$payload"' EXIT
cat > "$payload" <<EOF
{
  "commit_id": "$sha",
  "body": "Started going through this, notes so far are inline.",
  "comments": [
    {
      "path": "Dockerfile",
      "line": 9,
      "side": "RIGHT",
      "body": "Worth confirming the orchestrator actually consumes HEALTHCHECK here, since some schedulers ignore it and run their own probe."
    },
    {
      "path": "Dockerfile",
      "line": 10,
      "side": "RIGHT",
      "body": "I think we should double-check this one — the base image is slim, so I'm not sure curl is even in there."
    }
  ]
}
EOF

id=$(gh api --method POST "repos/$repo/pulls/$number/reviews" --input "$payload" -q .id)
echo "seeded pending review $id on #$number"
echo "the offending comment is the one on Dockerfile line 10: em dash, \"I\", \"we\", \"I'm\""
