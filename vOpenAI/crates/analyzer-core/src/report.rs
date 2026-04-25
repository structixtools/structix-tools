use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::architecture::{component_summaries, ComponentSummary};
use crate::detectors::change::{diff_files, ChangeSet, FileChange};
use crate::detectors::clone::{detect_exact_clone_groups, CloneGroup};
use crate::detectors::symbol::detect_symbol_changes_with_renames;
use crate::ir::file::FileAnalysis;
use crate::ir::model::{Evidence, Finding, Severity, SymbolChange, SymbolChangeKind};

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct RenamedFile {
    pub from: String,
    pub to: String,
}

impl RenamedFile {
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Report {
    pub base_ref: String,
    pub head_ref: String,
    pub base_files: Vec<FileAnalysis>,
    pub head_files: Vec<FileAnalysis>,
    pub base_components: Vec<ComponentSummary>,
    pub head_components: Vec<ComponentSummary>,
    pub summary: String,
    pub change_set: ChangeSet,
    pub clone_groups: Vec<CloneGroup>,
    pub symbol_changes: Vec<SymbolChange>,
    pub api_changes: Vec<SymbolChange>,
    pub renames: Vec<RenamedFile>,
    pub findings: Vec<Finding>,
}

pub fn build_report(base: &[FileAnalysis], head: &[FileAnalysis]) -> Report {
    let empty_renames: &[RenamedFile] = &[];
    build_report_with_context("", "", base, head, empty_renames)
}

pub fn build_report_with_renames(
    base: &[FileAnalysis],
    head: &[FileAnalysis],
    renames: &[RenamedFile],
) -> Report {
    build_report_with_context("", "", base, head, renames)
}

pub fn build_report_with_context(
    base_ref: impl Into<String>,
    head_ref: impl Into<String>,
    base: &[FileAnalysis],
    head: &[FileAnalysis],
    renames: &[RenamedFile],
) -> Report {
    let base_ref = base_ref.into();
    let head_ref = head_ref.into();
    let change_set = diff_files(base, head);
    let clone_groups = detect_exact_clone_groups(head);
    let rename_pairs: Vec<(&str, &str)> = renames
        .iter()
        .map(|rename| (rename.from.as_str(), rename.to.as_str()))
        .collect();
    let symbol_changes = detect_symbol_changes_with_renames(base, head, &rename_pairs);
    let api_changes: Vec<SymbolChange> = symbol_changes
        .iter()
        .filter(|change| change.exported)
        .cloned()
        .collect();
    let head_index: BTreeMap<&str, &FileAnalysis> =
        head.iter().map(|file| (file.path.as_str(), file)).collect();
    let base_index: BTreeMap<&str, &FileAnalysis> =
        base.iter().map(|file| (file.path.as_str(), file)).collect();
    let rename_from: BTreeSet<&str> = renames.iter().map(|rename| rename.from.as_str()).collect();
    let rename_to: BTreeSet<&str> = renames.iter().map(|rename| rename.to.as_str()).collect();

    let mut change_set = change_set;
    change_set
        .added
        .retain(|change| !rename_to.contains(change.path.as_str()));
    change_set
        .removed
        .retain(|change| !rename_from.contains(change.path.as_str()));

    let mut findings = Vec::new();

    for modified in &change_set.modified {
        if let Some(file) = head_index.get(modified.path.as_str()) {
            findings.push(modified_file_finding(modified, file));
        }
    }

    for added in &change_set.added {
        if let Some(file) = head_index.get(added.path.as_str()) {
            findings.push(added_file_finding(added, file));
        }
    }

    for removed in &change_set.removed {
        findings.push(removed_file_finding(removed));
    }

    for group in &clone_groups {
        findings.push(clone_group_finding(group, &head_index));
    }

    for rename in renames {
        findings.push(rename_file_finding(rename, &base_index, &head_index));
    }

    for change in &symbol_changes {
        findings.push(symbol_change_finding(change));
    }

    for change in &api_changes {
        findings.push(api_change_finding(change));
    }

    let summary = summarize(
        &change_set,
        &clone_groups,
        renames,
        &symbol_changes,
        &api_changes,
    );

    Report {
        base_ref,
        head_ref,
        base_files: base.to_vec(),
        head_files: head.to_vec(),
        base_components: component_summaries(base),
        head_components: component_summaries(head),
        summary,
        change_set,
        clone_groups,
        symbol_changes,
        api_changes,
        renames: renames.to_vec(),
        findings,
    }
}

impl Report {
    pub fn json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn html(&self) -> String {
        let data_json = serde_json::to_string(self).expect("report should serialize to JSON");
        let data_json = data_json.replace("</", "<\\/");

        REPORT_HTML_TEMPLATE.replace("__REPORT_JSON__", &data_json)
    }

