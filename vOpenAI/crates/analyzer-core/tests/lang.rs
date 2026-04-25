use analyzer_core::lang::Language;

#[test]
fn detects_csharp_and_typescript_by_extension() {
    assert_eq!(Language::from_path("src/app.cs"), Some(Language::CSharp));
    assert_eq!(
        Language::from_path("src/app.ts"),
        Some(Language::TypeScript)
    );
    assert_eq!(
        Language::from_path("src/app.tsx"),
        Some(Language::TypeScript)
    );
    assert_eq!(Language::from_path("src/app.txt"), None);
}
