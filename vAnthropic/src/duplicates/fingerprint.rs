use std::collections::HashMap;
use sha2::{Digest, Sha256};
use tree_sitter::{Language, Node, Parser};
use crate::parser::types::CodeEntity;
use crate::duplicates::normalizer::normalize;
use tree_sitter_c_sharp;

const IDENTIFIER_KINDS: &[&str] = &[
    "identifier",
    "property_identifier",
    "type_identifier",
    "field_identifier",
    "shorthand_property_identifier",
    "shorthand_property_identifier_pattern",
];

fn abstract_tokens(node: Node, src: &[u8], tokens: &mut Vec<String>) {
    if IDENTIFIER_KINDS.contains(&node.kind()) {
        tokens.push("ID".to_string());
    } else if node.child_count() == 0 {
        // Leaf — keep its syntactic type as a structural marker
        tokens.push(node.kind().to_string());
    } else {
        tokens.push(format!("<{}>", node.kind()));
        for i in 0..node.child_count() {
            abstract_tokens(node.child(i).unwrap(), src, tokens);
        }
        tokens.push(format!("</{}>", node.kind()));
    }
}

fn language_for(file_path: &str) -> Language {
    if file_path.ends_with(".cs") {
        tree_sitter_c_sharp::LANGUAGE.into()
    } else {
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()
    }
}

/// Minimum number of abstract tokens for an entity to be considered for structural cloning.
/// Filters out trivially small functions (getters, empty stubs) that match everywhere.
const MIN_TOKENS: usize = 15;

pub fn structural_fingerprint(entity: &CodeEntity) -> Option<String> {
    let language = language_for(&entity.file_path);
    let mut parser = Parser::new();
    parser.set_language(&language).expect("load language");
    let src = entity.source.as_bytes();
    let tree = parser.parse(src, None).expect("parse");
    let mut tokens = Vec::new();
    abstract_tokens(tree.root_node(), src, &mut tokens);
    if tokens.len() < MIN_TOKENS {
        return None;
    }
    let token_str = tokens.join(" ");
    let mut hasher = Sha256::new();
    hasher.update(token_str.as_bytes());
    Some(format!("{:x}", hasher.finalize()))
}

fn exact_normalized_hash(entity: &CodeEntity) -> String {
    let normalized = normalize(&entity.source);
    let mut hasher = Sha256::new();
    hasher.update(normalized.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Group entities with the same AST structure (identifiers abstracted).
/// Excludes groups where all members are also exact clones (those belong to hasher).
pub fn find_structural_clones(entities: &[CodeEntity]) -> Vec<Vec<&CodeEntity>> {
    // Collect exact-clone hashes to exclude
    let mut exact_buckets: HashMap<String, usize> = HashMap::new();
    for e in entities {
        *exact_buckets.entry(exact_normalized_hash(e)).or_default() += 1;
    }
    let exact_hashes: std::collections::HashSet<String> = exact_buckets
        .into_iter()
        .filter(|(_, count)| *count >= 2)
        .map(|(h, _)| h)
        .collect();

    let mut structural_buckets: HashMap<String, Vec<&CodeEntity>> = HashMap::new();
    for e in entities {
        if let Some(fp) = structural_fingerprint(e) {
            structural_buckets.entry(fp).or_default().push(e);
        }
    }

    structural_buckets
        .into_values()
        .filter(|group| {
            if group.len() < 2 {
                return false;
            }
            // Exclude groups where every member is an exact clone of each other
            let all_exact = group
                .iter()
                .all(|e| exact_hashes.contains(&exact_normalized_hash(e)));
            !all_exact
        })
        .collect()
}
