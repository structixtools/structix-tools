use structix::git_reader::path_matches_filters;

#[test]
fn matches_when_no_filters_are_provided() {
    assert!(path_matches_filters("samples/tetris-demo/frontend/src/main.ts", &[]));
}

#[test]
fn matches_exact_path_or_descendants() {
    let filters = vec!["samples/tetris-demo".to_string()];
    assert!(path_matches_filters("samples/tetris-demo/frontend/src/main.ts", &filters));
    assert!(path_matches_filters("samples/tetris-demo", &filters));
    assert!(!path_matches_filters("samples/tetris-demo-2/src/main.ts", &filters));
    assert!(!path_matches_filters("site/index.html", &filters));
}

#[test]
fn normalizes_dot_prefixes_and_backslashes() {
    let filters = vec!["./samples\\tetris-demo".to_string()];
    assert!(path_matches_filters("samples/tetris-demo/backend/Program.cs", &filters));
}
