use std::collections::HashMap;
use clap::{Parser, Subcommand};
use crate::parser::types::CodeEntity;
use crate::parser::typescript::TypeScriptParser;
use crate::parser::csharp::CSharpParser;
use crate::differ::entity_diff::diff_entities;
use crate::manifest::schema::{ChangeKind, ChangeManifest};
use crate::duplicates::hasher::find_exact_clones;
use crate::duplicates::fingerprint::find_structural_clones;
use crate::git_reader::get_files_at_ref;

#[derive(Parser)]
#[command(name = "structix", about = "Structural code analysis and diff tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show structural changes between two git refs
    Diff {
        /// Path to the git repository
        #[arg(default_value = ".")]
        repo_path: String,
        /// Base ref
        #[arg(long, default_value = "HEAD~1")]
        from: String,
        /// Target ref
        #[arg(long, default_value = "HEAD")]
        to: String,
        /// Use Claude to narrate changes (requires ANTHROPIC_API_KEY)
        #[arg(long)]
        explain: bool,
        /// Output raw JSON manifest
        #[arg(long)]
        json: bool,
        /// Output a ready-to-paste prompt for Claude chat (no API key needed)
        #[arg(long)]
        prompt: bool,
        /// Output a self-contained HTML report (open in browser)
        #[arg(long)]
        html: bool,
        /// Save output to file (default: timestamped filename, applies to --prompt, --json, --html)
        #[arg(long, short = 'o')]
        output: Option<String>,
        /// Restrict analysis to one or more repo-relative paths
        #[arg(long = "path")]
        paths: Vec<String>,
    },
    /// Detect duplicate code in the repository working tree
    Duplicates {
        /// Path to search
        #[arg(default_value = ".")]
        repo_path: String,
        /// Use Claude to advise on refactoring (requires ANTHROPIC_API_KEY)
        #[arg(long)]
        explain: bool,
        /// Output a ready-to-paste prompt for Claude chat (no API key needed)
        #[arg(long)]
        prompt: bool,
        /// Save output to file (default: timestamped filename, applies to --prompt)
        #[arg(long, short = 'o')]
        output: Option<String>,
        /// Restrict analysis to one or more repo-relative paths
        #[arg(long = "path")]
        paths: Vec<String>,
    },
}

fn extract_all(files: &HashMap<String, String>) -> Vec<CodeEntity> {
    let ts_parser = TypeScriptParser::new();
    let cs_parser = CSharpParser::new();
    let mut entities = Vec::new();
    for (path, source) in files {
        if path.ends_with(".ts") || path.ends_with(".tsx") {
            entities.extend(ts_parser.extract(path, source));
        } else if path.ends_with(".cs") {
            entities.extend(cs_parser.extract(path, source));
        }
    }
    entities
}

