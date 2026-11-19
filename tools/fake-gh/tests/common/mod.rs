//! Running the built `gh` in a throwaway workspace, as every test file does.
// Each test file is a crate of its own and uses only some of these.
#![allow(dead_code)]

use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Output, Stdio};

use serde_json::Value;
use tempfile::TempDir;

pub fn workspace_impersonating(pull_request: Value) -> TempDir {
    let workspace = TempDir::new().expect("temporary workspace");
    let state = workspace.path().join(".git/fake-gh");
    fs::create_dir_all(&state).expect("state directory");
    fs::write(state.join("pull-request.json"), pull_request.to_string()).expect("pull request");
    workspace
}

/// The calls `gh` logged, one JSON object per line, as a grader reads them.
pub fn recorded_calls(workspace: &TempDir) -> Vec<Value> {
    fs::read_to_string(workspace.path().join(".git/fake-gh/calls.jsonl"))
        .unwrap_or_default()
        .lines()
        .map(|line| serde_json::from_str(line).expect("a call as JSON"))
        .collect()
}

/// Runs the built `gh` in the workspace and reads its answer as JSON.
pub fn gh(workspace: &TempDir, args: &[&str]) -> Value {
    answer_as_json(args, &run(workspace.path(), args))
}

/// Runs the built `gh` in the workspace and reads its answer as text.
pub fn gh_text(workspace: &TempDir, args: &[&str]) -> String {
    String::from_utf8(run(workspace.path(), args).stdout).expect("gh prints UTF-8")
}

pub fn run(directory: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_gh"))
        .args(args)
        .current_dir(directory)
        .output()
        .expect("gh runs")
}

/// Runs the built `gh` in the workspace with `input` on its standard input, as a pipe
/// into `--input -` does, and reads its answer as JSON.
pub fn gh_fed(workspace: &TempDir, args: &[&str], input: &str) -> Value {
    answer_as_json(args, &run_fed(workspace, args, input))
}

/// Runs the built `gh` in the workspace with `input` on its standard input.
pub fn run_fed(workspace: &TempDir, args: &[&str], input: &str) -> Output {
    let mut gh = Command::new(env!("CARGO_BIN_EXE_gh"))
        .args(args)
        .current_dir(workspace.path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("gh runs");
    gh.stdin
        .take()
        .expect("gh's input")
        .write_all(input.as_bytes())
        .expect("the input reaches gh");
    gh.wait_with_output().expect("gh finishes")
}

fn answer_as_json(args: &[&str], output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
        panic!(
            "gh {args:?} printed no JSON ({error}): {}",
            String::from_utf8_lossy(&output.stdout)
        )
    })
}