    pub fn pr_comment(&self) -> String {
        let mut out = String::new();
        out.push_str("## Risk\n");

        let mut risk_findings: Vec<&Finding> = self
            .findings
            .iter()
            .filter(|finding| finding.risk_score > 0.0)
            .collect();
        risk_findings.sort_by(|left, right| {
            right
                .risk_score
                .partial_cmp(&left.risk_score)
                .unwrap_or(Ordering::Equal)
        });

        if risk_findings.is_empty() {
            out.push_str("- No high-risk API changes detected.\n");
        } else {
            for finding in risk_findings.iter().take(5) {
                out.push_str("- risk ");
                out.push_str(&format_percent(finding.risk_score));
                out.push_str(": ");
                out.push_str(&finding.title);
                out.push_str(" — ");
                out.push_str(&finding.detail);
                out.push('\n');
            }
        }

        out.push('\n');
        out.push_str(&self.markdown());
        out
    }

    pub fn markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# Change Report\n\n");
        out.push_str(&self.summary);
        out.push_str("\n\n## Highlights\n");

        if !self.symbol_changes.is_empty() {
            out.push_str("\n## Symbol Changes\n");
            for change in &self.symbol_changes {
                out.push_str("- ");
                out.push_str(&symbol_change_title(change));
                out.push_str(": ");
                out.push_str(&symbol_change_detail(change));
                out.push('\n');
            }
        }

        if !self.api_changes.is_empty() {
            out.push_str("\n## API Changes\n");
            for change in &self.api_changes {
                out.push_str("- ");
                out.push_str(&api_change_title(change));
                out.push_str(": ");
                out.push_str(&api_change_detail(change));
                out.push('\n');
            }
        }

        if self.findings.is_empty() {
            out.push_str("- No relevant changes detected.\n");
            return out;
        }

        for finding in &self.findings {
            out.push_str("- ");
            out.push_str(&finding.title);
            out.push_str(": ");
            out.push_str(&finding.detail);
            out.push('\n');
        }

        out
    }
}

fn modified_file_finding(change: &FileChange, file: &FileAnalysis) -> Finding {
    Finding::new(
        "file.modified",
        Severity::Medium,
        0.96,
        format!("Modified `{}`", change.path),
        format!("{} symbols: {}", change.path, symbol_names(file)),
    )
    .with_evidence(file_evidence(
        file,
        format!("modified file: {}", change.path),
    ))
}

fn added_file_finding(change: &FileChange, file: &FileAnalysis) -> Finding {
    Finding::new(
        "file.added",
        Severity::Low,
        0.95,
        format!("Added `{}`", change.path),
        format!("{} symbols: {}", change.path, symbol_names(file)),
    )
    .with_evidence(file_evidence(file, format!("added file: {}", change.path)))
}

fn removed_file_finding(change: &FileChange) -> Finding {
    Finding::new(
        "file.removed",
        Severity::Low,
        0.95,
        format!("Removed `{}`", change.path),
        format!("{} was removed from the head revision", change.path),
    )
    .with_evidence(Evidence::new(format!("removed file: {}", change.path)))
}

