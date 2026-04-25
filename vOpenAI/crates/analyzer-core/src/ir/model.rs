use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SourceSpan {
    pub file: String,
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: u32,
    pub end_column: u32,
}

impl SourceSpan {
    pub fn new(
        file: impl Into<String>,
        start_line: u32,
        start_column: u32,
        end_line: u32,
        end_column: u32,
    ) -> Self {
        Self {
            file: file.into(),
            start_line,
            start_column,
            end_line,
            end_column,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Evidence {
    pub message: String,
    pub spans: Vec<SourceSpan>,
}

impl Evidence {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            spans: Vec::new(),
        }
    }

    pub fn with_span(mut self, span: SourceSpan) -> Self {
        self.spans.push(span);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Finding {
    pub code: String,
    pub severity: Severity,
    pub confidence: f32,
    pub risk_score: f32,
    pub title: String,
    pub detail: String,
    pub evidence: Vec<Evidence>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SymbolChangeKind {
    Added,
    Removed,
    Modified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SymbolKind {
    Namespace,
    Module,
    Class,
    Struct,
    Interface,
    Enum,
    Record,
    Function,
    Method,
    Constructor,
    Property,
    Field,
    Variable,
    Constant,
    TypeAlias,
    Parameter,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Symbol {
    pub id: String,
    pub name: String,
    pub kind: SymbolKind,
    pub file: String,
    pub span: SourceSpan,
    pub container: Option<String>,
    pub signature: Option<String>,
    pub exported: bool,
}

impl Symbol {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        kind: SymbolKind,
        file: impl Into<String>,
        span: SourceSpan,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            kind,
            file: file.into(),
            span,
            container: None,
            signature: None,
            exported: false,
        }
    }

    pub fn with_container(mut self, container: impl Into<String>) -> Self {
        self.container = Some(container.into());
        self
    }

    pub fn with_signature(mut self, signature: impl Into<String>) -> Self {
        self.signature = Some(signature.into());
        self
    }

    pub fn exported(mut self, exported: bool) -> Self {
        self.exported = exported;
        self
    }

    pub fn qualified_name(&self) -> String {
        match &self.container {
            Some(container) if !container.is_empty() => format!("{container}.{}", self.name),
            _ => self.name.clone(),
        }
    }

    pub fn change_key(&self) -> String {
        format!("{:?}|{}", self.kind, self.qualified_name())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Reference {
    pub from_symbol: String,
    pub to_symbol: String,
    pub span: SourceSpan,
}

impl Reference {
    pub fn new(
        from_symbol: impl Into<String>,
        to_symbol: impl Into<String>,
        span: SourceSpan,
    ) -> Self {
        Self {
            from_symbol: from_symbol.into(),
            to_symbol: to_symbol.into(),
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SymbolChange {
    pub file: String,
    pub qualified_name: String,
    pub name: String,
    pub symbol_kind: SymbolKind,
    pub kind: SymbolChangeKind,
    pub exported: bool,
    pub before_signature: Option<String>,
    pub after_signature: Option<String>,
    pub before_span: Option<SourceSpan>,
    pub after_span: Option<SourceSpan>,
}

impl SymbolChange {
    pub fn added(symbol: &Symbol) -> Self {
        Self {
            file: symbol.file.clone(),
            qualified_name: symbol.qualified_name(),
            name: symbol.name.clone(),
            symbol_kind: symbol.kind,
            kind: SymbolChangeKind::Added,
            exported: symbol.exported,
            before_signature: None,
            after_signature: symbol.signature.clone(),
            before_span: None,
            after_span: Some(symbol.span.clone()),
        }
    }

    pub fn removed(symbol: &Symbol) -> Self {
        Self {
            file: symbol.file.clone(),
            qualified_name: symbol.qualified_name(),
            name: symbol.name.clone(),
            symbol_kind: symbol.kind,
            kind: SymbolChangeKind::Removed,
            exported: symbol.exported,
            before_signature: symbol.signature.clone(),
            after_signature: None,
            before_span: Some(symbol.span.clone()),
            after_span: None,
        }
    }

    pub fn modified(before: &Symbol, after: &Symbol) -> Self {
        Self {
            file: after.file.clone(),
            qualified_name: after.qualified_name(),
            name: after.name.clone(),
            symbol_kind: after.kind,
            kind: SymbolChangeKind::Modified,
            exported: before.exported || after.exported,
            before_signature: before.signature.clone(),
            after_signature: after.signature.clone(),
            before_span: Some(before.span.clone()),
            after_span: Some(after.span.clone()),
        }
    }
}

impl Finding {
    pub fn new(
        code: impl Into<String>,
        severity: Severity,
        confidence: f32,
        title: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            severity,
            confidence,
            risk_score: 0.0,
            title: title.into(),
            detail: detail.into(),
            evidence: Vec::new(),
        }
    }

    pub fn with_risk_score(mut self, risk_score: f32) -> Self {
        self.risk_score = risk_score;
        self
    }

    pub fn with_evidence(mut self, evidence: Evidence) -> Self {
        self.evidence.push(evidence);
        self
    }
}
