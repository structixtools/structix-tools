use crate::lang::Language;

use serde::Serialize;

use super::model::{Reference, Symbol};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct FileMetrics {
    pub code_lines: usize,
    pub comment_lines: usize,
    pub blank_lines: usize,
    pub cyclomatic_complexity: usize,
}

impl FileMetrics {
    pub fn from_source(source: &str) -> Self {
        let mut metrics = Self::default();
        let mut in_block_comment = false;

        for line in source.lines() {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                metrics.blank_lines += 1;
                continue;
            }

            if in_block_comment {
                metrics.comment_lines += 1;
                if trimmed.contains("*/") {
                    in_block_comment = false;
                }
                continue;
            }

            if trimmed.starts_with("//") {
                metrics.comment_lines += 1;
                continue;
            }

            if trimmed.starts_with("/*") {
                metrics.comment_lines += 1;
                if !trimmed.contains("*/") {
                    in_block_comment = true;
                }
                continue;
            }

            metrics.code_lines += 1;
        }

        metrics
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FileAnalysis {
    pub path: String,
    pub language: Language,
    pub symbols: Vec<Symbol>,
    pub references: Vec<Reference>,
    pub metrics: FileMetrics,
    pub lexical_hash: Option<u64>,
    pub token_hash: Option<u64>,
    pub ast_hash: Option<u64>,
}

impl FileAnalysis {
    pub fn new(path: impl Into<String>, language: Language) -> Self {
        Self {
            path: path.into(),
            language,
            symbols: Vec::new(),
            references: Vec::new(),
            metrics: FileMetrics::default(),
            lexical_hash: None,
            token_hash: None,
            ast_hash: None,
        }
    }
}
