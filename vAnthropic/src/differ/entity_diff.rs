use std::collections::HashMap;
use crate::parser::types::CodeEntity;
use crate::manifest::schema::{ChangeKind, EntityChange};

fn identity_key(e: &CodeEntity) -> String {
    format!("{}::{}::{}", e.kind.as_str(), e.parent.as_deref().unwrap_or(""), e.name)
}

/// Returns the set of identity keys that appear in more than one distinct file.
/// These are ambiguous — we cannot reliably detect moves for them.
fn ambiguous_keys(entities: &[CodeEntity]) -> std::collections::HashSet<String> {
    let mut key_files: HashMap<String, std::collections::HashSet<String>> = HashMap::new();
    for e in entities {
        key_files
            .entry(identity_key(e))
            .or_default()
            .insert(e.file_path.clone());
    }
    key_files
        .into_iter()
        .filter(|(_, files)| files.len() > 1)
        .map(|(k, _)| k)
        .collect()
}

pub fn diff_entities(before: &[CodeEntity], after: &[CodeEntity]) -> Vec<EntityChange> {
    let mut changes = Vec::new();
    let before_map: HashMap<String, &CodeEntity> =
        before.iter().map(|e| (identity_key(e), e)).collect();
    let after_map: HashMap<String, &CodeEntity> =
        after.iter().map(|e| (identity_key(e), e)).collect();

    // Keys that appear in multiple files in either snapshot cannot be move-detected reliably.
    let ambiguous = {
        let mut a = ambiguous_keys(before);
        a.extend(ambiguous_keys(after));
        a
    };

    for (id, after_e) in &after_map {
        if let Some(before_e) = before_map.get(id) {
            if before_e.file_path != after_e.file_path && !ambiguous.contains(id) {
                changes.push(EntityChange {
                    kind: ChangeKind::Moved,
                    entity_key: after_e.unique_key(),
                    name: after_e.name.clone(),
                    file_path: after_e.file_path.clone(),
                    entity_kind: after_e.kind.as_str().to_string(),
                    before_signature: Some(before_e.signature.clone()),
                    after_signature: Some(after_e.signature.clone()),
                    before_source: Some(before_e.source.clone()),
                    after_source: Some(after_e.source.clone()),
                    old_file_path: Some(before_e.file_path.clone()),
                    old_name: None,
                });
            } else if before_e.signature != after_e.signature || before_e.source != after_e.source {
                changes.push(EntityChange {
                    kind: ChangeKind::Modified,
                    entity_key: after_e.unique_key(),
                    name: after_e.name.clone(),
                    file_path: after_e.file_path.clone(),
                    entity_kind: after_e.kind.as_str().to_string(),
                    before_signature: Some(before_e.signature.clone()),
                    after_signature: Some(after_e.signature.clone()),
                    before_source: Some(before_e.source.clone()),
                    after_source: Some(after_e.source.clone()),
                    old_file_path: None,
                    old_name: None,
                });
            }
        } else {
            changes.push(EntityChange {
                kind: ChangeKind::Added,
                entity_key: after_e.unique_key(),
                name: after_e.name.clone(),
                file_path: after_e.file_path.clone(),
                entity_kind: after_e.kind.as_str().to_string(),
                before_signature: None,
                after_signature: Some(after_e.signature.clone()),
                before_source: None,
                after_source: Some(after_e.source.clone()),
                old_file_path: None,
                old_name: None,
            });
        }
    }

    for (id, before_e) in &before_map {
        if !after_map.contains_key(id) {
            changes.push(EntityChange {
                kind: ChangeKind::Removed,
                entity_key: before_e.unique_key(),
                name: before_e.name.clone(),
                file_path: before_e.file_path.clone(),
                entity_kind: before_e.kind.as_str().to_string(),
                before_signature: Some(before_e.signature.clone()),
                after_signature: None,
                before_source: Some(before_e.source.clone()),
                after_source: None,
                old_file_path: None,
                old_name: None,
            });
        }
    }

    changes
}
