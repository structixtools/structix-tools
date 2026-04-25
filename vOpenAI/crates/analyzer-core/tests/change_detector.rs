use analyzer_core::detectors::change::diff_files;
use analyzer_core::lang::Language;

#[test]
fn detects_modified_files_by_lexical_hash() {
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

    let diff = diff_files(&[base], &[head]);

    assert_eq!(diff.modified.len(), 1);
    assert_eq!(diff.modified[0].path, "src/app.ts");
}
