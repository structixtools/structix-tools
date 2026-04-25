pub fn is_ignored_path(path: &str) -> bool {
    let normalized = path.replace('\\', "/").to_ascii_lowercase();

    normalized.split('/').any(|segment| {
        matches!(
            segment,
            "bin" | "obj" | "dist" | "node_modules" | "generated" | "vendor"
        ) || segment.ends_with(".generated.cs")
            || segment.ends_with(".g.cs")
    })
}
