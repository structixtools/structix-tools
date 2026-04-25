use std::collections::{BTreeMap, BTreeSet};

use crate::ir::file::FileAnalysis;
use crate::ir::model::{Symbol, SymbolChange};

pub fn detect_symbol_changes(base: &[FileAnalysis], head: &[FileAnalysis]) -> Vec<SymbolChange> {
    detect_symbol_changes_with_renames(base, head, &[])
}

pub fn detect_symbol_changes_with_renames(
    base: &[FileAnalysis],
    head: &[FileAnalysis],
    renames: &[(&str, &str)],
) -> Vec<SymbolChange> {
    let base_index: BTreeMap<&str, &FileAnalysis> =
        base.iter().map(|file| (file.path.as_str(), file)).collect();
    let head_index: BTreeMap<&str, &FileAnalysis> =
        head.iter().map(|file| (file.path.as_str(), file)).collect();
    let rename_lookup: BTreeMap<&str, &str> = renames.iter().copied().collect();

    let mut changes = Vec::new();
    let mut visited_base = BTreeSet::new();

    for head_file in head {
        let base_path = rename_lookup
            .get(head_file.path.as_str())
            .copied()
            .unwrap_or(head_file.path.as_str());

        if let Some(base_file) = base_index.get(base_path) {
            visited_base.insert(base_file.path.as_str());
            changes.extend(diff_symbols(base_file, head_file));
        } else {
            changes.extend(head_file.symbols.iter().map(SymbolChange::added));
        }
    }

    for base_file in base {
        if visited_base.contains(base_file.path.as_str()) {
            continue;
        }

        if head_index.contains_key(base_file.path.as_str()) {
            continue;
        }

        changes.extend(base_file.symbols.iter().map(SymbolChange::removed));
    }

    changes
}

fn diff_symbols(base: &FileAnalysis, head: &FileAnalysis) -> Vec<SymbolChange> {
    let base_symbols = index_symbols(&base.symbols);
    let head_symbols = index_symbols(&head.symbols);
    let mut changes = Vec::new();

    for (key, head_symbol) in &head_symbols {
        match base_symbols.get(key.as_str()) {
            None => changes.push(SymbolChange::added(head_symbol)),
            Some(base_symbol) => {
                if signatures_differ(base_symbol, head_symbol) {
                    changes.push(SymbolChange::modified(base_symbol, head_symbol));
                }
            }
        }
    }

    for (key, base_symbol) in &base_symbols {
        if !head_symbols.contains_key(key.as_str()) {
            changes.push(SymbolChange::removed(base_symbol));
        }
    }

    changes
}

fn index_symbols(symbols: &[Symbol]) -> BTreeMap<String, &Symbol> {
    symbols
        .iter()
        .map(|symbol| (symbol.change_key(), symbol))
        .collect()
}

fn signatures_differ(base: &Symbol, head: &Symbol) -> bool {
    base.signature != head.signature
}
