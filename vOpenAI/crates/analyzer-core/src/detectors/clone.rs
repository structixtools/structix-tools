use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::ir::file::FileAnalysis;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CloneGroup {
    pub fingerprint: u64,
    pub members: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CloneDrift {
    pub fingerprint: u64,
    pub members: Vec<String>,
    pub changed_members: Vec<String>,
}

pub fn detect_clone_groups(files: &[FileAnalysis]) -> Vec<CloneGroup> {
    let mut groups: BTreeMap<u64, Vec<String>> = BTreeMap::new();

    for file in files {
        let Some(fingerprint) = file.token_hash else {
            continue;
        };

        groups
            .entry(fingerprint)
            .or_default()
            .push(file.path.clone());
    }

    groups
        .into_iter()
        .filter(|(_, members)| members.len() > 1)
        .map(|(fingerprint, members)| CloneGroup {
            fingerprint,
            members,
        })
        .collect()
}

pub fn detect_exact_clone_groups(files: &[FileAnalysis]) -> Vec<CloneGroup> {
    let mut groups: BTreeMap<u64, Vec<String>> = BTreeMap::new();

    for file in files {
        let Some(fingerprint) = file.lexical_hash else {
            continue;
        };

        groups
            .entry(fingerprint)
            .or_default()
            .push(file.path.clone());
    }

    groups
        .into_iter()
        .filter(|(_, members)| members.len() > 1)
        .map(|(fingerprint, members)| CloneGroup {
            fingerprint,
            members,
        })
        .collect()
}

pub fn detect_clone_drifts(base: &[FileAnalysis], head: &[FileAnalysis]) -> Vec<CloneDrift> {
    let head_index: BTreeMap<&str, &FileAnalysis> =
        head.iter().map(|file| (file.path.as_str(), file)).collect();

    detect_clone_groups(base)
        .into_iter()
        .filter_map(|group| {
            let head_hashes: BTreeSet<Option<u64>> = group
                .members
                .iter()
                .map(|member| {
                    head_index
                        .get(member.as_str())
                        .and_then(|file| file.token_hash)
                })
                .collect();

            if head_hashes.len() <= 1 {
                return None;
            }

            let changed_members = group
                .members
                .iter()
                .filter(|member| {
                    head_index
                        .get(member.as_str())
                        .and_then(|file| file.token_hash)
                        != Some(group.fingerprint)
                })
                .cloned()
                .collect::<Vec<_>>();

            if changed_members.is_empty() {
                return None;
            }

            Some(CloneDrift {
                fingerprint: group.fingerprint,
                members: group.members,
                changed_members,
            })
        })
        .collect()
}