fn rename_file_finding(
    rename: &RenamedFile,
    base_index: &BTreeMap<&str, &FileAnalysis>,
    head_index: &BTreeMap<&str, &FileAnalysis>,
) -> Finding {
    let base_file = base_index.get(rename.from.as_str());
    let head_file = head_index.get(rename.to.as_str());
    let content_changed = match (base_file, head_file) {
        (Some(base_file), Some(head_file)) => {
            base_file.lexical_hash != head_file.lexical_hash
                || base_file.token_hash != head_file.token_hash
                || base_file.ast_hash != head_file.ast_hash
        }
        _ => false,
    };

    let mut evidence = Evidence::new(format!("rename: {} -> {}", rename.from, rename.to));
    if let Some(file) = head_file {
        for symbol in &file.symbols {
            evidence = evidence.with_span(symbol.span.clone());
        }
    }

    let detail = if content_changed {
        format!(
            "Renamed `{}` -> `{}` with content changes",
            rename.from, rename.to
        )
    } else {
        format!(
            "Renamed `{}` -> `{}` with content preserved",
            rename.from, rename.to
        )
    };

    Finding::new(
        "file.renamed",
        if content_changed {
            Severity::Medium
        } else {
            Severity::Low
        },
        0.98,
        format!("Renamed `{}` -> `{}`", rename.from, rename.to),
        detail,
    )
    .with_evidence(evidence)
}

fn clone_group_finding(group: &CloneGroup, head_index: &BTreeMap<&str, &FileAnalysis>) -> Finding {
    let member_list = group
        .members
        .iter()
        .map(|member| format!("`{member}`"))
        .collect::<Vec<_>>()
        .join(", ");

    let mut evidence = Evidence::new(format!("exact clone group: {member_list}"));
    for member in &group.members {
        if let Some(file) = head_index.get(member.as_str()) {
            for symbol in &file.symbols {
                evidence = evidence.with_span(symbol.span.clone());
            }
        }
    }

    Finding::new(
        "clone.exact",
        Severity::Medium,
        0.99,
        "Exact clone group",
        format!("Exact clone group across {member_list}"),
    )
    .with_evidence(evidence)
}

fn symbol_change_finding(change: &SymbolChange) -> Finding {
    let (code, severity) = match change.kind {
        SymbolChangeKind::Added => ("symbol.added", Severity::Low),
        SymbolChangeKind::Removed => ("symbol.removed", Severity::Medium),
        SymbolChangeKind::Modified => ("symbol.modified", Severity::Medium),
    };

    Finding::new(
        code,
        severity,
        0.97,
        symbol_change_title(change),
        symbol_change_detail(change),
    )
    .with_evidence(symbol_change_evidence(change))
}

fn api_change_finding(change: &SymbolChange) -> Finding {
    let (code, severity, risk_score) = match change.kind {
        SymbolChangeKind::Added => ("api.added", Severity::Low, 0.20),
        SymbolChangeKind::Removed => ("api.removed", Severity::High, 0.98),
        SymbolChangeKind::Modified => ("api.changed", Severity::Medium, 0.82),
    };

    Finding::new(
        code,
        severity,
        0.99,
        api_change_title(change),
        api_change_detail(change),
    )
    .with_risk_score(risk_score)
    .with_evidence(symbol_change_evidence(change))
}

fn symbol_change_title(change: &SymbolChange) -> String {
    match change.kind {
        SymbolChangeKind::Added => format!("Added symbol `{}`", change.qualified_name),
        SymbolChangeKind::Removed => format!("Removed symbol `{}`", change.qualified_name),
        SymbolChangeKind::Modified => format!("Modified symbol `{}`", change.qualified_name),
    }
}

fn symbol_change_detail(change: &SymbolChange) -> String {
    match change.kind {
        SymbolChangeKind::Added => format!(
            "{} now defines `{}` ({:?})",
            change.file, change.qualified_name, change.symbol_kind
        ),
        SymbolChangeKind::Removed => format!(
            "{} no longer defines `{}` ({:?})",
            change.file, change.qualified_name, change.symbol_kind
        ),
        SymbolChangeKind::Modified => format!(
            "{} changed `{}` ({:?})",
            change.file, change.qualified_name, change.symbol_kind
        ),
    }
}

