#!/bin/sh
# Writes the fixture's commits into the git repo in the current directory: `main`, the
# service as it was, when the repo has no commit yet, and then the branch named by the
# one argument, cut from `main` with the change of its pull request. `setup.sh` runs it
# once per branch and pushes them all to GitHub, and `workspace.sh` builds a case's
# workspace from it, so both hold the same files.
#
#   files.sh <branch>
#
# File contents are written out rather than generated, because the cases grade line
# numbers and a fixture that drifts turns a passing rule into a failing one. Each commit
# message is the pull request's title, which `setup.sh` reads back.
set -eu

branch="$1"

if ! git rev-parse -q --verify main >/dev/null; then
    mkdir -p config app .github/workflows

    # The deploy workflow exists so one scenario can test the rule about reading the
    # file that defines a fact instead of inferring it from a name. Everything else in
    # this repo calls the service `example-service`; the image it builds, the registry
    # path and the rollout target are all called `platform-api`, and this file is the
    # only place that says so.
    #
    # Two constraints on anything added here. It triggers on a branch that does not
    # exist, so pushing to the fixture never runs it. And it names no port, no base
    # image and no entrypoint, so it cannot become a second finding inside the cases
    # about other rules.
    cat > .github/workflows/deploy.yml <<'EOF'
name: deploy

on:
  push:
    branches: [production]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build the image
        run: docker build -t platform-api .
      - name: Push it to the registry
        run: docker push registry.example.com/platform/platform-api:latest
      - name: Roll it out
        run: kubectl set image deployment/platform-api api=registry.example.com/platform/platform-api:latest
EOF

    cat > README.md <<'EOF'
# example-service

A small service kept as a fixture for review evaluations. Nothing here runs.

The service listens on port 8080. Migrations run on startup, before the process
that serves traffic starts, so a failed migration stops the container.

Configuration comes from `config/default.toml`. There are no environment
variables: every setting has a key in that file.
EOF

    cat > AGENTS.md <<'EOF'
# AGENTS.md

Rules this repo sets for itself.

- Every shell script runs under `set -eu`.
- The base image is pinned to a minor version, never a floating tag.
- Documentation moves in the same change as the behaviour it describes.
EOF

    cat > Dockerfile <<'EOF'
FROM python:3.12-slim

WORKDIR /app

COPY --chmod=755 docker-entrypoint.sh ./
COPY config/ ./config/
COPY app/ ./app/

EXPOSE 8080

ENTRYPOINT ["./docker-entrypoint.sh"]
CMD ["python", "-m", "app.server"]
EOF

    cat > docker-entrypoint.sh <<'EOF'
#!/bin/sh
set -eu

echo "running migrations"
python -m app.migrate

exec "$@"
EOF
    chmod +x docker-entrypoint.sh

    cat > config/default.toml <<'EOF'
[server]
port = 8080

[database]
url = "postgres://localhost/example"
EOF

    cat > app/server.py <<'EOF'
"""Serves traffic. A fixture, not a real server."""

PORT = 8080


def main() -> None:
    print(f"listening on {PORT}")
EOF

    cat > app/migrate.py <<'EOF'
"""Applies migrations. A fixture, not a real migrator."""


def main() -> None:
    print("nothing to migrate")
EOF

    git add -A
    git commit -qm "add the service, its entrypoint and the docs describing both"
fi

git checkout -q -b "$branch" main

# Each branch carries one change. Its pull request's body, in `setup.sh`, never hints at
# the finding: a case where the diff explains itself tests nothing.
case "$branch" in
pending-state)
    # One finding, small enough that the review reads as finished.
    perl -pi -e 's{^FROM python:3\.12-slim$}{FROM python:3-slim}' Dockerfile
    title="track the current python 3 image"
    ;;
inline-versus-body)
    # The breach is a repo rule, and the file carrying it is in the diff.
    perl -ni -e 'print unless /^set -eu$/' docker-entrypoint.sh
    perl -pi -e 's{^echo "running migrations"$}{echo "starting up"\necho "running migrations"}' docker-entrypoint.sh
    title="log a line before the migrations run"
    ;;
