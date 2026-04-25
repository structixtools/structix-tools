use structix::parser::types::EntityKind;
use structix::parser::typescript::TypeScriptParser;

const SAMPLE_TS: &str = r#"
export interface User {
  id: number;
  name: string;
}

export async function getUser(id: number): Promise<User> {
  return fetch(`/api/users/${id}`).then(r => r.json());
}

export class UserService {
  private baseUrl: string;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl;
  }

  async findAll(): Promise<User[]> {
    return [];
  }
}
"#;

#[test]
fn ts_extracts_interface() {
    let parser = TypeScriptParser::new();
    let entities = parser.extract("test.ts", SAMPLE_TS);
    let interfaces: Vec<_> = entities.iter().filter(|e| e.kind == EntityKind::Interface).collect();
    assert_eq!(interfaces.len(), 1, "expected 1 interface, got: {:?}", interfaces.iter().map(|e| &e.name).collect::<Vec<_>>());
    assert_eq!(interfaces[0].name, "User");
}

#[test]
fn ts_extracts_function() {
    let parser = TypeScriptParser::new();
    let entities = parser.extract("test.ts", SAMPLE_TS);
    let funcs: Vec<_> = entities.iter().filter(|e| e.kind == EntityKind::Function).collect();
    assert!(funcs.iter().any(|e| e.name == "getUser"), "getUser not found, functions: {:?}", funcs.iter().map(|e| &e.name).collect::<Vec<_>>());
}

#[test]
fn ts_function_has_async_and_export_qualifiers() {
    let parser = TypeScriptParser::new();
    let entities = parser.extract("test.ts", SAMPLE_TS);
    let get_user = entities.iter().find(|e| e.name == "getUser" && e.kind == EntityKind::Function)
        .expect("getUser function not found");
    assert!(get_user.qualifiers.contains(&"async".to_string()), "missing async, got: {:?}", get_user.qualifiers);
    assert!(get_user.qualifiers.contains(&"export".to_string()), "missing export, got: {:?}", get_user.qualifiers);
}

#[test]
fn ts_extracts_class() {
    let parser = TypeScriptParser::new();
    let entities = parser.extract("test.ts", SAMPLE_TS);
    let classes: Vec<_> = entities.iter().filter(|e| e.kind == EntityKind::Class).collect();
    assert!(classes.iter().any(|e| e.name == "UserService"), "UserService not found");
}

#[test]
fn ts_extracts_method_with_parent() {
    let parser = TypeScriptParser::new();
    let entities = parser.extract("test.ts", SAMPLE_TS);
    let find_all = entities.iter()
        .find(|e| e.name == "findAll" && e.kind == EntityKind::Method)
        .expect("findAll method not found");
    assert_eq!(find_all.parent.as_deref(), Some("UserService"));
}