fn api_change_title(change: &SymbolChange) -> String {
    match change.kind {
        SymbolChangeKind::Added => format!("Added API `{}`", change.qualified_name),
        SymbolChangeKind::Removed => format!("Removed API `{}`", change.qualified_name),
        SymbolChangeKind::Modified => format!("Changed API `{}`", change.qualified_name),
    }
}

fn api_change_detail(change: &SymbolChange) -> String {
    match change.kind {
        SymbolChangeKind::Added => format!(
            "exported `{}` is now available in {}",
            change.qualified_name, change.file
        ),
        SymbolChangeKind::Removed => format!(
            "exported `{}` is no longer available in {}",
            change.qualified_name, change.file
        ),
        SymbolChangeKind::Modified => format!(
            "exported `{}` changed in {}",
            change.qualified_name, change.file
        ),
    }
}

fn symbol_change_evidence(change: &SymbolChange) -> Evidence {
    let mut evidence = Evidence::new(format!(
        "symbol change: {} in {}",
        change.qualified_name, change.file
    ));

    if let Some(span) = &change.before_span {
        evidence = evidence.with_span(span.clone());
    }

    if let Some(span) = &change.after_span {
        evidence = evidence.with_span(span.clone());
    }

    evidence
}

fn file_evidence(file: &FileAnalysis, message: String) -> Evidence {
    let mut evidence = Evidence::new(message);
    for symbol in &file.symbols {
        evidence = evidence.with_span(symbol.span.clone());
    }
    evidence
}

fn symbol_names(file: &FileAnalysis) -> String {
    if file.symbols.is_empty() {
        return "<none>".to_string();
    }

    file.symbols
        .iter()
        .map(|symbol| symbol.name.clone())
        .collect::<Vec<_>>()
        .join(", ")
}

fn summarize(
    change_set: &ChangeSet,
    clone_groups: &[CloneGroup],
    renames: &[RenamedFile],
    symbol_changes: &[SymbolChange],
    api_changes: &[SymbolChange],
) -> String {
    let mut parts = Vec::new();

    if !change_set.added.is_empty() {
        parts.push(count_phrase(
            change_set.added.len(),
            "added file",
            "added files",
        ));
    }

    if !change_set.modified.is_empty() {
        parts.push(count_phrase(
            change_set.modified.len(),
            "modified file",
            "modified files",
        ));
    }

    if !change_set.removed.is_empty() {
        parts.push(count_phrase(
            change_set.removed.len(),
            "removed file",
            "removed files",
        ));
    }

    if !renames.is_empty() {
        parts.push(count_phrase(renames.len(), "renamed file", "renamed files"));
    }

    if !clone_groups.is_empty() {
        parts.push(count_phrase(
            clone_groups.len(),
            "exact clone group",
            "exact clone groups",
        ));
    }

    if !symbol_changes.is_empty() {
        parts.push(count_phrase(
            symbol_changes.len(),
            "symbol change",
            "symbol changes",
        ));
    }

    if !api_changes.is_empty() {
        parts.push(count_phrase(api_changes.len(), "api change", "api changes"));
    }

    if parts.is_empty() {
        return "No relevant changes detected.".to_string();
    }

    format!("Detected {}.", join_phrases(&parts))
}

fn count_phrase(count: usize, singular: &str, plural: &str) -> String {
    if count == 1 {
        format!("1 {singular}")
    } else {
        format!("{count} {plural}")
    }
}

fn join_phrases(parts: &[String]) -> String {
    match parts {
        [] => String::new(),
        [one] => one.clone(),
        [first, second] => format!("{first} and {second}"),
        _ => {
            let mut rendered = parts[..parts.len() - 1].join(", ");
            rendered.push_str(", and ");
            rendered.push_str(parts.last().unwrap());
            rendered
        }
    }
}

fn format_percent(value: f32) -> String {
    format!("{:.0}%", value * 100.0)
}

