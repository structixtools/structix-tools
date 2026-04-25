use analyzer_core::input::snapshot::{ChangeKind, ChangedFile, Snapshot};

#[test]
fn snapshot_filters_ignored_paths() {
    let snapshot = Snapshot::new(
        "base",
        "head",
        vec![
            ChangedFile::modified("src/app.ts"),
            ChangedFile::modified("dist/generated.js"),
        ],
    );

    let interesting = snapshot.interesting_changes();

    assert_eq!(interesting.len(), 1);
    assert_eq!(interesting[0].path, "src/app.ts");
    assert_eq!(interesting[0].kind, ChangeKind::Modified);
}
