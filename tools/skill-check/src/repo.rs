//! The rules that hold over the whole tree rather than over one skill file.
//!
//! `AGENTS.md` states them in prose: no machine-local paths, no owner name
//! outside the two places allowed to carry it, and a README table row for every
//! skill. They are mechanical, so they should not consume attention in review.
//! They read every text file in the repo, which is why they sit beside the skill
//! checks rather than inside them.

use std::fs;
use std::io;
use std::path::Path;

use crate::skill_directories;

/// Directories the walk never enters: version control, and build output that
/// holds machine-local copies of everything else.
const SKIPPED_DIRECTORIES: [&str; 2] = [".git", "target"];

/// What a path that exists on one machine only starts with.
const HOME_PREFIXES: [&str; 2] = ["/Users/", "/home/"];

/// One violated rule, attributed to the file it was found in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoProblem {
    /// The path relative to the repo root, so a report names a file the reader
    /// can open.
    pub file: String,
    /// What is wrong, phrased so the report is actionable on its own.
    pub message: String,
}

/// Checks the repo-wide rules and collects one problem per violation. An empty
/// result means the tree is in line.
///
/// Only failures to read the tree surface as an `Err`. Everything else, a
/// missing `LICENSE` included, is a `RepoProblem`.
pub fn check_repo(root: &Path) -> io::Result<Vec<RepoProblem>> {
    let files = text_files(root)?;
    let mut problems = Vec::new();
    problems.extend(machine_local_paths(&files));
    problems.extend(owner_name_leaks(&files));
    problems.extend(readme_table_drift(root, &files)?);
    Ok(problems)
}

fn machine_local_paths(files: &[(String, String)]) -> Vec<RepoProblem> {
    let mut problems = Vec::new();
    for (file, content) in files {
        for (number, line) in content.lines().enumerate() {
            if let Some(path) = machine_local_path(line) {
                problems.push(RepoProblem {
                    file: file.clone(),
                    message: format!(
                        "line {} hardcodes {path}, a path that exists on one machine only",
                        number + 1
                    ),
                });
            }
        }
    }
    problems
}

/// The first machine-local path on a line, if there is one.
///
/// A prefix with no directory name after it is a mention of the rule rather than
/// a path, which is what `AGENTS.md` and `docs/PLAN.md` both need in order to
/// state the rule at all. Matching the bare prefix would make this check flag
/// its own specification.
fn machine_local_path(line: &str) -> Option<String> {
    for prefix in HOME_PREFIXES {
        for (at, _) in line.match_indices(prefix) {
            let segment: String = line[at + prefix.len()..]
                .chars()
                .take_while(is_path_character)
                .collect();
            if !segment.is_empty() {
                return Some(format!("{prefix}{segment}"));
            }
        }
    }
    None
}

fn is_path_character(character: &char) -> bool {
    character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-')
}

fn owner_name_leaks(files: &[(String, String)]) -> Vec<RepoProblem> {
    let report = |message: &str| {
        vec![RepoProblem {
            file: "LICENSE".to_owned(),
            message: message.to_owned(),
        }]
    };

    let Some((_, license)) = files.iter().find(|(file, _)| file == "LICENSE") else {
        return report(
            "there is no LICENSE, so there is no copyright line to take the owner name from and \
             the rule that the name appears nowhere else cannot be checked",
        );
    };
    let Some(owner) = owner_name(license) else {
        return report(
            "the copyright line names no owner, so the rule that the name appears nowhere else \
             cannot be checked",
        );
    };

    let owner = normalized(&owner);
    let mut problems = Vec::new();
    for (file, content) in files {
        for (number, line) in content.lines().enumerate() {
            if normalized(line).contains(&owner) && !names_the_owner_by_right(file, line) {
                problems.push(RepoProblem {
                    file: file.clone(),
                    message: format!(
                        "line {} names the owner, which only the repo's GitHub URL in README.md \
                         and the copyright line in LICENSE are allowed to do",
                        number + 1
                    ),
                });
            }
        }
    }
    problems
}

/// The owner's name as `LICENSE` writes it: whatever follows the year on the
/// copyright line. Taking it from the one file allowed to carry it means this
/// check needs no name of its own, which it could not have without breaking the
/// rule it enforces.
fn owner_name(license: &str) -> Option<String> {
    let line = license
        .lines()
        .find(|line| line.trim_start().starts_with("Copyright"))?;
    let mut tokens = line.split_whitespace().skip_while(|token| !is_year(token));
    tokens.next()?;
    let name = tokens.collect::<Vec<_>>().join(" ");
    (!name.is_empty()).then_some(name)
}

