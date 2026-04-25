use structix::parser::types::{CodeEntity, EntityKind};

#[test]
fn code_entity_unique_key() {
    let e = CodeEntity {
        kind: EntityKind::Function,
        name: "getUser".to_string(),
        file_path: "src/api.ts".to_string(),
        start_line: 1,
        end_line: 10,
        source: "function getUser() {}".to_string(),
        signature: "getUser()".to_string(),
        qualifiers: vec!["export".to_string()],
        parent: None,
    };
    assert_eq!(e.unique_key(), "src/api.ts::function::::getUser");
}

#[test]
fn code_entity_unique_key_with_parent() {
    let e = CodeEntity {
        kind: EntityKind::Method,
        name: "findAll".to_string(),
        file_path: "src/service.ts".to_string(),
        start_line: 5,
        end_line: 10,
        source: "findAll() {}".to_string(),
        signature: "findAll()".to_string(),
        qualifiers: vec![],
        parent: Some("UserService".to_string()),
    };
    assert_eq!(e.unique_key(), "src/service.ts::method::UserService::findAll");
}

#[test]
fn entity_kind_as_str() {
    assert_eq!(EntityKind::Function.as_str(), "function");
    assert_eq!(EntityKind::Class.as_str(), "class");
    assert_eq!(EntityKind::Interface.as_str(), "interface");
    assert_eq!(EntityKind::Method.as_str(), "method");
    assert_eq!(EntityKind::Constructor.as_str(), "constructor");
}