const REPORT_HTML_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Architecture View</title>
  <style>
    :root {
      color-scheme: dark;
      --bg: #0b1020;
      --panel: #111a33;
      --panel-2: #151f3a;
      --text: #e5ecff;
      --muted: #95a4cc;
      --line: #253458;
      --good: #4fd1c5;
      --warn: #f6ad55;
      --bad: #fc8181;
      --accent: #8ab4ff;
    }
    * { box-sizing: border-box; }
    body {
      margin: 0;
      background: radial-gradient(circle at top, #152044 0, var(--bg) 55%);
      color: var(--text);
      font-family: Inter, ui-sans-serif, system-ui, -apple-system, Segoe UI, sans-serif;
    }
    header {
      display: flex;
      flex-wrap: wrap;
      gap: 12px;
      justify-content: space-between;
      align-items: center;
      padding: 20px 24px;
      border-bottom: 1px solid var(--line);
      background: rgba(10, 16, 32, 0.75);
      backdrop-filter: blur(8px);
      position: sticky;
      top: 0;
      z-index: 10;
    }
    h1 { margin: 0; font-size: 20px; }
    .subtitle { color: var(--muted); font-size: 14px; margin-top: 4px; }
    .toolbar { display: flex; gap: 8px; align-items: center; flex-wrap: wrap; }
    .button {
      background: var(--panel-2);
      color: var(--text);
      border: 1px solid var(--line);
      border-radius: 999px;
      padding: 10px 14px;
      cursor: pointer;
    }
    .button.active { border-color: var(--accent); box-shadow: 0 0 0 1px var(--accent) inset; }
    .file-input { color: var(--muted); }
    main { padding: 24px; max-width: 1400px; margin: 0 auto; }
    .grid {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
      gap: 16px;
      margin-top: 16px;
    }
    .panel {
      background: rgba(17, 26, 51, 0.88);
      border: 1px solid var(--line);
      border-radius: 18px;
      padding: 18px;
      box-shadow: 0 12px 32px rgba(0, 0, 0, 0.18);
      min-width: 0;
    }
    .panel h2, .panel h3 { margin: 0 0 12px; }
    .panel h2 { font-size: 18px; }
    .panel h3 { font-size: 14px; color: var(--muted); text-transform: uppercase; letter-spacing: 0.08em; }
    .stats { display: flex; flex-wrap: wrap; gap: 8px; margin-bottom: 14px; }
    .pill {
      padding: 6px 10px;
      border-radius: 999px;
      background: rgba(255,255,255,0.04);
      border: 1px solid var(--line);
      font-size: 12px;
      color: var(--muted);
    }
    .pill.good { color: var(--good); }
    .pill.warn { color: var(--warn); }
    .pill.bad { color: var(--bad); }
    .component-card {
      background: linear-gradient(180deg, rgba(21,31,58,0.95), rgba(17,26,51,0.92));
      border: 1px solid var(--line);
      border-radius: 16px;
      padding: 14px;
      margin-bottom: 12px;
      min-width: 0;
      overflow-wrap: anywhere;
    }
    .component-title { display:flex; justify-content:space-between; gap:12px; align-items:flex-start; flex-wrap: wrap; min-width: 0; }
    .component-title strong { font-size: 14px; min-width: 0; overflow-wrap: anywhere; }
    .component-title code { color: var(--accent); white-space: pre-wrap; overflow-wrap: anywhere; }
    .small { font-size: 12px; color: var(--muted); overflow-wrap: anywhere; word-break: break-word; }
    .bars { display: flex; gap: 8px; margin-top: 10px; flex-wrap: wrap; }
    .bar {
      min-width: 74px;
      padding: 8px 10px;
      border-radius: 12px;
      background: rgba(255,255,255,0.04);
      border: 1px solid var(--line);
    }
    .bar .num { font-size: 20px; font-weight: 700; }
    .bar .label { font-size: 11px; color: var(--muted); }
    .list { margin: 0; padding-left: 18px; color: var(--text); }
    .list li { margin: 6px 0; }
    .hidden { display: none; }
    .table {
      width: 100%;
      border-collapse: collapse;
      margin-top: 10px;
      overflow: hidden;
      border-radius: 14px;
    }
    .table th, .table td {
      border-bottom: 1px solid var(--line);
      padding: 10px 12px;
      text-align: left;
      vertical-align: top;
      font-size: 13px;
      overflow-wrap: anywhere;
      word-break: break-word;
    }
    .table th { color: var(--muted); font-weight: 600; }
    .kpi { display:inline-flex; gap: 8px; align-items: center; }
    .swatch { width: 10px; height: 10px; border-radius: 999px; display:inline-block; }
    .risk-high { background: var(--bad); }
    .risk-med { background: var(--warn); }
    .risk-low { background: var(--good); }
    .callout { color: var(--muted); font-size: 13px; }
    .empty { color: var(--muted); border: 1px dashed var(--line); border-radius: 14px; padding: 18px; }
    input[type="file"] { color: var(--muted); }
    code { background: rgba(255,255,255,0.06); padding: 2px 6px; border-radius: 8px; white-space: pre-wrap; overflow-wrap: anywhere; }
  </style>
</head>
<body>
  <header>
    <div>
      <h1>Architecture View</h1>
      <div class="subtitle" id="header-subtitle">Base vs head compare with timeline</div>
    </div>
    <div class="toolbar">
      <button class="button active" data-tab="compare">Compare</button>
      <button class="button" data-tab="timeline">Timeline</button>
      <label class="file-input">Load bundles
        <input id="bundle-input" type="file" multiple accept="application/json,.json">
      </label>
    </div>
  </header>
  <main>
    <div class="panel" id="summary-panel"></div>
    <section id="compare-view"></section>
    <section id="timeline-view" class="hidden"></section>
  </main>

  <script id="analysis-data" type="application/json">__REPORT_JSON__</script>
  <script>
    const initialReport = JSON.parse(document.getElementById('analysis-data').textContent);
    const state = { tab: 'compare', reports: [initialReport] };

    const escapeHtml = (value) => String(value)
      .replaceAll('&', '&amp;')
      .replaceAll('<', '&lt;')
      .replaceAll('>', '&gt;')
      .replaceAll('"', '&quot;')
      .replaceAll("'", '&#39;');

    function byId(report, side) {
      const items = report[side + '_components'] || [];
      return Object.fromEntries(items.map((item) => [item.id, item]));
    }

    function componentIds(report) {
      const ids = new Set();
      for (const side of ['base', 'head']) {
        for (const item of (report[side + '_components'] || [])) ids.add(item.id);
      }
      return [...ids].sort();
    }

    function renderPills(report) {
      const apiRemoved = (report.findings || []).filter((f) => f.code === 'api.removed').length;
      const apiChanged = (report.findings || []).filter((f) => f.code === 'api.changed').length;
      const cloneDrift = (report.findings || []).filter((f) => f.code === 'clone.drift').length;
      return `
        <div class="stats">
          <span class="pill">${escapeHtml(report.base_ref || 'base')}</span>
          <span class="pill">→</span>
          <span class="pill">${escapeHtml(report.head_ref || 'head')}</span>
          <span class="pill ${apiRemoved ? 'bad' : 'good'}">API removed: ${apiRemoved}</span>
          <span class="pill ${apiChanged ? 'warn' : 'good'}">API changed: ${apiChanged}</span>
          <span class="pill ${cloneDrift ? 'warn' : 'good'}">Clone drift: ${cloneDrift}</span>
        </div>
      `;
    }

    function renderComponentCard(title, component, baseComponent) {
      const delta = baseComponent ? component.file_count - baseComponent.file_count : component.file_count;
      const riskClass = delta > 0 ? 'warn' : delta < 0 ? 'good' : 'good';
      return `
        <div class="component-card" data-component-id="${escapeHtml(component.id)}">
          <div class="component-title">
            <strong><code>${escapeHtml(component.id)}</code></strong>
            <span class="pill ${riskClass}">${delta >= 0 ? '+' : ''}${delta} files</span>
          </div>
          <div class="small">${escapeHtml(title)} · ${component.file_count} files</div>
          <div class="small">${escapeHtml((component.files || []).join(', '))}</div>
        </div>
      `;
    }

    function renderCompare() {
      const report = state.reports[0];
      const base = byId(report, 'base');
      const head = byId(report, 'head');
      const ids = componentIds(report);

      const diffCards = ids.map((id) => renderComponentCard('Component', head[id] || base[id], base[id] || head[id])).join('');

      const findings = (report.findings || []).slice(0, 12).map((finding) => `
        <div class="component-card">
          <div class="component-title">
            <strong>${escapeHtml(finding.title)}</strong>
            <span class="pill">${escapeHtml(finding.code)}</span>
          </div>
          <div class="small">${escapeHtml(finding.detail)}</div>
        </div>
      `).join('');

      document.getElementById('compare-view').innerHTML = `
        <div class="grid">
          <div class="panel">
            <h2>Base</h2>
            <p class="callout">${escapeHtml(report.base_ref || 'base')}</p>
            ${(report.base_components || []).map((component) => renderComponentCard('Base component', component, null)).join('') || '<div class="empty">No base components.</div>'}
          </div>
          <div class="panel">
            <h2>Head</h2>
            <p class="callout">${escapeHtml(report.head_ref || 'head')}</p>
            ${(report.head_components || []).map((component) => renderComponentCard('Head component', component, base[component.id] || null)).join('') || '<div class="empty">No head components.</div>'}
          </div>
          <div class="panel">
            <h2>Findings</h2>
            ${findings || '<div class="empty">No findings.</div>'}
          </div>
        </div>
      `;
    }

    function renderTimeline() {
      const reports = state.reports;
      if (!reports.length) {
        document.getElementById('timeline-view').innerHTML = '<div class="empty">Load one or more JSON reports to see the timeline.</div>';
        return;
      }

      const ids = [...new Set(reports.flatMap((report) => (report.head_components || []).map((component) => component.id)))].sort();
      const headerCells = reports.map((report, index) => `<th>Snapshot ${index + 1}<br><span class="small">${escapeHtml(report.head_ref || 'head')}</span></th>`).join('');
      const rows = ids.map((id) => {
        const cells = reports.map((report) => {
          const component = (report.head_components || []).find((item) => item.id === id);
          if (!component) return '<td class="small">—</td>';
          return `<td><span class="kpi"><span class="swatch ${component.file_count > 1 ? 'risk-med' : 'risk-low'}"></span><code>${escapeHtml(id)}</code></span><br><span class="small">${component.file_count} files</span></td>`;
        }).join('');
        return `<tr><th>${escapeHtml(id)}</th>${cells}</tr>`;
      }).join('');

      document.getElementById('timeline-view').innerHTML = `
        <div class="panel">
          <h2>Timeline</h2>
          <p class="callout">Load multiple report JSON files to compare how components change across snapshots.</p>
          <table class="table">
            <thead><tr><th>Component</th>${headerCells}</tr></thead>
            <tbody>${rows || '<tr><td colspan="2" class="small">No components.</td></tr>'}</tbody>
          </table>
        </div>
      `;
    }

    function renderSummary() {
      const report = state.reports[0];
      document.getElementById('summary-panel').innerHTML = `
        <h3>Current Snapshot</h3>
        <div class="small">${escapeHtml(report.summary || '')}</div>
        ${renderPills(report)}
      `;
      document.getElementById('header-subtitle').textContent = `${report.base_components?.length || 0} base components → ${report.head_components?.length || 0} head components`;
    }

    function render() {
      renderSummary();
      renderCompare();
      renderTimeline();
      document.getElementById('compare-view').classList.toggle('hidden', state.tab !== 'compare');
      document.getElementById('timeline-view').classList.toggle('hidden', state.tab !== 'timeline');
      document.querySelectorAll('[data-tab]').forEach((button) => {
        button.classList.toggle('active', button.dataset.tab === state.tab);
      });
    }

    document.querySelectorAll('[data-tab]').forEach((button) => {
      button.addEventListener('click', () => {
        state.tab = button.dataset.tab;
        render();
      });
    });

    document.getElementById('bundle-input').addEventListener('change', async (event) => {
      const files = [...event.target.files || []];
      const loaded = [];
      for (const file of files) {
        const text = await file.text();
        loaded.push(JSON.parse(text));
      }
      state.reports = [initialReport, ...loaded];
      state.tab = 'timeline';
      render();
    });

    render();
  </script>
</body>
</html>"#;