fn is_year(token: &str) -> bool {
    token.len() == 4 && token.chars().all(|character| character.is_ascii_digit())
}

/// Letters and digits only, lowercased, so a name matches whatever sits between
/// its parts: `Ada Lovelace`, `ada-lovelace`, `ada.lovelace@example.com`, or the
/// `AdaLovelace` of a clone URL.
///
/// The whole name has to match. A first name on its own would flag this repo's
/// own history, which records a skill once named after its owner.
fn normalized(text: &str) -> String {
    text.chars()
        .filter(|character| character.is_alphanumeric())
        .flat_map(|character| character.to_lowercase())
        .collect()
}

/// The two places `AGENTS.md` allows the owner's name, both because naming the
/// owner is what makes them true: the repo's GitHub URL in the README, in the
/// clone and install commands alike, which is the address the repo lives at,
/// and the copyright line in `LICENSE`, which has to say who holds the copyright.
fn names_the_owner_by_right(file: &str, line: &str) -> bool {
    match file {
        "README.md" => line.contains("github.com"),
        "LICENSE" => line.trim_start().starts_with("Copyright"),
        _ => false,
    }
}

/// The skills table and `skills/` have to say the same thing, in both
/// directions: a skill nobody can find in the README is as much drift as a row
/// pointing at a skill that is no longer there.
fn readme_table_drift(root: &Path, files: &[(String, String)]) -> io::Result<Vec<RepoProblem>> {
    let report = |message: String| RepoProblem {
        file: "README.md".to_owned(),
        message,
    };

    let Some((_, readme)) = files.iter().find(|(file, _)| file == "README.md") else {
        return Ok(vec![report(
            "there is no README.md, so there is no skills table for a skill to appear in"
                .to_owned(),
        )]);
    };

    // A row, not a mention: a skill named in a paragraph is not a listing.
    let listed: Vec<String> = readme
        .lines()
        .filter(|line| line.trim_start().starts_with('|'))
        .flat_map(linked_skills)
        .collect();
    let present = skill_directories(&root.join("skills"))?;

    let mut problems = Vec::new();
    for skill in &present {
        if !listed.contains(skill) {
            problems.push(report(format!(
                "the skills table has no row linking skills/{skill}/SKILL.md"
            )));
        }
    }
    for skill in &listed {
        if !present.contains(skill) {
            problems.push(report(format!(
                "the skills table has a row linking skills/{skill}/SKILL.md, which is not a skill \
                 directory"
            )));
        }
    }
    Ok(problems)
}

/// The skills a table row links to: whatever a link writes between `skills/` and
/// `/SKILL.md`.
fn linked_skills(row: &str) -> Vec<String> {
    let mut names = Vec::new();
    for (at, _) in row.match_indices("skills/") {
        let rest = &row[at + "skills/".len()..];
        if let Some(end) = rest.find("/SKILL.md") {
            let name = &rest[..end];
            if !name.is_empty() && !name.contains('/') {
                names.push(name.to_owned());
            }
        }
    }
    names
}

/// Every file under `root` that reads as text, as its path relative to `root`
/// and its contents, sorted so a report comes out in the same order however the
/// filesystem hands the tree over.
///
/// A file that is not valid UTF-8 is not prose and has no rule here to break, so
/// it is skipped rather than reported.
fn text_files(root: &Path) -> io::Result<Vec<(String, String)>> {
    let mut files = Vec::new();
    collect_text_files(root, root, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_text_files(
    root: &Path,
    directory: &Path,
    files: &mut Vec<(String, String)>,
) -> io::Result<()> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let kind = entry.file_type()?;
        let path = entry.path();
        // Symlinks are followed by neither branch: this repo holds none, and one
        // pointing at an ancestor would walk forever.
        if kind.is_dir() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if !SKIPPED_DIRECTORIES.contains(&name.as_str()) {
                collect_text_files(root, &path, files)?;
            }
        } else if kind.is_file()
            && let Ok(content) = fs::read_to_string(&path)
        {
            let relative = path.strip_prefix(root).unwrap_or(&path);
            files.push((relative.to_string_lossy().into_owned(), content));
        }
    }
    Ok(())
}
