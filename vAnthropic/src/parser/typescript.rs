use tree_sitter::{Language, Node, Parser};
use crate::parser::types::{CodeEntity, EntityKind};

pub struct TypeScriptParser {
    language: Language,
}

impl TypeScriptParser {
    pub fn new() -> Self {
        Self {
            language: tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        }
    }

    pub fn extract(&self, file_path: &str, source: &str) -> Vec<CodeEntity> {
        let mut parser = Parser::new();
        parser.set_language(&self.language).expect("load ts language");
        let tree = parser.parse(source, None).expect("parse");
        let src = source.as_bytes();
        let mut entities = Vec::new();
        Self::walk(tree.root_node(), src, file_path, &mut entities, None, false);
        entities
    }

    fn node_text<'a>(node: Node, src: &'a [u8]) -> &'a str {
        std::str::from_utf8(&src[node.start_byte()..node.end_byte()]).unwrap_or("")
    }

    fn walk(
        node: Node,
        src: &[u8],
        file_path: &str,
        entities: &mut Vec<CodeEntity>,
        parent: Option<&str>,
        is_exported: bool,
    ) {
        match node.kind() {
            "export_statement" => {
                // Recurse into the exported declaration, propagating the export flag
                for i in 0..node.child_count() {
                    let child = node.child(i).unwrap();
                    if matches!(
                        child.kind(),
                        "function_declaration"
                            | "class_declaration"
                            | "interface_declaration"
                            | "enum_declaration"
                            | "lexical_declaration"
                            | "abstract_class_declaration"
                    ) {
                        Self::walk(child, src, file_path, entities, parent, true);
                        return;
                    }
                }
                for i in 0..node.child_count() {
                    Self::walk(node.child(i).unwrap(), src, file_path, entities, parent, is_exported);
                }
            }

            "interface_declaration" => {
                let name = node
                    .child_by_field_name("name")
                    .map(|n| Self::node_text(n, src).to_string())
                    .unwrap_or_default();
                let mut qualifiers = vec![];
                if is_exported {
                    qualifiers.push("export".to_string());
                }
                entities.push(CodeEntity {
                    kind: EntityKind::Interface,
                    name,
                    file_path: file_path.to_string(),
                    start_line: node.start_position().row + 1,
                    end_line: node.end_position().row + 1,
                    source: Self::node_text(node, src).to_string(),
                    signature: String::new(),
                    qualifiers,
                    parent: parent.map(|s| s.to_string()),
                });
                // Don't recurse into interface body
            }

            "function_declaration" => {
                let name = node
                    .child_by_field_name("name")
                    .map(|n| Self::node_text(n, src).to_string())
                    .unwrap_or_default();
                let params = node
                    .child_by_field_name("parameters")
                    .map(|n| Self::node_text(n, src).to_string())
                    .unwrap_or_default();
                let ret = node
                    .child_by_field_name("return_type")
                    .map(|n| Self::node_text(n, src).to_string())
                    .unwrap_or_default();
                let sig = format!("{}{}{}", name, params, ret);

                let mut qualifiers = vec![];
                if is_exported {
                    qualifiers.push("export".to_string());
                }
                for i in 0..node.child_count() {
                    if node.child(i).unwrap().kind() == "async" {
                        qualifiers.push("async".to_string());
                    }
                }

                entities.push(CodeEntity {
                    kind: EntityKind::Function,
                    name,
                    file_path: file_path.to_string(),
                    start_line: node.start_position().row + 1,
                    end_line: node.end_position().row + 1,
                    source: Self::node_text(node, src).to_string(),
                    signature: sig,
                    qualifiers,
                    parent: parent.map(|s| s.to_string()),
                });
            }

            "class_declaration" | "abstract_class_declaration" => {
                let name = node
                    .child_by_field_name("name")
                    .map(|n| Self::node_text(n, src).to_string())
                    .unwrap_or_default();
                let mut qualifiers = vec![];
                if is_exported {
                    qualifiers.push("export".to_string());
                }
                if node.kind() == "abstract_class_declaration" {
                    qualifiers.push("abstract".to_string());
                }

                entities.push(CodeEntity {
                    kind: EntityKind::Class,
                    name: name.clone(),
                    file_path: file_path.to_string(),
                    start_line: node.start_position().row + 1,
                    end_line: node.end_position().row + 1,
                    source: Self::node_text(node, src).to_string(),
                    signature: name.clone(),
                    qualifiers,
                    parent: parent.map(|s| s.to_string()),
                });

                // Recurse into body with this class as parent
                if let Some(body) = node.child_by_field_name("body") {
                    for i in 0..body.child_count() {
                        Self::walk(body.child(i).unwrap(), src, file_path, entities, Some(&name), false);
                    }
                }
            }

            "method_definition" => {
                let name = node
                    .child_by_field_name("name")
                    .map(|n| Self::node_text(n, src).to_string())
                    .unwrap_or_default();
                let params = node
                    .child_by_field_name("parameters")
                    .map(|n| Self::node_text(n, src).to_string())
                    .unwrap_or_default();

                let kind = if name == "constructor" {
                    EntityKind::Constructor
                } else {
                    EntityKind::Method
                };

                let mut qualifiers = vec![];
                for i in 0..node.child_count() {
                    match node.child(i).unwrap().kind() {
                        "async" | "static" | "public" | "private" | "protected"
                        | "override" | "abstract" | "readonly" => {
                            qualifiers.push(node.child(i).unwrap().kind().to_string());
                        }
                        _ => {}
                    }
                }

                entities.push(CodeEntity {
                    kind,
                    name,
                    file_path: file_path.to_string(),
                    start_line: node.start_position().row + 1,
                    end_line: node.end_position().row + 1,
                    source: Self::node_text(node, src).to_string(),
                    signature: params,
                    qualifiers,
                    parent: parent.map(|s| s.to_string()),
                });
            }

            "public_field_definition" => {
                let name = node
                    .child_by_field_name("name")
                    .map(|n| Self::node_text(n, src).to_string())
                    .unwrap_or_default();
                entities.push(CodeEntity {
                    kind: EntityKind::Property,
                    name,
                    file_path: file_path.to_string(),
                    start_line: node.start_position().row + 1,
                    end_line: node.end_position().row + 1,
                    source: Self::node_text(node, src).to_string(),
                    signature: String::new(),
                    qualifiers: vec![],
                    parent: parent.map(|s| s.to_string()),
                });
            }

            _ => {
                for i in 0..node.child_count() {
                    Self::walk(node.child(i).unwrap(), src, file_path, entities, parent, is_exported);
                }
            }
        }
    }
}
