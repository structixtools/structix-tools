use analyzer_core::detectors::clone::detect_clone_drifts;
use analyzer_core::lang::Language;

#[test]
fn detects_when_one_copy_in_a_clone_group_changes() {
    let base_a = r#"
export function greet(name: string): string {
    return name;
}
"#;
    let base_b = r#"
export function greet(person: string): string {
    return person;
}
"#;
    let head_a = r#"
export function greet(name: string): string {
    return `Hello ${name}`;
}
"#;
    let head_b = base_b;

    let base = vec![
        Language::TypeScript
            .parse("src/a.ts", base_a)
            .expect("parse should succeed"),
        Language::TypeScript
            .parse("src/b.ts", base_b)
            .expect("parse should succeed"),
    ];
    let head = vec![
        Language::TypeScript
            .parse("src/a.ts", head_a)
            .expect("parse should succeed"),
        Language::TypeScript
            .parse("src/b.ts", head_b)
            .expect("parse should succeed"),
    ];

    let drifts = detect_clone_drifts(&base, &head);

    assert_eq!(drifts.len(), 1);
    assert_eq!(drifts[0].changed_members, vec!["src/a.ts"]);
    assert_eq!(drifts[0].members, vec!["src/a.ts", "src/b.ts"]);
}
