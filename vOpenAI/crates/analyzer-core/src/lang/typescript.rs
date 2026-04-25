use std::path::Path;

use crate::ir::file::FileAnalysis;
use crate::ir::model::{SourceSpan, Symbol, SymbolKind};

use super::{fingerprint, Language, ParseError};

pub fn parse_file(path: impl AsRef<Path>, source: &str) -> Result<FileAnalysis, ParseError> {
    let path = path.as_ref();

    let mut parser = tree_sitter::Parser::new();
    let language = tree_sitter_typescript::LANGUAGE_TYPESCRIPT;
    parser
        .set_language(&language.into())
        .map_err(|err| ParseError::new(format!("failed to load TypeScript parser: {err}")))?;

    let tree = parser
        .parse(source, None)
        .ok_or_else(|| ParseError::new("failed to parse TypeScript source"))?;

    let mut analysis = FileAnalysis::new(path.to_string_lossy(), Language::TypeScript);
    analysis.metrics.code_lines = source.lines().count();

    analysis.lexical_hash = Some(fingerprint::lexical_hash(tree.root_node(), source));
    analysis.token_hash = Some(fingerprint::token_hash(tree.root_node(), source));
    analysis.ast_hash = Some(fingerprint::structure_hash(tree.root_node()));

    collect_symbols(tree.root_node(), source, path, &mut analysis, false)?;

    Ok(analysis)
}

fn collect_symbols(
    node: tree_sitter::Node,
    source: &str,
    path: &Path,
    analysis: &mut FileAnalysis,
    exported: bool,
) -> Result<(), ParseError> {
    match node.kind() {
        "export_statement" => {
            let mut cursor = node.walk();
            for child in node.named_children(&mut cursor) {
                collect_symbols(child, source, path, analysis, true)?;
            }
        }
        "function_declaration" => {
            push_symbol(node, source, path, analysis, SymbolKind::Function, exported)?;
        }
        "class_declaration" => {
            push_symbol(node, source, path, analysis, SymbolKind::Class, exported)?;
        }
        "interface_declaration" => {
            push_symbol(
                node,
                source,
                path,
                analysis,
                SymbolKind::Interface,
                exported,
            )?;
        }
        "type_alias_declaration" => {
            push_symbol(
                node,
                source,
                path,
                analysis,
                SymbolKind::TypeAlias,
                exported,
            )?;
        }
        _ => {
            let mut cursor = node.walk();
            for child in node.named_children(&mut cursor) {
                collect_symbols(child, source, path, analysis, exported)?;
            }
        }
    }

    Ok(())
}

fn push_symbol(
    node: tree_sitter::Node,
    source: &str,
    path: &Path,
    analysis: &mut FileAnalysis,
    kind: SymbolKind,
    exported: bool,
) -> Result<(), ParseError> {
    let name_node = node
        .child_by_field_name("name")
        .ok_or_else(|| ParseError::new(format!("missing name for {}", node.kind())));
    let Ok(name_node) = name_node else {
        return Ok(());
    };
    let Ok(name) = name_node.utf8_text(source.as_bytes()) else {
        return Ok(());
    };
    let start = node.start_position();
    let end = node.end_position();
    let span = SourceSpan::new(
        path.to_string_lossy(),
        start.row as u32 + 1,
        start.column as u32 + 1,
        end.row as u32 + 1,
        end.column as u32 + 1,
    );

    let signature = source[node.start_byte()..node.end_byte()]
        .trim()
        .to_string();
    let symbol = Symbol::new(
        format!("{}::{}", path.to_string_lossy(), name),
        name,
        kind,
        path.to_string_lossy(),
        span,
    )
    .with_signature(signature)
    .exported(exported);

    analysis.symbols.push(symbol);
    Ok(())
}
