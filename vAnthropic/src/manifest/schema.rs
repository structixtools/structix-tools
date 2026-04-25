use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeKind {
    Added,
    Removed,
    Modified,
    Moved,
    Renamed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityChange {
    pub kind: ChangeKind,
    pub entity_key: String,
    pub name: String,
    pub file_path: String,
    pub entity_kind: String,
    pub before_signature: Option<String>,
    pub after_signature: Option<String>,
    pub before_source: Option<String>,
    pub after_source: Option<String>,
    pub old_file_path: Option<String>,
    pub old_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeManifest {
    pub from_ref: String,
    pub to_ref: String,
    pub changes: Vec<EntityChange>,
    pub files_affected: Vec<String>,
    pub summary: Option<String>,
}
