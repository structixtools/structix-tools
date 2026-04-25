use structix::parser::types::{CodeEntity, EntityKind};
use structix::duplicates::normalizer::normalize;
use structix::duplicates::hasher::find_exact_clones;
use structix::duplicates::fingerprint::find_structural_clones;

fn make(name: &str, source: &str) -> CodeEntity {
    CodeEntity {
        kind: EntityKind::Function,
        name: name.to_string(),
        file_path: format!("{}.ts", name),
        start_line: 1,
        end_line: 5,
        source: source.to_string(),
        signature: name.to_string(),
        qualifiers: vec![],
        parent: None,
    }
}

// --- normalizer ---

#[test]
fn normalizer_strips_line_comments() {
    let result = normalize("function foo() { // comment\n  return 1;\n}");
    assert!(!result.contains("//"), "line comment not stripped: {}", result);
}

#[test]
fn normalizer_strips_block_comments() {
    let result = normalize("/* header */\nfunction foo() { return 1; }");
    assert!(!result.contains("/*"), "block comment not stripped: {}", result);
}

#[test]
fn normalizer_collapses_whitespace() {
    // Both should normalize to the same string regardless of extra spaces
    let a = normalize("function   foo()   {   return   1;   }");
    let b = normalize("function foo() { return 1; }");
    assert_eq!(a, b);
}

#[test]
fn normalizer_is_deterministic() {
    let src = "function foo() { return 1; }";
    assert_eq!(normalize(src), normalize(src));
}

// --- exact clone hasher ---

#[test]
fn exact_clone_finds_identical_functions() {
    // Same source text (copy-paste) — only the entity name differs, source is identical
    let src = "function doWork() { const x = 1; return x + 2; }";
    let e1 = make("copy_a", src);
    let e2 = make("copy_b", src);
    let entities = [e1, e2];
    let groups = find_exact_clones(&entities);
    assert_eq!(groups.len(), 1, "expected 1 clone group");
    assert_eq!(groups[0].len(), 2);
}

#[test]
fn exact_clone_no_match_when_different() {
    let e1 = make("foo", "function foo() { return 1; }");
    let e2 = make("bar", "function bar() { return 2; }");
    let entities = [e1, e2];
    let groups = find_exact_clones(&entities);
    assert!(groups.is_empty());
}

#[test]
fn exact_clone_no_self_match() {
    let e = make("foo", "function foo() { return 1; }");
    let entities = [e];
    let groups = find_exact_clones(&entities);
    assert!(groups.is_empty());
}

// --- structural clone fingerprint ---

const FUNC_A: &str = "function getUser(id) { const x = fetch(id); return x.json(); }";
const FUNC_B: &str = "function getOrder(orderId) { const result = fetch(orderId); return result.json(); }";
const FUNC_C: &str = "function doThing(x) { if (x > 0) { return x * 2; } return 0; }";

#[test]
fn structural_clone_groups_similar_functions() {
    let e1 = make("getUser", FUNC_A);
    let e2 = make("getOrder", FUNC_B);
    let e3 = make("doThing", FUNC_C);
    let entities = [e1, e2, e3];
    let groups = find_structural_clones(&entities);
    let all_names: Vec<Vec<String>> = groups.iter()
        .map(|g| g.iter().map(|e| e.name.clone()).collect())
        .collect();
    let has_pair = all_names.iter().any(|g| {
        g.contains(&"getUser".to_string()) && g.contains(&"getOrder".to_string())
    });
    assert!(has_pair, "getUser/getOrder structural clone not found, groups: {:?}", all_names);
}

#[test]
fn structural_clone_excludes_exact_clones() {
    // Entities with literally identical source are exact clones — structural detector should skip them
    let src = "function doWork() { const x = 1; return x + 2; }";
    let e1 = make("copy_a", src);
    let e2 = make("copy_b", src);
    let entities = [e1, e2];
    let groups = find_structural_clones(&entities);
    assert!(groups.is_empty(), "exact clones should not appear in structural groups: {:?}",
        groups.iter().map(|g| g.iter().map(|e| &e.name).collect::<Vec<_>>()).collect::<Vec<_>>());
}
