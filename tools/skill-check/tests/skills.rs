//! What `check_skills` has to do, written before it does anything.
//!
//! Each case builds a throwaway skill tree in a temporary directory. Fixtures
//! are never committed: a deliberately broken `SKILL.md` under `skills/` would
//! be caught by the checker itself, which is why the checker takes the tree to
//! scan as an argument rather than finding `skills/` on its own.

use std::fs;
use std::path::{Path, PathBuf};

use skill_check::{Problem, check_skills};
use tempfile::TempDir;

const DESCRIPTION: &str = "Use when a test needs a well formed skill.";

fn tree() -> TempDir {
    TempDir::new().expect("temporary skills tree")
}

/// Writes `<root>/<directory>/SKILL.md`. The frontmatter goes in verbatim, so a
/// case can hand over a malformed one.
fn write_skill(root: &Path, directory: &str, frontmatter: &str, body_lines: usize) {
    let dir = root.join(directory);
    fs::create_dir_all(&dir).expect("fixture directory");
    let body: String = (1..=body_lines)
        .map(|n| format!("body line {n}\n"))
        .collect();
    fs::write(dir.join("SKILL.md"), format!("{frontmatter}\n\n{body}")).expect("fixture skill");
}

fn frontmatter(name: &str, description: &str) -> String {
    format!("---\nname: {name}\ndescription: {description}\n---")
}

fn problems(tree: &TempDir) -> Vec<Problem> {
    check_skills(tree.path()).expect("reading the tree")
}

/// Whether any problem says what the case is about, so a rule that fails for an
/// unrelated reason does not read as a pass.
fn mentions(problems: &[Problem], needle: &str) -> bool {
    let needle = needle.to_lowercase();
    problems
        .iter()
        .any(|problem| problem.message.to_lowercase().contains(&needle))
}

fn repo_skills() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../skills")
        .canonicalize()
        .expect("the repo's own skills directory")
}

#[test]
fn a_well_formed_skill_is_accepted() {
    let tree = tree();
    write_skill(
        tree.path(),
        "writing-things",
        &frontmatter("writing-things", DESCRIPTION),
        10,
    );
    assert_eq!(problems(&tree), vec![]);
}

#[test]
fn a_name_not_matching_the_directory_is_reported() {
    let tree = tree();
    write_skill(
        tree.path(),
        "writing-things",
        &frontmatter("writing-stuff", DESCRIPTION),
        10,
    );
    let found = problems(&tree);
    assert!(mentions(&found, "name"), "{found:?}");
}

#[test]
fn a_missing_description_is_reported() {
    let tree = tree();
    write_skill(
        tree.path(),
        "writing-things",
        "---\nname: writing-things\n---",
        10,
    );
    let found = problems(&tree);
    assert!(mentions(&found, "description"), "{found:?}");
}

#[test]
fn a_description_over_1024_characters_is_reported() {
    let tree = tree();
    write_skill(
        tree.path(),
        "writing-things",
        &frontmatter("writing-things", &"x".repeat(1100)),
        10,
    );
    let found = problems(&tree);
    assert!(mentions(&found, "1024"), "{found:?}");
}

#[test]
fn a_body_over_500_lines_is_reported() {
    let tree = tree();
    write_skill(
        tree.path(),
        "writing-things",
        &frontmatter("writing-things", DESCRIPTION),
        600,
    );
    let found = problems(&tree);
    assert!(mentions(&found, "500"), "{found:?}");
}

#[test]
fn frontmatter_that_does_not_parse_is_reported() {
    let tree = tree();
    write_skill(
        tree.path(),
        "writing-things",
        "name: writing-things\ndescription: No fences at all.",
        10,
    );
    let found = problems(&tree);
    assert!(mentions(&found, "frontmatter"), "{found:?}");
}

