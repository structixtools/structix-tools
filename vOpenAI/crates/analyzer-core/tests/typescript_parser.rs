use analyzer_core::ir::model::SymbolKind;
use analyzer_core::lang::Language;

#[test]
fn parses_exported_typescript_function_symbol() {
    let source = r#"
export function greet(name: string): string {
    return `Hello ${name}`;
}
"#;

    let analysis = Language::TypeScript
        .parse("src/example.ts", source)
        .expect("parse should succeed");

    assert_eq!(analysis.symbols.len(), 1);
    let symbol = &analysis.symbols[0];
    assert_eq!(symbol.name, "greet");
    assert_eq!(symbol.kind, SymbolKind::Function);
    assert!(symbol.exported);
}

#[test]
fn fingerprints_ignore_identifier_renames() {
    let source_a = r#"
export function greet(name: string): string {
    return name;
}
"#;
    let source_b = r#"
export function greet(person: string): string {
    return person;
}
"#;

    let analysis_a = Language::TypeScript
        .parse("src/a.ts", source_a)
        .expect("parse should succeed");
    let analysis_b = Language::TypeScript
        .parse("src/b.ts", source_b)
        .expect("parse should succeed");

    assert!(analysis_a.lexical_hash.is_some());
    assert!(analysis_a.token_hash.is_some());
    assert!(analysis_a.ast_hash.is_some());
    assert_ne!(analysis_a.lexical_hash, analysis_b.lexical_hash);
    assert_eq!(analysis_a.token_hash, analysis_b.token_hash);
    assert_eq!(analysis_a.ast_hash, analysis_b.ast_hash);
}
