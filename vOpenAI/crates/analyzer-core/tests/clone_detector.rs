use analyzer_core::detectors::clone::{detect_clone_groups, detect_exact_clone_groups};
use analyzer_core::lang::Language;

#[test]
fn groups_typescript_identifier_renames_as_clones() {
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

    let groups = detect_clone_groups(&[analysis_a, analysis_b]);

    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].members.len(), 2);
}

#[test]
fn exact_clones_require_matching_lexical_tokens() {
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

    let exact_groups = detect_exact_clone_groups(&[analysis_a, analysis_b]);

    assert!(exact_groups.is_empty());
}
