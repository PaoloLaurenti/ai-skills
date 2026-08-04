//! Mechanical checks over the skill files this repo ships.
//!
//! The rules come from `docs/PLAN.md` and from the frontmatter requirements in
//! `AGENTS.md`. Everything here is about the shape of a skill file, never about
//! the quality of the prose inside it.

use std::fs;
use std::io;
use std::path::Path;

/// What the frontmatter allows a description to run to. Anthropic's limit.
const DESCRIPTION_LIMIT: usize = 1024;

/// How long a skill body runs before it should be split into reference files.
const BODY_LINE_LIMIT: usize = 500;

/// The two fields a SKILL.md frontmatter is required to carry. Both are
/// optional here so that a file missing one is reported against the rule it
/// breaks, rather than as a frontmatter that failed to parse.
#[derive(serde::Deserialize)]
struct Frontmatter {
    name: Option<String>,
    description: Option<String>,
}

/// One violated rule, attributed to the skill directory it was found in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    /// The name of the skill directory, so a report names the skill at fault.
    pub skill: String,
    /// What is wrong, phrased so the report is actionable on its own.
    pub message: String,
}

/// Checks every `<dir>/<skill-name>/SKILL.md` and collects one problem per
/// violated rule. An empty result means the tree is in line.
///
/// Only failures to read the tree surface as an `Err`. A skill that breaks a
/// rule is a `Problem`, including one whose frontmatter does not parse.
pub fn check_skills(dir: &Path) -> io::Result<Vec<Problem>> {
    let mut problems = Vec::new();
    for skill in skill_directories(dir)? {
        let content = fs::read_to_string(dir.join(&skill).join("SKILL.md"))?;
        problems.extend(check_skill(&skill, &content));
    }
    Ok(problems)
}

fn check_skill(skill: &str, content: &str) -> Vec<Problem> {
    let report = |message: String| Problem {
        skill: skill.to_owned(),
        message,
    };

    let Some((frontmatter, body)) = split_frontmatter(content) else {
        return vec![report(
            "the frontmatter is missing: a SKILL.md opens with a --- fenced YAML block".to_owned(),
        )];
    };

    let parsed: Frontmatter = match serde_yaml_ng::from_str(frontmatter) {
        Ok(parsed) => parsed,
        Err(error) => return vec![report(format!("the frontmatter does not parse: {error}"))],
    };

    let mut problems = Vec::new();
    match parsed.name.as_deref() {
        None => problems.push(report("the frontmatter has no name field".to_owned())),
        Some(name) if name != skill => problems.push(report(format!(
            "the name field says {name}, which is not the directory name {skill}"
        ))),
        Some(_) => {}
    }
    match parsed.description.as_deref() {
        None => problems.push(report(
            "the frontmatter has no description field".to_owned(),
        )),
        Some(description) => {
            // Characters rather than bytes, since a description is prose and
            // these are often written partly in Italian.
            let length = description.chars().count();
            if length > DESCRIPTION_LIMIT {
                problems.push(report(format!(
                    "the description is {length} characters, over the {DESCRIPTION_LIMIT} allowed"
                )));
            }
        }
    }

    let body_lines = body.lines().count();
    if body_lines > BODY_LINE_LIMIT {
        problems.push(report(format!(
            "the body is {body_lines} lines, over the {BODY_LINE_LIMIT} a skill stays under"
        )));
    }

    problems
}

/// Splits the leading `---` fenced block from the body that follows it.
/// `None` when the file does not open with a fence or never closes it.
fn split_frontmatter(content: &str) -> Option<(&str, &str)> {
    let after_fence = content.strip_prefix("---\n")?;
    let mut offset = 0;
    for line in after_fence.split_inclusive('\n') {
        if line.trim_end() == "---" {
            return Some((&after_fence[..offset], &after_fence[offset + line.len()..]));
        }
        offset += line.len();
    }
    None
}

/// The skill directory names under `dir`, sorted, so a report comes out in the
/// same order however the filesystem happens to hand them over.
fn skill_directories(dir: &Path) -> io::Result<Vec<String>> {
    let mut names = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.path().is_dir() {
            names.push(entry.file_name().to_string_lossy().into_owned());
        }
    }
    names.sort();
    Ok(names)
}
