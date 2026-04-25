use analyzer_core::lang::Language;
use analyzer_core::report::build_report;

#[test]
fn report_renders_json() {
    let source = r#"
export function greet(name: string): string {
    return name;
}
"#;

    let analysis = Language::TypeScript
        .parse("src/app.ts", source)
        .expect("parse should succeed");
    let report = build_report(&[], &[analysis]);

    let json = report.json().expect("json rendering should succeed");

    assert!(json.contains("\"summary\""));
    assert!(json.contains("\"findings\""));
    assert!(json.contains("\"symbol_changes\""));
    assert!(json.contains("\"api_changes\""));
    assert!(json.contains("\"risk_score\""));
}
