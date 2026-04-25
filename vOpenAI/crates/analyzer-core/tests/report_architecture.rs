use analyzer_core::ir::file::FileAnalysis;
use analyzer_core::lang::Language;
use analyzer_core::report::build_report;

#[test]
fn report_includes_base_and_head_component_snapshots() {
    let base = vec![FileAnalysis::new(
        "framework/src/Volo.Abp.AutoMapper/Volo/Abp/AutoMapper/AutoMapperOptions.cs",
        Language::CSharp,
    )];
    let head = vec![
        FileAnalysis::new(
            "framework/src/Volo.Abp.AutoMapper/Volo/Abp/AutoMapper/AutoMapperOptions.cs",
            Language::CSharp,
        ),
        FileAnalysis::new(
            "framework/src/Volo.Abp.AutoMapper/AutoMapper/AbpAutoMapperOptions.cs",
            Language::CSharp,
        ),
    ];

    let report = build_report(&base, &head);

    assert_eq!(report.base_files.len(), 1);
    assert_eq!(report.head_files.len(), 2);
    assert_eq!(report.base_components.len(), 1);
    assert_eq!(report.head_components.len(), 1);
    assert_eq!(
        report.head_components[0].id,
        "framework/src/Volo.Abp.AutoMapper"
    );
    assert_eq!(report.head_components[0].file_count, 2);
}
