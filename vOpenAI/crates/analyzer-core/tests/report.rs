use analyzer_core::ir::model::Finding;
use analyzer_core::lang::Language;
use analyzer_core::report::build_report;

#[test]
fn report_summarizes_modified_files_and_clone_groups() {
    let base_source = r#"
export function greet(name: string): string {
    return name;
}
"#;
    let head_source = r#"
export function greet(name: string): string {
    return `Hello ${name}`;
}
"#;
    let clone_source = r#"
export function greet(name: string): string {
    return `Hello ${name}`;
}
"#;

    let base = Language::TypeScript
        .parse("src/app.ts", base_source)
        .expect("parse should succeed");
    let head = Language::TypeScript
        .parse("src/app.ts", head_source)
        .expect("parse should succeed");
    let clone = Language::TypeScript
        .parse("src/copy.ts", clone_source)
        .expect("parse should succeed");

    let report = build_report(&[base], &[head, clone]);

    assert!(report.summary.contains("1 modified file"));
    assert!(report.summary.contains("1 exact clone group"));
    assert!(report
        .findings
        .iter()
        .any(|finding: &Finding| finding.code == "file.modified"));
    assert!(report
        .findings
        .iter()
        .any(|finding: &Finding| finding.code == "clone.exact"));
    assert!(report.markdown().contains("Modified `src/app.ts`"));
    assert!(report.markdown().contains("symbols: greet"));
}
