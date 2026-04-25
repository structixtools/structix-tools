# Analyzer Manual

Analyzer is a Rust workspace for structured comparison of C# and TypeScript repositories across git refs.

## What Analyzer Does

Analyzer compares a **base** ref and a **head** ref, then produces a report tailored for humans, automation, and pull-request workflows.

It focuses on:
- file changes
- symbol changes
- API changes
- clone groups
- clone drift
- report rendering in multiple formats

## Supported Languages

- TypeScript
- C#

## Workspace Layout

- `crates/analyzer-core` — analysis engine
- `crates/analyzer-cli` — command-line interface

This split makes the project easier to maintain and extend.

## Installation

### Build from source

```bash
cd vOpenAI
cargo build --release -p analyzer-cli
```

Binary output:
- Linux / WSL: `target/release/analyzer-cli`
- Windows: `target/release/analyzer-cli.exe`

## Core CLI Usage

```bash
cargo run -p analyzer-cli -- \
  --repo /path/to/repo \
  --base main \
  --head HEAD \
  --format markdown
```

## Required arguments

- `--repo <path>`
- `--base <ref>`
- `--head <ref>`

Optional:

- `--path <repo-relative-path>` to restrict analysis to one or more parts of a monorepo; repeat the flag as needed

## Output formats

- `markdown`
- `json`
- `pr-comment`
- `html`

---

## Base and Head Model

Think of:
- **base** = the trusted starting point
- **head** = the newer state you want to inspect

Examples:

```bash
--base HEAD~1 --head HEAD
--base main --head HEAD
--base dev --head feature/my-change
```

---

## Main Findings

### File changes
Tracks which files changed between refs.

### Symbol changes
Tracks structural changes to symbols discovered from source parsing.

### API changes
Useful for identifying possible breaking changes or interface evolution.

### Clone groups
Finds duplicate or highly similar code structures.

### Clone drift
Highlights places where originally similar code has diverged over time.
This is especially useful for maintenance and refactoring planning.

---

## Output Modes in Practice

### Markdown
Use for human-readable reports in local review.

```bash
cargo run -p analyzer-cli -- --repo . --base main --head HEAD --format markdown
cargo run -p analyzer-cli -- --repo . --base main --head HEAD --path samples/tetris-demo --format markdown
```

### JSON
Use for machines, CI pipelines, or downstream tooling.

```bash
cargo run -p analyzer-cli -- --repo . --base main --head HEAD --format json
```

### PR Comment
Use for compact GitHub review summaries.

```bash
cargo run -p analyzer-cli -- --repo . --base main --head HEAD --format pr-comment
```

### HTML
Use for a richer browser-based report.

```bash
cargo run -p analyzer-cli -- --repo . --base main --head HEAD --format html > report.html
cargo run -p analyzer-cli -- --repo . --base main --head HEAD --path samples/tetris-demo --format html > report.html
```

---

## Strengths

- strong separation between CLI and analysis engine
- PR-friendly output
- explicit API-change reporting
- clone drift support
- clean workspace architecture

## Best Fit

Use Analyzer when your main need is structured repo comparison for engineering reviews, CI, or PR workflows where API-level change awareness matters.
