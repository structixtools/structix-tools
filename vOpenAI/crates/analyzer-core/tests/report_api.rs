use analyzer_core::lang::Language;
use analyzer_core::report::build_report;

#[test]
fn report_scores_removed_apis_higher_than_changed_apis() {
    let base_source = r#"
export function greet(name: string): string {
    return name;
}

export function farewell(name: string): string {
    return name;
}
"#;
    let head_source = r#"
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

    let report = build_report(&[base], &[head]);

    let changed = report
        .findings
        .iter()
        .find(|finding| finding.code == "api.changed")
        .expect("changed api finding should exist");
    let removed = report
        .findings
        .iter()
        .find(|finding| finding.code == "api.removed")
        .expect("removed api finding should exist");

    assert!(removed.risk_score > changed.risk_score);
    assert!(removed.risk_score >= 0.9);
    assert!(changed.risk_score >= 0.7);
    assert!(report.markdown().contains("## API Changes"));
}
