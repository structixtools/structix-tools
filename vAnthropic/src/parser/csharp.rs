use tree_sitter::{Language, Node, Parser};
use crate::parser::types::{CodeEntity, EntityKind};

const QUALIFIER_KEYWORDS: &[&str] = &[
    "public", "private", "protected", "internal", "static",
    "async", "abstract", "virtual", "override", "sealed",
    "readonly", "partial",
];

pub struct CSharpParser {
    language: Language,
}

impl CSharpParser {
    pub fn new() -> Self {
        Self {
            language: tree_sitter_c_sharp::LANGUAGE.into(),
        }
    }

    pub fn extract(&self, file_path: &str, source: &str) -> Vec<CodeEntity> {
        let mut parser = Parser::new();
        parser.set_language(&self.language).expect("load cs language");
        let tree = parser.parse(source, None).expect("parse");
        let src = source.as_bytes();
        let mut entities = Vec::new();
        Self::walk(tree.root_node(), src, file_path, &mut entities, None);
        entities
    }

    fn node_text<'a>(node: Node, src: &'a [u8]) -> &'a str {
        std::str::from_utf8(&src[node.start_byte()..node.end_byte()]).unwrap_or("")
    }

    fn collect_qualifiers(node: Node, src: &[u8]) -> Vec<String> {
        let mut quals = vec![];
        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();
            let text = Self::node_text(child, src);
            if QUALIFIER_KEYWORDS.contains(&text) {
                quals.push(text.to_string());
            }
        }
        quals
    }

    fn walk(node: Node, src: &[u8], file_path: &str, entities: &mut Vec<CodeEntity>, parent: Option<&str>) {
        match node.kind() {
            "interface_declaration" => {
                let name = node.child_by_field_name("name")
                    .map(|n| Self::node_text(n, src).to_string())
                    .unwrap_or_default();
                entities.push(CodeEntity {
                    kind: EntityKind::Interface,
                    name,
                    file_path: file_path.to_string(),
                    start_line: node.start_position().row + 1,
                    end_line: node.end_position().row + 1,
                    source: Self::node_text(node, src).to_string(),
                    signature: String::new(),
                    qualifiers: Self::collect_qualifiers(node, src),
                    parent: parent.map(|s| s.to_string()),
                });
                // Don't recurse into interface body
            }

            "class_declaration" => {
                let name = node.child_by_field_name("name")
                    .map(|n| Self::node_text(n, src).to_string())
                    .unwrap_or_default();
                entities.push(CodeEntity {
                    kind: EntityKind::Class,
                    name: name.clone(),
                    file_path: file_path.to_string(),
                    start_line: node.start_position().row + 1,
                    end_line: node.end_position().row + 1,
                    source: Self::node_text(node, src).to_string(),
                    signature: name.clone(),
                    qualifiers: Self::collect_qualifiers(node, src),
                    parent: parent.map(|s| s.to_string()),
                });
                if let Some(body) = node.child_by_field_name("body") {
                    for i in 0..body.child_count() {
                        Self::walk(body.child(i).unwrap(), src, file_path, entities, Some(&name));
                    }
                }
            }

            "method_declaration" => {
                let name = node.child_by_field_name("name")
                    .map(|n| Self::node_text(n, src).to_string())
                    .unwrap_or_default();
                let params = node.child_by_field_name("parameters")
                    .map(|n| Self::node_text(n, src).to_string())
                    .unwrap_or_default();
                let ret = node.child_by_field_name("type")
                    .map(|n| Self::node_text(n, src).to_string())
                    .unwrap_or_default();
                entities.push(CodeEntity {
                    kind: EntityKind::Method,
                    name,
                    file_path: file_path.to_string(),
                    start_line: node.start_position().row + 1,
                    end_line: node.end_position().row + 1,
                    source: Self::node_text(node, src).to_string(),
                    signature: format!("{} {}", ret, params).trim().to_string(),
                    qualifiers: Self::collect_qualifiers(node, src),
                    parent: parent.map(|s| s.to_string()),
                });
            }

            "constructor_declaration" => {
                let name = node.child_by_field_name("name")
                    .map(|n| Self::node_text(n, src).to_string())
                    .unwrap_or_default();
                entities.push(CodeEntity {
                    kind: EntityKind::Constructor,
                    name,
                    file_path: file_path.to_string(),
                    start_line: node.start_position().row + 1,
                    end_line: node.end_position().row + 1,
                    source: Self::node_text(node, src).to_string(),
                    signature: String::new(),
                    qualifiers: Self::collect_qualifiers(node, src),
                    parent: parent.map(|s| s.to_string()),
                });
            }

            _ => {
                for i in 0..node.child_count() {
                    Self::walk(node.child(i).unwrap(), src, file_path, entities, parent);
                }
            }
        }
    }
}
