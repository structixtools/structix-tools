use analyzer_core::architecture::component_summaries;
use analyzer_core::ir::file::FileAnalysis;
use analyzer_core::lang::Language;

#[test]
fn groups_files_into_stable_components() {
    let files = vec![
        FileAnalysis::new(
            "framework/src/Volo.Abp.AutoMapper/Volo/Abp/AutoMapper/AutoMapperOptions.cs",
            Language::CSharp,
        ),
        FileAnalysis::new(
            "framework/src/Volo.Abp.AutoMapper/AutoMapper/AbpAutoMapperOptions.cs",
            Language::CSharp,
        ),
        FileAnalysis::new(
            "npm/ng-packs/packages/account/src/lib/resolvers/index.ts",
            Language::TypeScript,
        ),
    ];

    let summaries = component_summaries(&files);

    assert_eq!(summaries.len(), 2);
    assert_eq!(summaries[0].id, "framework/src/Volo.Abp.AutoMapper");
    assert_eq!(summaries[0].file_count, 2);
    assert_eq!(summaries[1].id, "npm/ng-packs/packages/account");
    assert_eq!(summaries[1].file_count, 1);
}
