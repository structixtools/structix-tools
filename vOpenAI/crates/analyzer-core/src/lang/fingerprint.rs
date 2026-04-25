pub fn lexical_hash(root: tree_sitter::Node, source: &str) -> u64 {
    let mut hasher = Fnv64::new();
    hash_lexical(root, source, &mut hasher);
    hasher.finish()
}

pub fn token_hash(root: tree_sitter::Node, source: &str) -> u64 {
    let mut hasher = Fnv64::new();
    hash_tokens(root, source, &mut hasher);
    hasher.finish()
}

pub fn structure_hash(root: tree_sitter::Node) -> u64 {
    let mut hasher = Fnv64::new();
    hash_structure(root, &mut hasher);
    hasher.finish()
}

fn hash_tokens(node: tree_sitter::Node, source: &str, hasher: &mut Fnv64) {
    if is_comment_kind(node.kind()) {
        return;
    }

    if node.child_count() == 0 {
        let token = normalize_token(node, source);
        if !token.is_empty() {
            hasher.write(token.as_bytes());
            hasher.write_u8(0);
        }
        return;
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        hash_tokens(child, source, hasher);
    }
}

fn hash_lexical(node: tree_sitter::Node, source: &str, hasher: &mut Fnv64) {
    if is_comment_kind(node.kind()) {
        return;
    }

    if node.child_count() == 0 {
        if let Ok(token) = node.utf8_text(source.as_bytes()) {
            hasher.write(token.as_bytes());
            hasher.write_u8(0);
        }
        return;
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        hash_lexical(child, source, hasher);
    }
}

fn hash_structure(node: tree_sitter::Node, hasher: &mut Fnv64) {
    if is_comment_kind(node.kind()) {
        return;
    }

    hasher.write(node.kind().as_bytes());
    hasher.write_u8(0xfe);
    hasher.write_u32(node.child_count() as u32);
    hasher.write_u8(0xff);

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        hash_structure(child, hasher);
    }
}

fn normalize_token(node: tree_sitter::Node, source: &str) -> String {
    let kind = node.kind();

    if kind == "identifier" {
        return "IDENT".to_string();
    }

    if is_literal_kind(kind) {
        return "LIT".to_string();
    }

    if kind == "modifier" {
        return node
            .utf8_text(source.as_bytes())
            .map(|text| text.trim().to_ascii_lowercase())
            .unwrap_or_default();
    }

    kind.to_ascii_lowercase()
}

fn is_comment_kind(kind: &str) -> bool {
    kind.contains("comment")
}

fn is_literal_kind(kind: &str) -> bool {
    matches!(
        kind,
        "string"
            | "number"
            | "null"
            | "true"
            | "false"
            | "character_literal"
            | "integer_literal"
            | "real_literal"
            | "string_literal"
            | "verbatim_string_literal"
            | "raw_string_literal"
    ) || kind.contains("literal")
}

struct Fnv64(u64);

impl Fnv64 {
    const OFFSET_BASIS: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;

    fn new() -> Self {
        Self(Self::OFFSET_BASIS)
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 ^= u64::from(*byte);
            self.0 = self.0.wrapping_mul(Self::PRIME);
        }
    }

    fn write_u8(&mut self, value: u8) {
        self.write(&[value]);
    }

    fn write_u32(&mut self, value: u32) {
        self.write(&value.to_le_bytes());
    }

    fn finish(self) -> u64 {
        self.0
    }
}
