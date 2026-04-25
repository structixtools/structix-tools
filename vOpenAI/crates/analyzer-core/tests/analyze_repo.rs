use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use analyzer_core::analysis::analyze_repo;

#[test]
fn analyzes_a_git_repo_and_explains_the_changes() {
    let repo = create_repo();

    write_file(
        &repo.join("src/app.ts"),
        r#"
export function greet(name: string): string {
    return name;
}
"#,
    );
    write_file(
        &repo.join("src/copy.ts"),
        r#"
export function greet(name: string): string {
    return `Hello ${name}`;
}
"#,
    );
    git(&repo, ["add", "."]);
    git(&repo, ["commit", "-m", "base"]);

    write_file(
        &repo.join("src/app.ts"),
        r#"
export function greet(name: string): string {
    return `Hello ${name}`;
}
"#,
    );
    git(&repo, ["add", "src/app.ts"]);
    git(&repo, ["commit", "-m", "update app"]);

    let report = analyze_repo(&repo, "HEAD~1", "HEAD").expect("analysis should succeed");

    assert!(report.summary.contains("1 modified file"));
    assert!(report.summary.contains("1 exact clone group"));
    assert!(report.markdown().contains("Modified `src/app.ts`"));
    assert!(report.markdown().contains("symbols: greet"));
}

#[test]
fn analyzes_clone_drift_across_revisions() {
    let repo = create_repo();

    write_file(
        &repo.join("src/a.ts"),
        r#"
export function greet(name: string): string {
    return name;
}
"#,
    );
    write_file(
        &repo.join("src/b.ts"),
        r#"
export function greet(person: string): string {
    return person;
}
"#,
    );
    git(&repo, ["add", "."]);
    git(&repo, ["commit", "-m", "base"]);

    write_file(
        &repo.join("src/a.ts"),
        r#"
export function greet(name: string): string {
    return `Hello ${name}`;
}
"#,
    );
    git(&repo, ["add", "src/a.ts"]);
    git(&repo, ["commit", "-m", "update a"]);

    let report = analyze_repo(&repo, "HEAD~1", "HEAD").expect("analysis should succeed");

    assert!(report.summary.contains("clone drift group"));
    assert!(report.markdown().contains("Clone drift detected"));
}

#[test]
fn analyzes_repo_even_when_one_csharp_file_has_recovery_errors() {
    let repo = create_repo();

    write_file(
        &repo.join("src/app.ts"),
        r#"
export function greet(name: string): string {
    return name;
}
"#,
    );
    write_file(
        &repo.join("src/Broken.cs"),
        r#"
public class Broken
{
    public void M(
    {
    }
}
"#,
    );
    git(&repo, ["add", "."]);
    git(&repo, ["commit", "-m", "base"]);

    write_file(
        &repo.join("src/app.ts"),
        r#"
export function greet(name: string): string {
    return `Hello ${name}`;
}
"#,
    );
    git(&repo, ["add", "src/app.ts"]);
    git(&repo, ["commit", "-m", "update ts"]);

    let report = analyze_repo(&repo, "HEAD~1", "HEAD").expect("analysis should succeed");

    assert!(report.summary.contains("1 modified file"));
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.code == "file.modified"));
}

fn create_repo() -> PathBuf {
    let mut repo = std::env::temp_dir();
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be monotonic")
        .as_nanos();
    repo.push(format!("analyzer-core-test-{suffix}"));
    fs::create_dir_all(repo.join("src")).expect("create repo directory");

    git(&repo, ["init"]);
    git(&repo, ["config", "user.email", "test@example.com"]);
    git(&repo, ["config", "user.name", "Test User"]);

    repo
}

fn write_file(path: &Path, contents: &str) {
    fs::write(path, contents).expect("write test file");
}

fn git<const N: usize>(repo: &Path, args: [&str; N]) {
    let output = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git command");

    assert!(
        output.status.success(),
        "git command failed: {}\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}
