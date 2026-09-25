#!/bin/sh
# Runs several scenarios side by side, each on its own pull request, with the plugin
# either as installed or disabled.
#
#   arm.sh <with|without> <model> <NN> [<NN>...]
#
# Scenario NN runs on branch `guided-<N>`, so 01 on `guided-1`. Its turns are the
# blockquoted lines of its `NN-*.md`, in order, one turn each: those files quote nothing
# else, so the prompts have one home and are read from it rather than copied here.
#
# `without` disables the plugin once around every run, for the same reason baseline.sh
# exists, and switches it back on from a trap. Each run's output prefix is printed as it
# starts, and a run's own report lands in `<prefix>.out`.
set -eu

[ "$#" -ge 3 ] || {
    echo "usage: arm.sh <with|without> <model> <NN> [<NN>...]" >&2
    exit 2
}
arm="$1"
model="$2"
shift 2

here="$(cd "$(dirname "$0")" && pwd)"
plugin=myai@skills-dir

case "$arm" in
    with) ;;
    without)
        trap 'claude plugin enable "$plugin" >/dev/null 2>&1' EXIT INT TERM
        claude plugin disable "$plugin" >/dev/null 2>&1
        # Read from the Status line of this plugin's own entry, found by name, since how many
        # lines separate it from the section header is the listing's layout to change.
        loaded=$(claude plugin list | awk -v p="$plugin" '$0 ~ p {f=1} f && /Status:/ {print; exit}' \
            | grep -c 'loaded' || true)
        echo "$plugin reporting loaded: $loaded (expected 0)"
        [ "$loaded" -eq 0 ] || {
            echo "arm: the plugin is still loaded, so this would not be a baseline" >&2
            exit 1
        }
        ;;
    *)
        echo "arm: the first argument is with or without, not $arm" >&2
        exit 2
        ;;
esac

pids=""
for scenario in "$@"; do
    file=$(ls "$here/$scenario"-*.md)
    branch="guided-$(echo "$scenario" | sed 's/^0*//')"
    prefix="$(mktemp -d)/run"
    echo "$scenario on $branch: $prefix"
    (
        set --
        while IFS= read -r line; do
            set -- "$@" "$line"
        done <<EOF
$(sed -n 's/^> //p' "$file")
EOF
        EVAL_BRANCH="$branch" EVAL_RUN_OUT="$prefix" "$here/run.sh" "$model" "$@"
    ) > "$prefix.out" 2>&1 &
    pids="$pids $!"
done

status=0
for pid in $pids; do
    wait "$pid" || status=1
done
exit "$status"
