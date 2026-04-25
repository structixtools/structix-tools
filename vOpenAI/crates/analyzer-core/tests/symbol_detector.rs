use analyzer_core::detectors::symbol::detect_symbol_changes;
use analyzer_core::ir::model::SymbolChangeKind;
use analyzer_core::lang::Language;

#[test]
fn detects_modified_symbol_body() {
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

    let changes = detect_symbol_changes(&[base], &[head]);

    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].kind, SymbolChangeKind::Modified);
    assert_eq!(changes[0].name, "greet");
    assert_eq!(changes[0].file, "src/app.ts");
}
