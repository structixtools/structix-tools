use analyzer_core::lang::Language;
use analyzer_core::report::{build_report_with_renames, RenamedFile};

#[test]
fn report_counts_renames_without_fake_add_remove_noise() {
    let base_source = r#"
export function greet(name: string): string {
    return name;
}
"#;
    let head_source = r#"
export function greet(name: string): string {
    return name;
}
"#;

    let base = Language::TypeScript
        .parse("src/old.ts", base_source)
        .expect("parse should succeed");
    let head = Language::TypeScript
        .parse("src/new.ts", head_source)
        .expect("parse should succeed");

    let report = build_report_with_renames(
        &[base],
        &[head],
        &[RenamedFile::new("src/old.ts", "src/new.ts")],
    );

    assert!(report.summary.contains("1 renamed file"));
    assert!(report
        .markdown()
        .contains("Renamed `src/old.ts` -> `src/new.ts`"));
    assert!(!report.summary.contains("added file"));
    assert!(!report.summary.contains("removed file"));
}
