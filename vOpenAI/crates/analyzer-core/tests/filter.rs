use analyzer_core::input::filter::is_ignored_path;

#[test]
fn ignores_generated_and_vendor_paths() {
    assert!(is_ignored_path("src/obj/Debug/foo.cs"));
    assert!(is_ignored_path("web/node_modules/react/index.js"));
    assert!(is_ignored_path("dist/app.js"));
    assert!(is_ignored_path("src/Models/foo.generated.cs"));
    assert!(!is_ignored_path("src/app.ts"));
}