fn walk_dir(root: &str, path_filters: &[String]) -> Vec<(String, String)> {
    let mut results = Vec::new();
    fn recurse(dir: &str, root: &std::path::Path, path_filters: &[String], results: &mut Vec<(String, String)>) {
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name.starts_with('.') || matches!(name, "node_modules" | "target") {
                continue;
            }
            if path.is_dir() {
                recurse(path.to_str().unwrap_or(""), root, path_filters, results);
            } else if let Some(s) = path.to_str() {
                if s.ends_with(".ts") || s.ends_with(".tsx") || s.ends_with(".cs") {
                    let relative = path.strip_prefix(root).ok().and_then(|p| p.to_str()).unwrap_or(s).replace('\\', "/");
                    if !crate::git_reader::path_matches_filters(&relative, path_filters) {
                        continue;
                    }
                    if let Ok(content) = std::fs::read_to_string(s) {
                        results.push((relative, content));
                    }
                }
            }
        }
    }
    let root_path = std::path::Path::new(root);
    recurse(root, root_path, path_filters, &mut results);
    results
}

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Diff { repo_path, from, to, explain, json, prompt, html, output, paths } => {
            let before_files = get_files_at_ref(&repo_path, &from, &paths)?;
            let after_files = get_files_at_ref(&repo_path, &to, &paths)?;
            let before = extract_all(&before_files);
            let after = extract_all(&after_files);
            let changes = diff_entities(&before, &after);
            let mut files_set = std::collections::HashSet::new();
            for c in &changes {
                files_set.insert(c.file_path.clone());
            }
            let files_affected: Vec<String> = files_set.into_iter().collect();
            let manifest = ChangeManifest {
                from_ref: from.clone(),
                to_ref: to.clone(),
                changes,
                files_affected,
                summary: None,
            };

            if json {
                let content = serde_json::to_string_pretty(&manifest)?;
                write_to_file(&content, output, "json")?;
                return Ok(());
            }

            if prompt {
                let content = build_diff_prompt(&manifest)?;
                write_to_file(&content, output, "md")?;
                return Ok(());
            }

            if html {
                let content = crate::html_report::build_html_report(&manifest);
                write_to_file(&content, output, "html")?;
                return Ok(());
            }

            println!("\nStructural diff: {} → {}", from, to);
            println!("Files affected: {}", manifest.files_affected.len());
            println!("Total changes: {}\n", manifest.changes.len());

            for change in &manifest.changes {
                let marker = match change.kind {
                    ChangeKind::Added => "+",
                    ChangeKind::Removed => "-",
                    ChangeKind::Modified => "~",
                    ChangeKind::Moved => "→",
                    ChangeKind::Renamed => "↩",
                };
                println!(
                    "  [{}] {} {} ({})",
                    marker, change.entity_kind, change.name, change.file_path
                );
                if change.kind == ChangeKind::Moved {
                    if let Some(old) = &change.old_file_path {
                        println!("      from: {}", old);
                        println!("      to:   {}", change.file_path);
                    }
                }
                if let (Some(b), Some(a)) = (&change.before_signature, &change.after_signature) {
                    if b != a {
                        println!("      before: {}", b);
                        println!("      after:  {}", a);
                    }
                }
            }

            if explain && !manifest.changes.is_empty() {
                let api_key = std::env::var("ANTHROPIC_API_KEY")
                    .map_err(|_| anyhow::anyhow!("ANTHROPIC_API_KEY not set"))?;
                println!("\n--- AI Summary ---");
                println!("{}", crate::explainer::narrator::narrate_changes(&manifest, &api_key)?);
            }
        }

        Commands::Duplicates { repo_path, explain, prompt, output, paths } => {
            let file_pairs = walk_dir(&repo_path, &paths);
            let files: HashMap<String, String> = file_pairs.into_iter().collect();
            let entities = extract_all(&files);

            let exact = find_exact_clones(&entities);
            let structural = find_structural_clones(&entities);

            println!("\nExact clones: {} group(s)", exact.len());
            for group in &exact {
                let desc: Vec<String> = group
                    .iter()
                    .map(|e| format!("{} ({}:{})", e.name, e.file_path, e.start_line))
                    .collect();
                println!("  {}", desc.join(" ↔ "));
            }

            println!("\nStructural clones: {} group(s)", structural.len());
            for group in &structural {
                let desc: Vec<String> = group
                    .iter()
                    .map(|e| format!("{} ({}:{})", e.name, e.file_path, e.start_line))
                    .collect();
                println!("  {}", desc.join(" ↔ "));
            }

            if prompt {
                let content = build_duplicates_prompt(&exact, &structural);
                write_to_file(&content, output, "md")?;
                return Ok(());
            }

            if explain && (!exact.is_empty() || !structural.is_empty()) {
                let api_key = std::env::var("ANTHROPIC_API_KEY")
                    .map_err(|_| anyhow::anyhow!("ANTHROPIC_API_KEY not set"))?;
                let all_groups: Vec<Vec<&CodeEntity>> =
                    exact.iter().chain(structural.iter()).cloned().collect();
                println!("\n--- AI Refactoring Advice ---");
                println!(
                    "{}",
                    crate::explainer::dedup_advisor::advise_duplicates(&all_groups, &api_key)?
                );
            }
        }
    }
    Ok(())
}

/// Format current UTC time as YYYY-MM-DDTHH-MM-SS (safe for filenames).
fn timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let s = secs % 60;
    let m = (secs / 60) % 60;
    let h = (secs / 3600) % 24;
    let days = secs / 86400; // days since 1970-01-01
    // Compute calendar date from days
    let (year, month, day) = days_to_ymd(days);
    format!("{:04}-{:02}-{:02}T{:02}-{:02}-{:02}", year, month, day, h, m, s)
}

