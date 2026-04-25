use analyzer_core::architecture::component_id_from_path;

#[test]
fn infers_stable_component_ids_from_repository_paths() {
    assert_eq!(
        component_id_from_path(
            "framework/src/Volo.Abp.AutoMapper/Volo/Abp/AutoMapper/AutoMapperOptions.cs"
        ),
        "framework/src/Volo.Abp.AutoMapper"
    );
    assert_eq!(
        component_id_from_path("npm/ng-packs/packages/account/src/lib/resolvers/index.ts"),
        "npm/ng-packs/packages/account"
    );
    assert_eq!(
        component_id_from_path("templates/app-nolayers/aspnet-core/MyCompanyName.MyProjectName.Blazor.Server/Data/MyProjectNameDbContext.cs"),
        "templates/app-nolayers/aspnet-core/MyCompanyName.MyProjectName.Blazor.Server"
    );
    assert_eq!(
        component_id_from_path(
            "modules/cms-kit/angular/projects/dev-app/src/app/home/home.component.ts"
        ),
        "modules/cms-kit/angular/projects/dev-app"
    );
    assert_eq!(component_id_from_path("src/app.ts"), "src");
}
