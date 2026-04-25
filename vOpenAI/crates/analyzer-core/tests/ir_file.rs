use analyzer_core::ir::file::FileAnalysis;
use analyzer_core::ir::file::FileMetrics;
use analyzer_core::lang::Language;

#[test]
fn empty_file_analysis_starts_clean() {
    let analysis = FileAnalysis::new("src/app.ts", Language::TypeScript);

    assert_eq!(analysis.path, "src/app.ts");
    assert_eq!(analysis.language, Language::TypeScript);
    assert_eq!(analysis.metrics.code_lines, 0);
    assert!(analysis.symbols.is_empty());
    assert!(analysis.references.is_empty());
}

#[test]
fn counts_code_comment_and_blank_lines() {
    let source = "let x = 1;\n// hello\n\n/* block */\nlet y = 2;";
    let metrics = FileMetrics::from_source(source);

    assert_eq!(metrics.code_lines, 2);
    assert_eq!(metrics.comment_lines, 2);
    assert_eq!(metrics.blank_lines, 1);
}