fn days_to_ymd(mut days: u64) -> (u64, u64, u64) {
    let mut year = 1970u64;
    loop {
        let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
        let days_in_year = if leap { 366 } else { 365 };
        if days < days_in_year { break; }
        days -= days_in_year;
        year += 1;
    }
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let month_days: &[u64] = if leap {
        &[31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        &[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut month = 1u64;
    for &md in month_days {
        if days < md { break; }
        days -= md;
        month += 1;
    }
    (year, month, days + 1)
}

/// Write content to a file, printing the path to stderr. Returns the path used.
fn write_to_file(content: &str, output: Option<String>, ext: &str) -> anyhow::Result<String> {
    let path = output.unwrap_or_else(|| format!("structix-{}.{}", timestamp(), ext));
    std::fs::write(&path, content)?;
    eprintln!("Saved to: {}", path);
    Ok(path)
}

fn build_diff_prompt(manifest: &ChangeManifest) -> anyhow::Result<String> {
    let mut out = String::new();
    out.push_str(&format!(
        "I ran Structix on my repo and got the following structural diff ({} → {}).\n\
         Please summarize what changed, highlight any API surface changes (signature changes), \
         and flag any potential impact on callers.\n\n",
        manifest.from_ref, manifest.to_ref
    ));

    if manifest.changes.is_empty() {
        out.push_str("No structural changes detected.\n");
        return Ok(out);
    }

    out.push_str(&format!(
        "**{} change(s) across {} file(s)**\n\n",
        manifest.changes.len(),
        manifest.files_affected.len()
    ));

    for change in &manifest.changes {
        let marker = match change.kind {
            ChangeKind::Added => "[+]",
            ChangeKind::Removed => "[-]",
            ChangeKind::Modified => "[~]",
            ChangeKind::Moved => "[→]",
            ChangeKind::Renamed => "[↩]",
        };
        out.push_str(&format!(
            "- {} **{}** `{}` ({})\n",
            marker, change.name, change.file_path, change.entity_kind
        ));
        if let (Some(b), Some(a)) = (&change.before_signature, &change.after_signature) {
            if b != a {
                out.push_str(&format!("  - before: `{}`\n  - after:  `{}`\n", b, a));
            }
        }
    }

    out.push_str("\n<details><summary>Full JSON manifest</summary>\n\n```json\n");
    out.push_str(&serde_json::to_string_pretty(manifest)?);
    out.push_str("\n```\n</details>\n");

    Ok(out)
}

fn build_duplicates_prompt(
    exact: &[Vec<&CodeEntity>],
    structural: &[Vec<&CodeEntity>],
) -> String {
    let mut out = String::new();
    out.push_str(
        "I ran Structix duplicate detection on my codebase. \
         For each clone group below, please: confirm if they are functionally equivalent, \
         suggest a concrete refactoring strategy, and name a shared abstraction.\n\n",
    );

    if exact.is_empty() && structural.is_empty() {
        out.push_str("No duplicates detected.\n");
        return out;
    }

    if !exact.is_empty() {
        out.push_str(&format!("## Exact clones ({} group(s))\n\n", exact.len()));
        for (i, group) in exact.iter().enumerate() {
            out.push_str(&format!("### Group {}\n", i + 1));
            for e in group {
                let snippet = if e.source.len() > 300 { &e.source[..300] } else { &e.source };
                out.push_str(&format!(
                    "**{}** (`{}` line {})\n```\n{}\n```\n\n",
                    e.name, e.file_path, e.start_line, snippet
                ));
            }
        }
    }

    if !structural.is_empty() {
        out.push_str(&format!("## Structural clones ({} group(s))\n\n", structural.len()));
        for (i, group) in structural.iter().enumerate() {
            out.push_str(&format!("### Group {}\n", i + 1));
            for e in group {
                let snippet = if e.source.len() > 300 { &e.source[..300] } else { &e.source };
                out.push_str(&format!(
                    "**{}** (`{}` line {})\n```\n{}\n```\n\n",
                    e.name, e.file_path, e.start_line, snippet
                ));
            }
        }
    }

    out
}
