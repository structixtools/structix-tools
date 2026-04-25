use std::path::Path;

use crate::ir::file::FileAnalysis;

use serde::Serialize;

pub mod csharp;
pub mod fingerprint;
pub mod typescript;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Language {
    CSharp,
    TypeScript,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub message: String,
}

impl ParseError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ParseError {}

impl Language {
    pub fn from_path(path: impl AsRef<Path>) -> Option<Self> {
        let extension = path.as_ref().extension()?.to_str()?.to_ascii_lowercase();

        match extension.as_str() {
            "cs" => Some(Self::CSharp),
            "ts" | "tsx" => Some(Self::TypeScript),
            _ => None,
        }
    }

    pub fn parse(self, path: impl AsRef<Path>, source: &str) -> Result<FileAnalysis, ParseError> {
        match self {
            Self::CSharp => csharp::parse_file(path, source),
            Self::TypeScript => typescript::parse_file(path, source),
        }
    }
}
