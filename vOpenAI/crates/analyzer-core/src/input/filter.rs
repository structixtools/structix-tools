pub fn is_ignored_path(path: &str) -> bool {
    let normalized = normalize_path(path).to_ascii_lowercase();

    normalized.split('/').any(|segment| {
        matches!(
            segment,
            "bin" | "obj" | "dist" | "node_modules" | "generated" | "vendor"
        ) || segment.ends_with(".generated.cs")
            || segment.ends_with(".g.cs")
    })
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
