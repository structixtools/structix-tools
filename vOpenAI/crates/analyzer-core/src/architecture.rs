use serde::Serialize;

use crate::ir::file::FileAnalysis;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ComponentSummary {
    pub id: String,
    pub label: String,
    pub file_count: usize,
    pub files: Vec<String>,
}

pub fn component_id_from_path(path: &str) -> String {
    let normalized = path.replace('\\', "/");
    let segments: Vec<&str> = normalized
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect();

    if segments.is_empty() {
        return String::new();
    }

    if segments.first() == Some(&"framework") && segments.get(1) == Some(&"src") {
        return prefix(&segments, 3);
    }

    if segments.first() == Some(&"npm")
        && segments.get(1) == Some(&"ng-packs")
        && segments.get(2) == Some(&"packages")
        && segments.len() >= 4
    {
        return prefix(&segments, 4);
    }

    if segments.first() == Some(&"templates") && segments.len() >= 4 {
        return prefix(&segments, 4);
    }

    if segments.first() == Some(&"modules") {
        if let Some(projects_idx) = segments.iter().position(|segment| *segment == "projects") {
            if projects_idx + 1 < segments.len() {
                return prefix(&segments, projects_idx + 2);
            }
        }

        if let Some(src_idx) = segments.iter().position(|segment| *segment == "src") {
            if src_idx >= 1 {
                return prefix(&segments, src_idx);
            }
        }
    }

    if let Some(src_idx) = segments.iter().position(|segment| *segment == "src") {
        if src_idx == 0 {
            return segments[0].to_string();
        }

        if src_idx + 1 < segments.len() {
            return prefix(&segments, src_idx + 2);
        }

        return prefix(&segments, src_idx);
    }

    segments[0].to_string()
}

pub fn component_summaries(files: &[FileAnalysis]) -> Vec<ComponentSummary> {
    let mut groups: std::collections::BTreeMap<String, Vec<String>> =
        std::collections::BTreeMap::new();

    for file in files {
        let id = component_id_from_path(&file.path);
        groups.entry(id).or_default().push(file.path.clone());
    }

    groups
        .into_iter()
        .map(|(id, mut files)| {
            files.sort();
            let label = id.rsplit('/').next().unwrap_or(id.as_str()).to_string();
            let file_count = files.len();

            ComponentSummary {
                id,
                label,
                file_count,
                files,
            }
        })
        .collect()
}

fn prefix(segments: &[&str], count: usize) -> String {
    segments
        .iter()
        .take(count)
        .copied()
        .collect::<Vec<_>>()
        .join("/")
}
