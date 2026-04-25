use analyzer_core::lang::Language;
use analyzer_core::report::build_report;

#[test]
fn report_includes_symbol_change_section() {
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

    let base = Language::TypeScript
        .parse("src/app.ts", base_source)
        .expect("parse should succeed");
    let head = Language::TypeScript
        .parse("src/app.ts", head_source)
        .expect("parse should succeed");

    let report = build_report(&[base], &[head]);

    assert_eq!(report.symbol_changes.len(), 1);
    assert_eq!(report.api_changes.len(), 1);
    assert!(report.summary.contains("1 symbol change"));
    assert!(report.summary.contains("1 api change"));
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.code == "symbol.modified"));
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.code == "api.changed"));
    assert!(report.markdown().contains("## Symbol Changes"));
    assert!(report.markdown().contains("## API Changes"));
    assert!(report.markdown().contains("Modified symbol `greet`"));
}