/// The shape a long description takes in the wild: the value starts on the line
/// after the key and continues across several lines. Of the 131 `SKILL.md`
/// files reachable on the machine this was written on, 74 quote the
/// description and one wraps it exactly like this. A check that reads only the
/// key's own line sees an empty value here and reports a missing description on
/// a skill that has a perfectly good one.
#[test]
fn a_wrapped_description_is_not_mistaken_for_a_missing_one() {
    let tree = tree();
    let frontmatter = concat!(
        "---\n",
        "name: writing-things\n",
        "description:\n",
        "  \"Use when a description runs long enough to wrap,\n",
        "  which is what a real skill does once it approaches\n",
        "  the limit this checker is here to police.\"\n",
        "---"
    );
    write_skill(tree.path(), "writing-things", frontmatter, 10);
    assert_eq!(problems(&tree), vec![]);
}

/// The same shape, over the limit. No single line comes close to 1024
/// characters, so the check has to measure the value YAML produces rather than
/// any line of the file.
#[test]
fn a_wrapped_description_is_measured_whole() {
    let tree = tree();
    let mut frontmatter = String::from("---\nname: writing-things\ndescription:\n");
    for _ in 0..6 {
        frontmatter.push_str(&format!("  {}\n", "x".repeat(200)));
    }
    frontmatter.push_str("---");
    write_skill(tree.path(), "writing-things", &frontmatter, 10);
    let found = problems(&tree);
    assert!(mentions(&found, "1024"), "{found:?}");
}

#[test]
fn one_bad_skill_among_several_is_reported_by_name() {
    let tree = tree();
    write_skill(
        tree.path(),
        "writing-things",
        &frontmatter("writing-things", DESCRIPTION),
        10,
    );
    write_skill(
        tree.path(),
        "writing-other-things",
        &frontmatter("writing-elsewhere", DESCRIPTION),
        10,
    );
    let found = problems(&tree);
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!(found[0].skill, "writing-other-things");
}

/// A `#` preceded by a space opens a comment in an unquoted YAML scalar, so the
/// value the loader sees stops there. This repo's own skill shipped this way:
/// its description parsed as 147 characters instead of 288, dropping 141 of them
/// and its English trigger phrases with them, while every other check passed.
#[test]
fn a_description_cut_short_by_a_comment_is_reported() {
    let tree = tree();
    write_skill(
        tree.path(),
        "writing-things",
        "---\nname: writing-things\ndescription: Use when asked about PR #12, and about all of this.\n---",
        10,
    );
    let found = problems(&tree);
    assert!(mentions(&found, "comment"), "{found:?}");
}

/// The same text quoted keeps its `#`, so nothing is dropped and nothing is
/// reported. Without this case the rule above would flag every legitimate `#`.
#[test]
fn a_hash_that_survives_parsing_is_left_alone() {
    let tree = tree();
    write_skill(
        tree.path(),
        "writing-things",
        "---\nname: writing-things\ndescription: \"Use when asked about PR #12, and about all of this.\"\n---",
        10,
    );
    assert_eq!(problems(&tree), vec![]);
}

/// The shape a description with a `#` in it should take: a folded block scalar,
/// where `#` is literal text and the lines join into one value. This is the form
/// `AGENTS.md` asks for, so a regression here would take the repo's own skill
/// with it.
#[test]
fn a_block_scalar_description_keeps_its_hash() {
    let tree = tree();
    let frontmatter = concat!(
        "---\n",
        "name: writing-things\n",
        "description: >-\n",
        "  Use when asked about PR #12, and about\n",
        "  everything that follows it.\n",
        "---"
    );
    write_skill(tree.path(), "writing-things", frontmatter, 10);
    assert_eq!(problems(&tree), vec![]);
}

/// A directory under `skills/` with nothing in it is a skill that will not load.
/// It belongs in the report next to every other broken skill, rather than
/// aborting the run before the remaining skills are looked at.
#[test]
fn a_directory_without_a_skill_file_is_reported() {
    let tree = tree();
    fs::create_dir_all(tree.path().join("writing-things")).expect("fixture directory");
    let found = problems(&tree);
    assert!(mentions(&found, "SKILL.md"), "{found:?}");
}

#[test]
fn this_repos_own_skills_pass() {
    let found = check_skills(&repo_skills()).expect("reading the repo's skills");
    assert_eq!(found, vec![]);
}
