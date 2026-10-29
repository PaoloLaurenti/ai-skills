# Reading the repo around a pull request

The lookups behind two sections of `SKILL.md`: checking the diff against the repo's own
docs, and verifying a claim about the architecture.

## Finding the docs

Every `README.md`, `AGENTS.md` and `CLAUDE.md`, at any depth, so the per-package ones that
nothing in the diff points at are found too:

```bash
find . \( -name 'README.md' -o -name 'AGENTS.md' -o -name 'CLAUDE.md' \) -not -path '*/node_modules/*'
```

## Where a claim about the architecture is answered

The obvious place is usually the wrong one: a crate or service name says nothing reliable
about what is deployed or how.

| Claim about | Where the answer actually is |
|---|---|
| Container or image identity and naming | Dockerfile stages, deploy workflow inputs, ECR repository |
| What a deployed unit runs | `ENTRYPOINT` and `CMD`, not the crate names |
| Local versus deployed topology | compose file for local, deploy workflow for real |
| Infra settings, credentials, roles | the infra repo, not this one |
| Dependency behaviour | upstream source at the pinned version |
