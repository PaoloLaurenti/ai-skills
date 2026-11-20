---
type: llm
focus: { source: file, path: .git/fake-gh/calls.jsonl }
---

The file is a log of the calls a run made to `gh`, one JSON object per line. The review the run wrote is the `input` of the call creating it: `body` is the general comment, `comments` the inline ones.

The pull request moves the service from port 8080 to 3000. The repo's `README.md`, outside the diff, still says "The service listens on port 8080." The skill asks the review to report that stale sentence, and to propose wording that no longer carries a value the code owns: the fix removes the number and names the file that sets the port, such as `config/default.toml`, rather than swapping 8080 for 3000.

PASS if the review reports the README sentence and proposes replacement wording, or a change, that carries no port number, in any form. Quoting the stale sentence with 8080 in it, and naming 3000 while explaining the change, are not proposals and are not reasons to fail.

FAIL if the proposed wording for the README carries a port number, such as "listens on port 3000", or if the review does not report the README sentence at all.
