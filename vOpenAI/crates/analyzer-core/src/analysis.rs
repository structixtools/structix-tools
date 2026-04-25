use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;

use crate::detectors::clone::detect_clone_drifts;
use crate::input::git::{changed_files, read_file, tracked_files, GitError};
use crate::input::snapshot::ChangeKind;
use crate::ir::file::FileAnalysis;
use crate::ir::model::{Evidence, Finding, Severity};
use crate::lang::{Language, ParseError};
use crate::report::{build_report_with_context, RenamedFile, Report};

#[derive(Debug)]
pub enum AnalysisError {
    Git(GitError),
    Parse(ParseError),
    UnsupportedLanguage { path: String },
}

impl std::fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Git(err) => write!(f, "{err}"),
            Self::Parse(err) => write!(f, "{err}"),
            Self::UnsupportedLanguage { path } => write!(f, "unsupported language for {path}"),
        }
    }
}

impl std::error::Error for AnalysisError {}

impl From<GitError> for AnalysisError {
    fn from(value: GitError) -> Self {
        Self::Git(value)
    }
}

impl From<ParseError> for AnalysisError {
    fn from(value: ParseError) -> Self {
        Self::Parse(value)
    }
}

pub fn analyze_repo(
    repo: impl AsRef<Path>,
    base_ref: &str,
    head_ref: &str,
) -> Result<Report, AnalysisError> {
    let repo = repo.as_ref();
    let changes = changed_files(repo, base_ref, head_ref)?;

    let mut base_files = Vec::new();
    let mut head_files = Vec::new();
    let mut seen_base_paths = BTreeSet::new();
    let mut seen_head_paths = BTreeSet::new();
    let mut renames = Vec::new();

    for change in changes {
        match change.kind {
            ChangeKind::Added | ChangeKind::Copied { .. } => {
                if let Some(head_file) = load_revision_analysis(repo, head_ref, &change.path)? {
                    seen_head_paths.insert(head_file.path.clone());
                    head_files.push(head_file);
                }
            }
            ChangeKind::Modified => {
                if let Some(base_file) = load_revision_analysis(repo, base_ref, &change.path)? {
                    seen_base_paths.insert(base_file.path.clone());
                    base_files.push(base_file);
                }
                if let Some(head_file) = load_revision_analysis(repo, head_ref, &change.path)? {
                    seen_head_paths.insert(head_file.path.clone());
                    head_files.push(head_file);
                }
            }
            ChangeKind::Deleted => {
                if let Some(base_file) = load_revision_analysis(repo, base_ref, &change.path)? {
                    seen_base_paths.insert(base_file.path.clone());
                    base_files.push(base_file);
                }
            }
            ChangeKind::Renamed { from } => {
                if let Some(base_file) = load_revision_analysis(repo, base_ref, &from)? {
                    seen_base_paths.insert(base_file.path.clone());
                    base_files.push(base_file);
                }
                if let Some(head_file) = load_revision_analysis(repo, head_ref, &change.path)? {
                    seen_head_paths.insert(head_file.path.clone());
                    head_files.push(head_file);
                }
                renames.push(RenamedFile::new(from, change.path));
            }
        }
    }

    for path in tracked_files(repo, base_ref)? {
        if seen_base_paths.contains(&path) {
            continue;
        }

        if let Some(base_file) = load_revision_analysis(repo, base_ref, &path)? {
            base_files.push(base_file);
        }
    }

    for path in tracked_files(repo, head_ref)? {
        if seen_head_paths.contains(&path) {
            continue;
        }

        if let Some(head_file) = load_revision_analysis(repo, head_ref, &path)? {
            head_files.push(head_file);
        }
    }

    let mut report =
        build_report_with_context(base_ref, head_ref, &base_files, &head_files, &renames);

    let head_index: BTreeMap<&str, &FileAnalysis> = head_files
        .iter()
        .map(|file| (file.path.as_str(), file))
        .collect();
    let clone_drifts = detect_clone_drifts(&base_files, &head_files);

    for drift in &clone_drifts {
        report
            .findings
            .push(clone_drift_finding(drift, &head_index));
    }

    if !clone_drifts.is_empty() {
        report.summary.push_str(&format!(
            " Also detected {} clone drift group{}.",
            clone_drifts.len(),
            if clone_drifts.len() == 1 { "" } else { "s" }
        ));
    }

    Ok(report)
}

fn load_revision_analysis(
    repo: &Path,
    rev: &str,
    path: &str,
) -> Result<Option<FileAnalysis>, AnalysisError> {
    let Some(language) = Language::from_path(path) else {
        return Ok(None);
    };

    let source = read_file(repo, rev, path)?;
    let analysis = language.parse(path, &source)?;
    Ok(Some(analysis))
}

fn clone_drift_finding(
    drift: &crate::detectors::clone::CloneDrift,
    head_index: &BTreeMap<&str, &FileAnalysis>,
) -> Finding {
    let members = drift
        .members
        .iter()
        .map(|member| format!("`{member}`"))
        .collect::<Vec<_>>()
        .join(", ");
    let changed_members = drift
        .changed_members
        .iter()
        .map(|member| format!("`{member}`"))
        .collect::<Vec<_>>()
        .join(", ");

    let mut evidence = Evidence::new(format!(
        "clone drift: {members}; changed members: {changed_members}"
    ));
    for member in &drift.changed_members {
        if let Some(file) = head_index.get(member.as_str()) {
            for symbol in &file.symbols {
                evidence = evidence.with_span(symbol.span.clone());
            }
        }
    }

    Finding::new(
        "clone.drift",
        Severity::Medium,
        0.97,
        "Clone drift detected",
        format!("Clone group drifted across {members}; changed members: {changed_members}"),
    )
    .with_evidence(evidence)
}
