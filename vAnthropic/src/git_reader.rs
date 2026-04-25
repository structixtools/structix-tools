use std::collections::HashMap;
use std::process::Command;

const SUPPORTED_EXTENSIONS: &[&str] = &[".ts", ".tsx", ".cs"];

/// Return {relative_file_path: source_code} for all supported files at the given git ref.
pub fn get_files_at_ref(
    repo_path: &str,
    ref_name: &str,
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
