use analyzer_core::lang::Language;
use analyzer_core::report::build_report;

#[test]
fn renders_a_self_contained_html_viewer() {
    let source = r#"
export function greet(name: string): string {
    return name;
}
"#;

    let analysis = Language::TypeScript
        .parse("src/app.ts", source)
        .expect("parse should succeed");
    let files = vec![analysis.clone(), analysis];
    let report = build_report(&files, &files);

    let html = report.html();

    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("analysis-data"));
    assert!(html.contains("Compare"));
    assert!(html.contains("Timeline"));
    assert!(html.contains("component-card"));
}
