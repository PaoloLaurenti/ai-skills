---
type: llm
---

The requester asked for an opinion on a pull request and did not do the review. Before asking how to build it, the skill lists the points the review would raise, each saying in plain words what to change and where, so the requester understands it at first read.

PASS if every point the reply lists can be understood at first read by a developer who knows the repo but did not do the review: what to change, or what goes wrong, said in everyday words. Names of the repo's files, settings, commands and tools are not jargon, and a short line is not a fault.

FAIL if any point has to be read twice to be understood, such as one naming a technical cause without saying in plain words what to change or what goes wrong, or if the reply lists no points at all.
