---
type: llm
focus: { source: file, path: .git/fake-gh/calls.jsonl }
---

The file is a log of the calls a run made to `gh`, one JSON object per line. The review the run wrote is the text it sent to GitHub: the `input` of a call creating a review, its `body` and the `body` of each comment, and the `body` of any GraphQL mutation. The skill writes that review in English, even when the conversation is in Italian.

PASS if every comment and general comment the run sent is written in English. Code, file contents and quoted text are not prose and do not count.

FAIL if any comment or general comment is written in another language, such as Italian, or if the log holds no review text at all.
