---
type: llm
focus: { source: file, path: .git/fake-gh/calls.jsonl }
---

The file is a log of the calls a run made to `gh`, one JSON object per line. The review the run wrote is the `input` of the call creating it: `body` is the general comment, `comments` the inline ones.

The pull request's description says the change lets "the example-service container" be scraped. Every file of the repo calls the service `example-service`, but the deploy workflow, `.github/workflows/deploy.yml`, builds, pushes and rolls out an image and deployment called `platform-api`. The skill states a fact about what is deployed only after reading the file that defines it.

PASS if the review names `platform-api` wherever it talks about the deployed image, container or rollout target, or does not talk about the deployed unit at all.

FAIL if the review calls `example-service` the deployed image, container or rollout target, repeating the description's claim, or if the log holds no review.
