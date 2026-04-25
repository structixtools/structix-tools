use std::collections::HashMap;
use sha2::{Digest, Sha256};
use crate::parser::types::CodeEntity;
use crate::duplicates::normalizer::normalize;

fn hash_entity(e: &CodeEntity) -> String {
    let normalized = normalize(&e.source);
    let mut hasher = Sha256::new();
    hasher.update(normalized.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Group entities with identical normalized source. Returns groups of 2+.
pub fn find_exact_clones(entities: &[CodeEntity]) -> Vec<Vec<&CodeEntity>> {
    let mut buckets: HashMap<String, Vec<&CodeEntity>> = HashMap::new();
    for e in entities {
        buckets.entry(hash_entity(e)).or_default().push(e);
    }
    buckets.into_values().filter(|g| g.len() >= 2).collect()
}
