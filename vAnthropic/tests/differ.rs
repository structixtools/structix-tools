use structix::parser::types::{CodeEntity, EntityKind};
use structix::differ::entity_diff::diff_entities;
use structix::manifest::schema::ChangeKind;

fn make_entity(name: &str, sig: &str, source: &str, file_path: &str) -> CodeEntity {
    CodeEntity {
        kind: EntityKind::Function,
        name: name.to_string(),
        file_path: file_path.to_string(),
        start_line: 1,
        end_line: 5,
        source: source.to_string(),
        signature: sig.to_string(),
        qualifiers: vec![],
        parent: None,
    }
}

#[test]
fn detects_added_entity() {
    let before = vec![make_entity("foo", "foo()", "fn foo() {}", "a.ts")];
    let after = vec![
        make_entity("foo", "foo()", "fn foo() {}", "a.ts"),
        make_entity("bar", "bar()", "fn bar() {}", "a.ts"),
    ];
    let changes = diff_entities(&before, &after);
    assert!(changes.iter().any(|c| c.kind == ChangeKind::Added && c.name == "bar"),
        "bar not added, got: {:?}", changes.iter().map(|c| (&c.name, &c.kind)).collect::<Vec<_>>());
}

#[test]
fn detects_removed_entity() {
    let before = vec![
        make_entity("foo", "foo()", "fn foo() {}", "a.ts"),
        make_entity("bar", "bar()", "fn bar() {}", "a.ts"),
    ];
    let after = vec![make_entity("foo", "foo()", "fn foo() {}", "a.ts")];
    let changes = diff_entities(&before, &after);
    assert!(changes.iter().any(|c| c.kind == ChangeKind::Removed && c.name == "bar"));
}

#[test]
fn detects_modified_signature() {
    let before = vec![make_entity("foo", "foo(a: string)", "fn foo(a: string) {}", "a.ts")];
    let after = vec![make_entity("foo", "foo(a: string, b: number)", "fn foo(a: string, b: number) {}", "a.ts")];
    let changes = diff_entities(&before, &after);
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].kind, ChangeKind::Modified);
    assert_eq!(changes[0].before_signature.as_deref(), Some("foo(a: string)"));
    assert_eq!(changes[0].after_signature.as_deref(), Some("foo(a: string, b: number)"));
}

#[test]
fn no_change_produces_empty_diff() {
    let e = make_entity("foo", "foo()", "fn foo() {}", "a.ts");
    let changes = diff_entities(&[e.clone()], &[e]);
    assert!(changes.is_empty());
}

#[test]
fn detects_moved_entity() {
    let before = vec![make_entity("foo", "foo()", "fn foo() {}", "a.ts")];
    let after = vec![make_entity("foo", "foo()", "fn foo() {}", "b.ts")];
    let changes = diff_entities(&before, &after);
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].kind, ChangeKind::Moved);
    assert_eq!(changes[0].old_file_path.as_deref(), Some("a.ts"));
    assert_eq!(changes[0].file_path, "b.ts");
}
