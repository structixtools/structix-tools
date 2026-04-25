use analyzer_core::input::filter::path_matches_filters;

#[test]
fn matches_all_when_filters_are_empty() {
    assert!(path_matches_filters("samples/tetris-demo/frontend/src/main.ts", &[]));
}

#[test]
fn matches_prefixes_on_directory_boundaries() {
    let filters = vec!["samples/tetris-demo".to_string()];
    assert!(path_matches_filters("samples/tetris-demo/frontend/src/main.ts", &filters));
    assert!(path_matches_filters("samples/tetris-demo", &filters));
    assert!(!path_matches_filters("samples/tetris-demo-2/frontend/src/main.ts", &filters));
    assert!(!path_matches_filters("README.md", &filters));
}

#[test]
fn normalizes_filters() {
    let filters = vec!["./samples\\tetris-demo".to_string()];
    assert!(path_matches_filters("samples/tetris-demo/backend/TetrisDemo.Api/Program.cs", &filters));
}
