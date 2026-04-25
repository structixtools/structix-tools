use std::path::Path;

use crate::ir::file::FileAnalysis;
use crate::ir::model::{SourceSpan, Symbol, SymbolKind};

use super::{fingerprint, Language, ParseError};

pub fn parse_file(path: impl AsRef<Path>, source: &str) -> Result<FileAnalysis, ParseError> {
    let path = path.as_ref();

    let mut parser = tree_sitter::Parser::new();
    let language = tree_sitter_c_sharp::LANGUAGE;
    parser
        .set_language(&language.into())
        .map_err(|err| ParseError::new(format!("failed to load C# parser: {err}")))?;

    let tree = parser
        .parse(source, None)
        .ok_or_else(|| ParseError::new("failed to parse C# source"))?;

    let mut analysis = FileAnalysis::new(path.to_string_lossy(), Language::CSharp);
    analysis.metrics.code_lines = source.lines().count();

    analysis.lexical_hash = Some(fingerprint::lexical_hash(tree.root_node(), source));
    analysis.token_hash = Some(fingerprint::token_hash(tree.root_node(), source));
    analysis.ast_hash = Some(fingerprint::structure_hash(tree.root_node()));

    collect_symbols(tree.root_node(), source, path, &mut analysis)?;

    Ok(analysis)
}

fn collect_symbols(
    node: tree_sitter::Node,
    source: &str,
    path: &Path,
    analysis: &mut FileAnalysis,
) -> Result<(), ParseError> {
    match node.kind() {
        "class_declaration" => {
            push_symbol(
                node,
                source,
                path,
                analysis,
                SymbolKind::Class,
                has_public_modifier(node, source),
            )?;
        }
        "interface_declaration" => {
            push_symbol(
                node,
                source,
                path,
                analysis,
                SymbolKind::Interface,
                has_public_modifier(node, source),
            )?;
        }
        "record_declaration" => {
            push_symbol(
                node,
                source,
                path,
                analysis,
                SymbolKind::Record,
                has_public_modifier(node, source),
            )?;
        }
        "method_declaration" => {
            push_symbol(
                node,
                source,
                path,
                analysis,
                SymbolKind::Method,
                has_public_modifier(node, source),
            )?;
        }
        "constructor_declaration" => {
            push_symbol(
                node,
                source,
                path,
                analysis,
                SymbolKind::Constructor,
                has_public_modifier(node, source),
            )?;
        }
        "property_declaration" => {
            push_symbol(
                node,
                source,
                path,
                analysis,
                SymbolKind::Property,
                has_public_modifier(node, source),
            )?;
        }
        "field_declaration" => {
            push_field_symbols(node, source, path, analysis)?;
        }
        _ => {
            let mut cursor = node.walk();
            for child in node.named_children(&mut cursor) {
                collect_symbols(child, source, path, analysis)?;
            }
        }
    }

    Ok(())
}

fn push_field_symbols(
    node: tree_sitter::Node,
    source: &str,
    path: &Path,
    analysis: &mut FileAnalysis,
) -> Result<(), ParseError> {
    let exported = has_public_modifier(node, source);
    let mut cursor = node.walk();

    for child in node.named_children(&mut cursor) {
        if child.kind() == "variable_declarator" {
            push_symbol(child, source, path, analysis, SymbolKind::Field, exported)?;
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
    let Some(name_node) = node.child_by_field_name("name") else {
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

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        collect_symbols(child, source, path, analysis)?;
    }

    Ok(())
}

fn has_public_modifier(node: tree_sitter::Node, source: &str) -> bool {
    let mut cursor = node.walk();
    let has_public = node.children(&mut cursor).any(|child| {
        child.kind() == "modifier"
            && child
                .utf8_text(source.as_bytes())
                .map(|text| text.trim() == "public")
                .unwrap_or(false)
    });
    has_public
}
