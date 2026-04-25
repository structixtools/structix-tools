use analyzer_core::ir::model::SymbolKind;
use analyzer_core::lang::Language;

#[test]
fn parses_public_csharp_class_and_method() {
    let source = r#"
public class Greeter
{
    public string Greet(string name)
    {
        return $"Hello {name}";
    }
}
"#;

    let analysis = Language::CSharp
        .parse("src/Greeter.cs", source)
        .expect("parse should succeed");

    assert!(analysis.symbols.iter().any(|symbol| {
        symbol.name == "Greeter" && symbol.kind == SymbolKind::Class && symbol.exported
    }));

    assert!(analysis.symbols.iter().any(|symbol| {
        symbol.name == "Greet" && symbol.kind == SymbolKind::Method && symbol.exported
    }));
}
