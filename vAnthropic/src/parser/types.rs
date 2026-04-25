use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityKind {
    Function,
    Method,
    Class,
    Interface,
    Enum,
    Import,
    Export,
    Property,
    Constructor,
}

impl EntityKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            EntityKind::Function => "function",
            EntityKind::Method => "method",
            EntityKind::Class => "class",
            EntityKind::Interface => "interface",
            EntityKind::Enum => "enum",
            EntityKind::Import => "import",
            EntityKind::Export => "export",
            EntityKind::Property => "property",
            EntityKind::Constructor => "constructor",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeEntity {
    pub kind: EntityKind,
    pub name: String,
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
    pub source: String,
    pub signature: String,
    pub qualifiers: Vec<String>,
    pub parent: Option<String>,
}

impl CodeEntity {
    pub fn unique_key(&self) -> String {
        format!(
            "{}::{}::{}::{}",
            self.file_path,
            self.kind.as_str(),
            self.parent.as_deref().unwrap_or(""),
            self.name
        )
    }
}
