use std::collections::BTreeMap;

use crate::ir::file::FileAnalysis;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FileChange {
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChangeSet {
    pub added: Vec<FileChange>,
    pub removed: Vec<FileChange>,
    pub modified: Vec<FileChange>,
}

pub fn diff_files(base: &[FileAnalysis], head: &[FileAnalysis]) -> ChangeSet {
    let base_map: BTreeMap<&str, &FileAnalysis> =
        base.iter().map(|file| (file.path.as_str(), file)).collect();
    let head_map: BTreeMap<&str, &FileAnalysis> =
        head.iter().map(|file| (file.path.as_str(), file)).collect();

    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut modified = Vec::new();

    for (path, head_file) in &head_map {
        match base_map.get(path) {
            None => added.push(FileChange {
                path: (*path).to_string(),
            }),
            Some(base_file) => {
                if !same_content(base_file, head_file) {
                    modified.push(FileChange {
                        path: (*path).to_string(),
                    });
                }
            }
        }
    }

    for path in base_map.keys() {
        if !head_map.contains_key(path) {
            removed.push(FileChange {
                path: (*path).to_string(),
            });
        }
    }

    ChangeSet {
        added,
        removed,
        modified,
    }
}

fn same_content(left: &FileAnalysis, right: &FileAnalysis) -> bool {
    left.lexical_hash == right.lexical_hash
        && left.token_hash == right.token_hash
        && left.ast_hash == right.ast_hash
}
