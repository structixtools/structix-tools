use super::filter::is_ignored_path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeKind {
    Added,
    Modified,
    Deleted,
    Renamed { from: String },
    Copied { from: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangedFile {
    pub path: String,
    pub kind: ChangeKind,
}

impl ChangedFile {
    pub fn added(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            kind: ChangeKind::Added,
        }
    }

    pub fn modified(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            kind: ChangeKind::Modified,
        }
    }

    pub fn deleted(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            kind: ChangeKind::Deleted,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snapshot {
    pub base_ref: String,
    pub head_ref: String,
    pub changes: Vec<ChangedFile>,
}

impl Snapshot {
    pub fn new(
        base_ref: impl Into<String>,
        head_ref: impl Into<String>,
        changes: Vec<ChangedFile>,
    ) -> Self {
        Self {
            base_ref: base_ref.into(),
            head_ref: head_ref.into(),
            changes,
        }
    }

    pub fn interesting_changes(&self) -> Vec<&ChangedFile> {
        self.changes
            .iter()
            .filter(|change| !is_ignored_path(&change.path))
            .collect()
    }
}
