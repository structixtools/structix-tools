use std::path::Path;
use std::process::Command;

use super::snapshot::{ChangeKind, ChangedFile};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitError {
    pub message: String,
}

impl GitError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for GitError {}

pub fn changed_files(
    repo: impl AsRef<Path>,
    base_ref: &str,
    head_ref: &str,
) -> Result<Vec<ChangedFile>, GitError> {
    let output = git_command(
        repo.as_ref(),
        [
            "diff",
            "--name-status",
            "--find-renames",
            base_ref,
            head_ref,
        ],
    )?;
    let text = String::from_utf8(output.stdout)
        .map_err(|err| GitError::new(format!("git diff output was not valid UTF-8: {err}")))?;

    let mut changes = Vec::new();

    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let mut fields = line.split('\t');
        let status = fields
            .next()
            .ok_or_else(|| GitError::new("missing git diff status"))?;

        let kind = status.chars().next().unwrap_or('M');
        let change = match kind {
            'A' => ChangedFile::added(fields.next().unwrap_or_default()),
            'M' => ChangedFile::modified(fields.next().unwrap_or_default()),
            'D' => ChangedFile::deleted(fields.next().unwrap_or_default()),
            'R' => {
                let from = fields.next().unwrap_or_default().to_string();
                let to = fields.next().unwrap_or_default().to_string();
                ChangedFile {
                    path: to,
                    kind: ChangeKind::Renamed { from },
                }
            }
            'C' => {
                let from = fields.next().unwrap_or_default().to_string();
                let to = fields.next().unwrap_or_default().to_string();
                ChangedFile {
                    path: to,
                    kind: ChangeKind::Copied { from },
                }
            }
            _ => continue,
        };

        changes.push(change);
    }

    Ok(changes)
}

pub fn read_file(repo: impl AsRef<Path>, rev: &str, path: &str) -> Result<String, GitError> {
    let object = format!("{rev}:{path}");
    let output = git_command(repo.as_ref(), ["show", &object])?;
    String::from_utf8(output.stdout)
        .map_err(|err| GitError::new(format!("git show output was not valid UTF-8: {err}")))
}

pub fn tracked_files(repo: impl AsRef<Path>, rev: &str) -> Result<Vec<String>, GitError> {
    let output = git_command(repo.as_ref(), ["ls-tree", "-r", "--name-only", rev])?;
    let text = String::from_utf8(output.stdout)
        .map_err(|err| GitError::new(format!("git ls-tree output was not valid UTF-8: {err}")))?;

    Ok(text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect())
}

fn git_command<const N: usize>(
    repo: &Path,
    args: [&str; N],
) -> Result<std::process::Output, GitError> {
    let output = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .map_err(|err| GitError::new(format!("failed to run git: {err}")))?;

    if output.status.success() {
        return Ok(output);
    }

    Err(GitError::new(format!(
        "git command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    )))
}
