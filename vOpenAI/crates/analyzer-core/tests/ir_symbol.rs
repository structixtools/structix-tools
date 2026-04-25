use analyzer_core::ir::model::{SourceSpan, Symbol, SymbolKind};

#[test]
fn symbol_builds_qualified_name() {
    let span = SourceSpan::new("src/service.ts", 10, 1, 14, 2);
    let symbol = Symbol::new("sym-1", "Run", SymbolKind::Method, "src/service.ts", span)
        .with_container("MyApp.Service")
        .exported(true);

    assert_eq!(symbol.qualified_name(), "MyApp.Service.Run");
    assert!(symbol.exported);
}
