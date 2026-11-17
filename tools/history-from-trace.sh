#!/bin/sh
# Rebuilds the conversation a later-turn case resumes, from the trace of a `plugin eval`
# run kept with `--keep-temp`, since such a run saves no session file of its own.
#
#   tools/history-from-trace.sh <trace.jsonl> <prompt> [<earlier history.jsonl>] > history.jsonl
#
# <prompt> is the recorded turn's prompt, which the trace leaves out. An earlier history,
# such as the one a second-turn case resumes, gets the new turn appended after it, for a
# case resuming at a third turn.
#
# Paths inside the run's workspace become relative: Claude copies a recorded path as it
# is, so a placeholder would send its edits into a directory named after it. The run's
# home and the real one become `<home>`, the real temporary directory `<tmp>`, and the
# login, which `ls -l` prints as the owner of every file, `<owner>`. `tools/check.sh` is
# what proves nothing machine-local is left.
set -eu

if [ $# -lt 2 ] || [ $# -gt 3 ]; then
    echo "usage: tools/history-from-trace.sh <trace.jsonl> <prompt> [<earlier history.jsonl>]" >&2
    exit 64
fi
trace="$1"
prompt="$2"
earlier="${3:-/dev/null}"

jq -c -n \
    --slurpfile trace "$trace" \
    --slurpfile earlier "$earlier" \
    --arg prompt "$prompt" \
    --arg uuid "$(uuidgen | tr 'A-Z' 'a-z')" \
    --arg home "$HOME" \
    --arg login "$(id -un)" \
    --arg tmp "${TMPDIR:-/tmp}" '
    def variants: if startswith("/private/") then ., ltrimstr("/private") else . end;

    ($trace[] | select(.type == "system" and .subtype == "init")) as $init
    | $init.cwd as $workspace
    | ($workspace | sub("/[^/]*$"; "")) as $run_home
    # Longest first, since the run home holds the workspace.
    | [ ($workspace + "/" | variants | [., ""]),
        ($workspace | variants | [., "."]),
        ($run_home | variants | [., "<home>"]),
        [$home, "<home>"],
        ($tmp | rtrimstr("/") | ("/private" + .), . | [., "<tmp>"]) ] as $pairs
    | def clean:
        reduce $pairs[] as $pair (.; split($pair[0]) | join($pair[1]))
        | gsub("\\b" + $login + "\\b"; "<owner>");

    ($earlier | first | .sessionId // $init.session_id) as $session
    | ($earlier | last | .uuid) as $after
    | [ { type: "user",
          uuid: $uuid,
          timestamp: (first($trace[] | .timestamp // empty)),
          message: { role: "user", content: $prompt } } ]
      + [ $trace[]
          | select((.type == "user" or .type == "assistant") and .parent_tool_use_id == null)
          | { type, uuid, timestamp, message }
            + (if has("tool_use_result") then { toolUseResult: .tool_use_result } else {} end) ]
    | map(. + { isSidechain: false, userType: "external", cwd: ".", sessionId: $session,
                version: $init.claude_code_version })
    | reduce .[] as $entry ({ previous: $after, entries: [] };
        { previous: $entry.uuid, entries: (.entries + [$entry + { parentUuid: .previous }]) })
    | $earlier[], (.entries[] | walk(if type == "string" then clean else . end))
'
