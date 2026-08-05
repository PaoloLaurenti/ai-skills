#!/bin/sh
# Creates the sandbox the review scenarios run against: one repo, and one branch
# and open pull request per scenario.
#
# The file contents are written out here rather than generated, because the
# scenarios assert on line numbers and a fixture that drifts turns a passing rule
# into a failing one. The owner comes from the authenticated login at run time, so
# no name is written down.
#
# Run once. To rebuild, delete the repo and run it again.
set -eu

login="$(gh api user -q .login)"
repo="${EVAL_FIXTURE_REPO:-$login/ai-skills-eval-fixture}"

if gh repo view "$repo" >/dev/null 2>&1; then
    echo "setup: $repo already exists." >&2
    echo "Delete it to rebuild, or set EVAL_FIXTURE_REPO to another name." >&2
    exit 1
fi

work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
cd "$work"
git init -q -b main

# The fixture's commits carry an identity of their own. A machine's git config may
# scope its identity to a directory this temporary clone does not sit under, and
# these commits belong to the account that will own the pull requests anyway.
git config user.name "$login"
git config user.email "$login@users.noreply.github.com"

mkdir -p config app

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
gh repo create "$repo" --private --source=. --push \
    --description "Fixture pull requests for the writing-pr-reviews evaluations"

# Each scenario branches from main, carries one change, and opens a pull request.
# The pull request body never hints at the finding: a scenario where the diff
# explains itself tests nothing.
open_pr() {
    branch="$1"
    title="$2"
    body="$3"
    git add -A
    git commit -qm "$title"
    git push -q -u origin "$branch"
    gh pr create --repo "$repo" --head "$branch" --base main \
        --title "$title" --body "$body"
}

# 01. Pending state. One finding, small enough that the review reads as finished.
git checkout -q -b pending-state main
perl -pi -e 's{^FROM python:3\.12-slim$}{FROM python:3-slim}' Dockerfile
open_pr pending-state "track the current python 3 image" \
    "Keeps the base image on the current 3.x release."

# 02. Inline versus general comment. The breach is a repo rule, and the file
# carrying it is in the diff.
git checkout -q -b inline-versus-body main
perl -ni -e 'print unless /^set -eu$/' docker-entrypoint.sh
perl -pi -e 's{^echo "running migrations"$}{echo "starting up"\necho "running migrations"}' docker-entrypoint.sh
open_pr inline-versus-body "log a line before the migrations run" \
    "Adds a startup line to the entrypoint so the logs show it came up."

# 03. Permalinks and line numbers. Three files end up named: two in the diff, one
# outside it, and the two settings disagree.
git checkout -q -b permalinks main
perl -pi -e 's{^EXPOSE 8080$}{ENV APP_TIMEOUT=60\nEXPOSE 8080}' Dockerfile
printf '\n[limits]\ntimeout_seconds = 30\n' >> config/default.toml
open_pr permalinks "make the request timeout configurable" \
    "Adds a timeout setting so it stops being hardcoded."

# 04. Suggestion blocks. Three lines that collapse into one, and a line that
# should simply go.
git checkout -q -b suggestions main
perl -pi -e 's{^COPY --chmod=755 docker-entrypoint\.sh \./$}{COPY docker-entrypoint.sh ./\nRUN chmod +x docker-entrypoint.sh\nRUN chown 1000:1000 docker-entrypoint.sh}' Dockerfile
perl -pi -e 's{^echo "running migrations"$}{echo "DEBUG: entrypoint starting"\necho "running migrations"}' docker-entrypoint.sh
open_pr suggestions "give the entrypoint an owner and the executable bit" \
    "Sets ownership and permissions on the entrypoint explicitly."

# 05. Docs cross-check. The README claim goes stale and the README is not in the
# diff.
git checkout -q -b docs-cross-check main
perl -pi -e 's{^EXPOSE 8080$}{EXPOSE 3000}' Dockerfile
perl -pi -e 's{^port = 8080$}{port = 3000}' config/default.toml
perl -pi -e 's{^PORT = 8080$}{PORT = 3000}' app/server.py
open_pr docs-cross-check "serve on port 3000" \
    "Moves the listening port off 8080, which collides with another service."

echo
echo "Fixture ready at $repo. Pull request numbers, by scenario branch:"
gh pr list --repo "$repo" --state open --json number,headRefName \
    -q '.[] | "  \(.headRefName)\t#\(.number)"'
