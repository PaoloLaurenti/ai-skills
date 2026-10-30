//! What `check_repo` has to do, written before it does anything.
//!
//! Each case builds a throwaway repo in a temporary directory, holding the least
//! a rule needs in order to have something to say. Fixtures are never committed:
//! a deliberately broken file in this repo would be caught by the checks
//! themselves, which is why they take the tree to scan as an argument.

use std::fs;
use std::path::{Path, PathBuf};

use skill_check::repo::{RepoProblem, check_repo};
use tempfile::TempDir;

/// Not the owner of this repo, and not a name anywhere in it: the checks read
/// the name out of the fixture's own `LICENSE`, so a case never has to spell a
/// real one.
const OWNER: &str = "Ada Lovelace";

const SKILL: &str =
    "---\nname: writing-things\ndescription: Use when a test needs a skill.\n---\n\nbody\n";

/// A repo every rule holds in.
fn repo() -> TempDir {
    let repo = TempDir::new().expect("temporary repo");
    write(
        repo.path(),
        "LICENSE",
        &format!("MIT License\n\nCopyright (c) 2026 {OWNER}\n"),
    );
    write(
        repo.path(),
        "README.md",
        "# repo\n\n| Skill | What it covers |\n| --- | --- |\n\
         | [`writing-things`](skills/writing-things/SKILL.md) | Things. |\n",
    );
    write(repo.path(), "skills/writing-things/SKILL.md", SKILL);
    repo
}

fn write(root: &Path, relative: &str, content: &str) {
    let path = root.join(relative);
    fs::create_dir_all(path.parent().expect("a parent directory")).expect("fixture directory");
    fs::write(path, content).expect("fixture file");
}

fn problems(repo: &TempDir) -> Vec<RepoProblem> {
    check_repo(repo.path()).expect("reading the repo")
}

/// Whether any problem says what the case is about, so a rule that fails for an
/// unrelated reason does not read as a pass.
fn mentions(problems: &[RepoProblem], needle: &str) -> bool {
    let needle = needle.to_lowercase();
    problems
        .iter()
        .any(|problem| problem.message.to_lowercase().contains(&needle))
}

/// A machine-local path, assembled rather than written out. A literal one in
/// this file would be a real violation, and the check that scans this repo would
/// report its own specification.
fn machine_local(prefix: &str) -> String {
    format!("the notes are in /{prefix}/someone/dev/notes.txt\n")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the repo root")
}

#[test]
fn a_repo_in_line_is_accepted() {
    assert_eq!(problems(&repo()), vec![]);
}

#[test]
fn a_home_directory_path_is_reported() {
    let repo = repo();
    write(repo.path(), "docs/notes.md", &machine_local("Users"));
    let found = problems(&repo);
    assert!(mentions(&found, "one machine only"), "{found:?}");
}

/// The other spelling of the same mistake, since the rule covers both and a
/// check written against one prefix would pass the other through.
#[test]
fn a_linux_home_directory_path_is_reported() {
    let repo = repo();
    write(repo.path(), "docs/notes.md", &machine_local("home"));
    let found = problems(&repo);
    assert!(mentions(&found, "one machine only"), "{found:?}");
}

/// `AGENTS.md` and `docs/PLAN.md` both have to name the prefixes in order to
/// state the rule. A prefix with no directory after it is a mention, not a path.
#[test]
fn a_bare_prefix_is_left_alone() {
    let repo = repo();
    write(
        repo.path(),
        "docs/notes.md",
        "No machine-local paths: `/Users/`, `/home/`.\n",
    );
    assert_eq!(problems(&repo), vec![]);
}

/// Build output holds absolute paths by design, and version control holds a copy
/// of everything. Scanning either would make the rules unpassable.
#[test]
fn build_output_and_version_control_are_not_scanned() {
    let repo = repo();
    write(
        repo.path(),
        "target/debug/build.log",
        &machine_local("Users"),
    );
    write(repo.path(), ".git/COMMIT_EDITMSG", &machine_local("home"));
    assert_eq!(problems(&repo), vec![]);
}

