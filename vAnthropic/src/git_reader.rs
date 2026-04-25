use std::collections::HashMap;
use std::process::Command;

const SUPPORTED_EXTENSIONS: &[&str] = &[".ts", ".tsx", ".cs"];

/// Return {relative_file_path: source_code} for all supported files at the given git ref.
pub fn get_files_at_ref(
    repo_path: &str,
    ref_name: &str,
    path_filters: &[String],
) -> anyhow::Result<HashMap<String, String>> {
    // List all files at ref
    let ls = Command::new("git")
        .args(["ls-tree", "-r", "--name-only", ref_name])
        .current_dir(repo_path)
        .output()?;

    if !ls.status.success() {
        anyhow::bail!(
            "git ls-tree failed: {}",
            String::from_utf8_lossy(&ls.stderr)
        );
    }

    let file_list = String::from_utf8_lossy(&ls.stdout);
    let mut files = HashMap::new();

    for path in file_list.lines() {
        if !SUPPORTED_EXTENSIONS.iter().any(|ext| path.ends_with(ext)) {
            continue;
        }
        if !path_matches_filters(path, path_filters) {
            continue;
        }
        let show = Command::new("git")
            .args(["show", &format!("{}:{}", ref_name, path)])
            .current_dir(repo_path)
            .output()?;

        if show.status.success() {
            if let Ok(content) = String::from_utf8(show.stdout) {
                files.insert(path.to_string(), content);
            }
        }
    }

    Ok(files)
}

pub fn path_matches_filters(path: &str, filters: &[String]) -> bool {
    if filters.is_empty() {
        return true;
    }

    let normalized_path = normalize_path(path);
    filters.iter().any(|filter| {
        let normalized_filter = normalize_path(filter);
        normalized_path == normalized_filter
            || normalized_path
                .strip_prefix(&normalized_filter)
                .is_some_and(|rest| rest.starts_with('/'))
    })
}

fn normalize_path(path: &str) -> String {
    let mut normalized = path.replace('\\', "/");
    while let Some(stripped) = normalized.strip_prefix("./") {
        normalized = stripped.to_string();
    }
    normalized.trim_matches('/').to_string()
}