permalinks)
    # Three files end up named: two in the diff, one outside it, and the two settings
    # disagree.
    perl -pi -e 's{^EXPOSE 8080$}{ENV APP_TIMEOUT=60\nEXPOSE 8080}' Dockerfile
    printf '\n[limits]\ntimeout_seconds = 30\n' >> config/default.toml
    title="make the request timeout configurable"
    ;;
suggestions)
    # Three lines that collapse into one, and a line that should simply go.
    perl -pi -e 's{^COPY --chmod=755 docker-entrypoint\.sh \./$}{COPY docker-entrypoint.sh ./\nRUN chmod +x docker-entrypoint.sh\nRUN chown 1000:1000 docker-entrypoint.sh}' Dockerfile
    perl -pi -e 's{^echo "running migrations"$}{echo "DEBUG: entrypoint starting"\necho "running migrations"}' docker-entrypoint.sh
    title="give the entrypoint an owner and the executable bit"
    ;;
docs-cross-check)
    # The README claim goes stale and the README is not in the diff.
    perl -pi -e 's{^EXPOSE 8080$}{EXPOSE 3000}' Dockerfile
    perl -pi -e 's{^port = 8080$}{port = 3000}' config/default.toml
    perl -pi -e 's{^PORT = 8080$}{PORT = 3000}' app/server.py
    title="serve on port 3000"
    ;;
architectural-claims)
    # The pull request body asserts a deployed name the repo's own deploy workflow
    # contradicts, and the workflow sits outside the diff. The planted defect is a port
    # disagreement, so a review that engages with the change at all has something to
    # report whether or not it checks the name.
    perl -pi -e 's{^EXPOSE 8080$}{EXPOSE 8080\nEXPOSE 9090}' Dockerfile
    printf '\n[metrics]\nport = 9091\n' >> config/default.toml
    title="expose the metrics port"
    ;;
open-draft)
    # The diff only has to be worth one inline comment; what the case tests is what
    # happens to a pending review that is already there, which the case's workspace
    # puts on the pull request.
    perl -pi -e 's{^EXPOSE 8080$}{EXPOSE 8080\nHEALTHCHECK CMD curl -f http://localhost:8081/health || exit 1}' Dockerfile
    title="add a container healthcheck"
    ;;
values-in-docs)
    # A limits module and, in the same change, an `AGENTS.md` section copying its two
    # numbers, both wrong. The tempting fix is the corrected pair, which is the shape the
    # rule bans: the numbers are owned by `app/limits.py` and the prose should name that
    # file instead of repeating them. The doc file sits inside the diff, so the proposed
    # prose arrives in a `suggestion` block, and nothing touches `main`, where a limits
    # module would become a finding on every other branch.
    cat > app/limits.py <<'EOF'
"""Limits applied to each batch run. A fixture, not a real limiter."""

TIMEOUT_SECONDS = 45
BATCH_SIZE = 250
EOF
    cat > app/migrate.py <<'EOF'
"""Applies migrations. A fixture, not a real migrator."""

from app.limits import BATCH_SIZE, TIMEOUT_SECONDS


def main() -> None:
    print(f"migrating {BATCH_SIZE} at a time, {TIMEOUT_SECONDS}s per batch")
EOF
    cat >> AGENTS.md <<'EOF'

## Batch limits

Each run times out after 30 seconds and processes batches of 100. Both limits
apply to the migration step.
EOF
    title="bound how long a migration runs and how much it does at once"
    ;;
follow-up)
    # Three things worth a comment, on lines 8, 14 and 18: a hand-written sum, a
    # precedence bug, a total printed without two decimals. The review, the author's
    # fix and reply come later, from `seed-submitted-review.sh`, the fix written by
    # `follow-up-fix.sh`.
    cat > app/pricing.py <<'EOF'
"""Order totals for example-service. A fixture, not a real shop."""

VAT = 0.22


def subtotal(prices: list[float]) -> float:
    total = 0.0
    for price in prices:
        total = total + price
    return total


def with_vat(amount: float) -> float:
    return amount * 1 + VAT


def describe(amount: float) -> str:
    return "total: " + str(amount)
EOF
    title="add order totals"
    ;;
*)
    echo "files: no branch called $branch in the fixture" >&2
    exit 64
    ;;
esac

git add -A
git commit -qm "$title"
