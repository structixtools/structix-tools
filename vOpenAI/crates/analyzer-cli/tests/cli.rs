use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use analyzer_cli::run;

#[test]
fn renders_report_for_a_repo() {
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

    let output = run([
        "--repo",
        repo.to_str().expect("repo path should be UTF-8"),
        "--base",
        "HEAD~1",
        "--head",
        "HEAD",
    ])
    .expect("cli run should succeed");

    assert!(output.contains("Change Report"));
    assert!(output.contains("1 modified file"));
    assert!(output.contains("1 exact clone group"));
    assert!(output.contains("Modified symbol `greet`"));
    assert!(output.contains("## API Changes"));
}

#[test]
fn renders_json_report_for_a_repo() {
    let repo = create_repo();

    write_file(
        &repo.join("src/app.ts"),
        r#"
export function greet(name: string): string {
    return name;
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

    let output = run([
        "--repo",
        repo.to_str().expect("repo path should be UTF-8"),
        "--base",
        "HEAD~1",
        "--head",
        "HEAD",
        "--format",
        "json",
    ])
    .expect("cli run should succeed");

    let trimmed = output.trim_start();
    assert!(trimmed.starts_with('{'));
    assert!(output.contains("\"summary\""));
    assert!(output.contains("\"findings\""));
}

#[test]
fn renders_pr_comment_for_a_repo() {
    let repo = create_repo();

    write_file(
        &repo.join("src/app.ts"),
        r#"
export function greet(name: string): string {
    return name;
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

    let output = run([
        "--repo",
        repo.to_str().expect("repo path should be UTF-8"),
        "--base",
        "HEAD~1",
        "--head",
        "HEAD",
        "--format",
        "pr-comment",
    ])
    .expect("cli run should succeed");

    assert!(output.contains("## Risk"));
    assert!(output.contains("## API Changes"));
    assert!(output.contains("risk"));
}

#[test]
fn path_filters_limit_cli_analysis_scope() {
    let repo = create_repo();
    fs::create_dir_all(repo.join("samples/demo")).expect("create sample directory");
    fs::create_dir_all(repo.join("site")).expect("create site directory");

    write_file(&repo.join("samples/demo/app.ts"), "export function demo(): string { return 'a'; }");
    write_file(&repo.join("site/index.ts"), "export function site(): string { return 'a'; }");
    git(&repo, ["add", "."]);
    git(&repo, ["commit", "-m", "base"]);

    write_file(&repo.join("samples/demo/app.ts"), "export function demo(): string { return 'b'; }");
    write_file(&repo.join("site/index.ts"), "export function site(): string { return 'b'; }");
    git(&repo, ["add", "."]);
    git(&repo, ["commit", "-m", "head"]);

    let output = run([
        "--repo",
        repo.to_str().expect("repo path should be UTF-8"),
        "--base",
        "HEAD~1",
        "--head",
        "HEAD",
        "--path",
        "samples/demo",
    ])
    .expect("cli run should succeed");

    assert!(output.contains("samples/demo/app.ts"));
    assert!(!output.contains("site/index.ts"));
}

#[test]
fn renders_html_report_for_a_repo() {
    let repo = create_repo();

    write_file(
        &repo.join("src/app.ts"),
        r#"
export function greet(name: string): string {
    return name;
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

    let output = run([
        "--repo",
        repo.to_str().expect("repo path should be UTF-8"),
        "--base",
        "HEAD~1",
        "--head",
        "HEAD",
        "--format",
        "html",
    ])
    .expect("cli run should succeed");

    assert!(output.contains("<!DOCTYPE html>"));
    assert!(output.contains("analysis-data"));
    assert!(output.contains("component-card"));
}

fn create_repo() -> PathBuf {
    let mut repo = std::env::temp_dir();
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be monotonic")
        .as_nanos();
    repo.push(format!("analyzer-cli-test-{suffix}"));
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