#[test]
fn the_owner_name_outside_its_two_places_is_reported() {
    let repo = repo();
    write(
        repo.path(),
        "docs/notes.md",
        &format!("Use when {OWNER} asks for a review.\n"),
    );
    let found = problems(&repo);
    assert!(mentions(&found, "names the owner"), "{found:?}");
}

/// The separator between the parts of a name is not what makes it a name, so the
/// check has to see through whatever sits there, or through nothing at all.
#[test]
fn the_owner_name_written_with_any_separator_is_reported() {
    let repo = repo();
    write(
        repo.path(),
        "docs/notes.md",
        "Reachable at ada.lovelace@example.com, or as ada-lovelace, or AdaLovelace.\n",
    );
    let found = problems(&repo);
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(mentions(&found, "names the owner"), "{found:?}");
}

/// The clone URL has to name the owner to be an address, so the README line
/// carrying it is one of the two exceptions.
#[test]
fn the_owner_name_in_the_clone_url_is_left_alone() {
    let repo = repo();
    let readme = fs::read_to_string(repo.path().join("README.md")).expect("the fixture README");
    write(
        repo.path(),
        "README.md",
        &format!(
            "{readme}\n```bash\ngit clone git@github.com:AdaLovelace/repo.git <repo-path>\n```\n"
        ),
    );
    assert_eq!(problems(&repo), vec![]);
}

/// The install command takes the same address as the clone URL, so it names the
/// owner by the same right.
#[test]
fn the_owner_name_in_the_install_command_is_left_alone() {
    let repo = repo();
    let readme = fs::read_to_string(repo.path().join("README.md")).expect("the fixture README");
    write(
        repo.path(),
        "README.md",
        &format!(
            "{readme}
```bash
npx skills add https://github.com/AdaLovelace/repo -g
```
"
        ),
    );
    assert_eq!(problems(&repo), vec![]);
}

/// The name comes out of `LICENSE`, so a `LICENSE` that names nobody leaves the
/// check with nothing to look for. That has to be reported rather than pass.
#[test]
fn a_license_with_no_copyright_line_is_reported() {
    let repo = repo();
    write(
        repo.path(),
        "LICENSE",
        "MIT License\n\nPermission is hereby\n",
    );
    let found = problems(&repo);
    assert!(mentions(&found, "copyright line"), "{found:?}");
}

#[test]
fn a_skill_missing_from_the_readme_table_is_reported() {
    let repo = repo();
    write(repo.path(), "skills/writing-other-things/SKILL.md", SKILL);
    let found = problems(&repo);
    assert!(
        mentions(&found, "skills/writing-other-things/SKILL.md"),
        "{found:?}"
    );
}

/// A skill named in a paragraph is not a listing. The rule asks for a row, so
/// the check reads table rows and nothing else.
#[test]
fn a_skill_named_outside_the_table_is_not_a_row() {
    let repo = repo();
    write(repo.path(), "skills/writing-other-things/SKILL.md", SKILL);
    let readme = fs::read_to_string(repo.path().join("README.md")).expect("the fixture README");
    write(
        repo.path(),
        "README.md",
        &format!("{readme}\nSee also skills/writing-other-things/SKILL.md.\n"),
    );
    let found = problems(&repo);
    assert!(
        mentions(&found, "skills/writing-other-things/SKILL.md"),
        "{found:?}"
    );
}

/// The other direction of the same drift: the skill is gone and its row is still
/// there, sending a reader to a file that no longer exists.
#[test]
fn a_row_pointing_at_no_skill_is_reported() {
    let repo = repo();
    fs::remove_dir_all(repo.path().join("skills/writing-things")).expect("the fixture skill");
    let found = problems(&repo);
    assert!(mentions(&found, "not a skill directory"), "{found:?}");
}

#[test]
fn this_repos_own_tree_passes() {
    let found = check_repo(&repo_root()).expect("reading the repo");
    assert_eq!(found, vec![]);
}
