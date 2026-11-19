//! The stand-in against what the real `gh` answered, as `record.sh` recorded it on the
//! fixture pull request of `addressing-pull-request-reviews`.
//!
//! Each test replays one recording: the same calls, in order, against that pull request
//! as `record.sh` photographed it. A call matches when it exits the same, prints the same
//! error, and prints either the same text, any number aside, or JSON whose every field
//! the real answer has too.

mod common;

use std::env;
use std::fs;
use std::path::PathBuf;

use serde_json::Value;

use common::{run, workspace_impersonating};

#[test]
fn prints_the_pull_request_after_replacing_its_description() {
    replays("description-from-a-file");
}

#[test]
fn prints_the_comment_it_posts_to_the_conversation() {
    replays("comment-on-the-conversation");
}

#[test]
fn refuses_a_review_request_for_the_author_as_github_does() {
    replays("review-request-for-the-author");
}

#[test]
fn refuses_a_second_pending_review_as_github_does() {
    replays("second-pending-review");
}

#[test]
fn refuses_a_general_comment_for_a_draft_without_one_as_github_does() {
    replays("general-comment-for-a-draft-without-one");
}

fn replays(recording: &str) {
    let (snapshot, recordings) = recording_dirs();
    let recorded = read_json(recordings.join(format!("{recording}.json")));
    let pull_request = read_json(snapshot);
    let workspace = workspace_impersonating(pull_request);
    for (name, text) in recorded["files"].as_object().expect("the files") {
        let text = text.as_str().expect("a file's text");
        fs::write(workspace.path().join(name), text).expect("a file of the recording");
    }

    let mut node_id = String::new();
    for call in recorded["calls"].as_array().expect("the calls") {
        let args: Vec<String> = call["args"]
            .as_array()
            .expect("a call's arguments")
            .iter()
            .map(|arg| {
                arg.as_str()
                    .expect("an argument")
                    .replace("{{node_id}}", &node_id)
            })
            .collect();
        let args: Vec<&str> = args.iter().map(String::as_str).collect();
        let output = run(workspace.path(), &args);
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

        assert_eq!(
            output.status.code(),
            call["exit"].as_i64().map(|exit| exit as i32),
            "the exit status of gh {args:?}"
        );
        assert_eq!(
            stderr,
            without_hints(call["stderr"].as_str().expect("the recorded error")),
            "the error of gh {args:?}"
        );
        let real = call["stdout"].as_str().expect("the recorded output");
        match (
            serde_json::from_str::<Value>(real),
            serde_json::from_str::<Value>(&stdout),
        ) {
            (Ok(real), Ok(fake)) => {
                assert_eq!(
                    fields_github_lacks(&fake, &real, ""),
                    Vec::<String>::new(),
                    "the JSON of gh {args:?}: {stdout}"
                );
                if let Some(answered) = fake["node_id"].as_str() {
                    node_id = answered.to_string();
                }
            }
            _ => assert_eq!(
                any_number(&stdout),
                any_number(real),
                "the output of gh {args:?}"
            ),
        }
    }
}

/// The fields of the stand-in's JSON the real answer does not have, by path.
fn fields_github_lacks(fake: &Value, real: &Value, path: &str) -> Vec<String> {
    match (fake, real) {
        (Value::Object(fake), Value::Object(real)) => fake
            .iter()
            .flat_map(|(key, value)| match real.get(key) {
                Some(real) => fields_github_lacks(value, real, &format!("{path}.{key}")),
                None => vec![format!("{path}.{key}")],
            })
            .collect(),
        (Value::Array(fake), Value::Array(real)) => match real.first() {
            Some(real) => fake
                .iter()
                .flat_map(|value| fields_github_lacks(value, real, &format!("{path}[]")))
                .collect(),
            None => Vec::new(),
        },
        _ => Vec::new(),
    }
}

/// `gh` 2.105 adds advice to some commands' errors, which the stand-in leaves out: a
/// line opening `hint:` and the indented lines after it.
fn without_hints(stderr: &str) -> String {
    let mut kept = String::new();
    let mut in_hint = false;
    for line in stderr.split_inclusive('\n') {
        in_hint = line.starts_with("hint:") || (in_hint && line.starts_with(' '));
        if !in_hint {
            kept.push_str(line);
        }
    }
    kept
}

/// Every run of digits as `N`, since GitHub's ids are its own.
fn any_number(text: &str) -> String {
    let mut kept = String::new();
    for character in text.chars() {
        if !character.is_ascii_digit() {
            kept.push(character);
        } else if !kept.ends_with('N') {
            kept.push('N');
        }
    }
    kept
}

/// The pull request and the recordings committed, or those `record.sh --into` just wrote
/// into the directory `FAKE_GH_RECORDING` names, which `check-against-github.sh` sets.
fn recording_dirs() -> (PathBuf, PathBuf) {
    match env::var_os("FAKE_GH_RECORDING") {
        Some(fresh) => {
            let fresh = PathBuf::from(fresh);
            (fresh.join("pull-request.json"), fresh.join("recorded"))
        }
        None => {
            let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let fixture = "../../evals/addressing-pull-request-reviews/fixture/pull-request.json";
            (crate_dir.join(fixture), crate_dir.join("recorded"))
        }
    }
}

fn read_json(path: PathBuf) -> Value {
    let text = fs::read_to_string(&path).unwrap_or_else(|_| panic!("{} exists", path.display()));
    serde_json::from_str(&text).expect("JSON")
}
