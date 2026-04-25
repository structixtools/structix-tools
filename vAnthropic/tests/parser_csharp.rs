use structix::parser::types::EntityKind;
use structix::parser::csharp::CSharpParser;

const SAMPLE_CS: &str = r#"
namespace MyApp.Services;

public interface IUserService
{
    Task<User> GetUserAsync(int id);
}

public class UserService : IUserService
{
    private readonly HttpClient _http;

    public UserService(HttpClient http)
    {
        _http = http;
    }

    public async Task<User> GetUserAsync(int id)
    {
        return await _http.GetFromJsonAsync<User>($"/api/users/{id}");
    }

    private string BuildUrl(int id) => $"/api/users/{id}";
}
"#;

#[test]
fn cs_extracts_interface() {
    let parser = CSharpParser::new();
    let entities = parser.extract("UserService.cs", SAMPLE_CS);
    let ifaces: Vec<_> = entities.iter().filter(|e| e.kind == EntityKind::Interface).collect();
    assert_eq!(ifaces.len(), 1, "expected 1 interface, got {:?}", ifaces.iter().map(|e| &e.name).collect::<Vec<_>>());
    assert_eq!(ifaces[0].name, "IUserService");
}

#[test]
fn cs_extracts_class() {
    let parser = CSharpParser::new();
    let entities = parser.extract("UserService.cs", SAMPLE_CS);
    let classes: Vec<_> = entities.iter().filter(|e| e.kind == EntityKind::Class).collect();
    assert!(classes.iter().any(|e| e.name == "UserService"), "UserService not found");
}

#[test]
fn cs_extracts_methods_with_parent() {
    let parser = CSharpParser::new();
    let entities = parser.extract("UserService.cs", SAMPLE_CS);
    let method = entities.iter()
        .find(|e| e.kind == EntityKind::Method && e.name == "GetUserAsync")
        .expect("GetUserAsync not found");
    assert_eq!(method.parent.as_deref(), Some("UserService"));
}

#[test]
fn cs_async_qualifier() {
    let parser = CSharpParser::new();
    let entities = parser.extract("UserService.cs", SAMPLE_CS);
    let method = entities.iter()
        .find(|e| e.kind == EntityKind::Method && e.name == "GetUserAsync")
        .expect("GetUserAsync not found");
    assert!(method.qualifiers.contains(&"async".to_string()), "missing async, got: {:?}", method.qualifiers);
}

#[test]
fn cs_extracts_constructor() {
    let parser = CSharpParser::new();
    let entities = parser.extract("UserService.cs", SAMPLE_CS);
    let ctor = entities.iter().find(|e| e.kind == EntityKind::Constructor);
    assert!(ctor.is_some(), "no constructor found");
    assert_eq!(ctor.unwrap().parent.as_deref(), Some("UserService"));
}
