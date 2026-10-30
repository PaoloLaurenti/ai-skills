#!/bin/sh
# Links this repo's skills into the places agents read them from, for whoever develops
# them here. Users of the skills install them with `npx skills add` instead.
#
#   tools/link-skills.sh
#
# Safe to run again at any time, after a clone, a new skill or a renamed one: it makes
# what is missing, removes what a rename or a deletion left behind, and leaves the rest.
#
# - Claude Code: `~/.claude/skills/myai`, one link to the repo, which loads every skill
#   as the `myai` plugin. A link there pointing somewhere else is reported and left, since
#   it may be another checkout on purpose.
# - Codex and GitHub Copilot: `~/.agents/skills/<skill>`, one link per skill. A link there
#   pointing into this repo's `skills/` at a skill that no longer exists is removed. Any
#   other entry is left alone: skills from other sources, `npx skills` among them, live
#   there too.
# - The commit hook: `core.hooksPath` set to `.githooks`.
set -eu

repo="$(cd "$(dirname "$0")/.." && pwd -P)"
claude_link="$HOME/.claude/skills/myai"
agents_dir="$HOME/.agents/skills"
status=0

say() { printf '%s\n' "$*"; }

mkdir -p "$(dirname "$claude_link")"
if [ -L "$claude_link" ]; then
    target="$(readlink "$claude_link")"
    if [ "$target" = "$repo" ]; then
        say "ok       $claude_link"
    else
        say "conflict $claude_link points to $target, left as it is"
        status=1
    fi
elif [ -e "$claude_link" ]; then
    say "conflict $claude_link is not a link, left as it is"
    status=1
else
    ln -s "$repo" "$claude_link"
    say "created  $claude_link"
fi

mkdir -p "$agents_dir"
for entry in "$agents_dir"/*; do
    [ -L "$entry" ] || continue
    target="$(readlink "$entry")"
    case "$target" in
        "$repo/skills/"*)
            if [ ! -f "$target/SKILL.md" ]; then
                rm "$entry"
                say "removed  $entry, its skill is gone"
            fi
            ;;
    esac
done

for skill in "$repo"/skills/*/; do
    skill="${skill%/}"
    name="$(basename "$skill")"
    link="$agents_dir/$name"
    if [ -L "$link" ] && [ "$(readlink "$link")" = "$skill" ]; then
        say "ok       $link"
    elif [ -e "$link" ] || [ -L "$link" ]; then
        say "conflict $link exists and is not a link to $skill, left as it is"
        status=1
    else
        ln -s "$skill" "$link"
        say "created  $link"
    fi
done

if [ "$(git -C "$repo" config core.hooksPath || true)" = ".githooks" ]; then
    say "ok       core.hooksPath"
else
    git -C "$repo" config core.hooksPath .githooks
    say "set      core.hooksPath to .githooks"
fi

exit "$status"
