# Analyzer User Guide

Analyze C# and TypeScript diffs, detect clone drift, and produce grounded explanations for humans or PRs.

## Quick Start

Build the CLI, then run it against a repo:

You can optionally repeat `--path <repo-relative-path>` to limit analysis to specific areas of a monorepo.

### Linux / WSL

```bash
source "$HOME/.cargo/env"   # or /root/.cargo/env if building as root
cargo build --release -p analyzer-cli
./target/release/analyzer-cli --repo /path/to/repo --base main --head HEAD --format html > report.html
./target/release/analyzer-cli --repo /path/to/repo --base main --head HEAD --path samples/tetris-demo --format html > report.html
```

### Windows

```powershell
cargo build --release -p analyzer-cli
.\target\release\analyzer-cli.exe --repo C:\path\to\repo --base main --head HEAD --format html > report.html
```

### Artifacts

- `artifacts/analyzer-cli-linux-x86_64`
- `artifacts/analyzer-cli-windows-x86_64.exe`
- `report.html` is generated when you run `--format html` and redirect the output

## What It Does

- Parses C# and TypeScript source with tree-sitter.
- Compares `base` and `head` git refs.
- Reports file changes, symbol changes, API changes, clone groups, and clone drift.
- Exports `markdown`, `json`, `pr-comment`, and `html` output.

## Base Vs Head

Think of `base` as "before" and `head` as "after".

- `base` is the snapshot you trust as the starting point.
- `head` is the snapshot you want to inspect.
- The analyzer compares the two refs and explains what changed.

Common examples:

```bash
# last commit vs current commit
--base HEAD~1 --head HEAD
```

```bash
# main branch vs your current branch
--base main --head HEAD
```

```bash
# two branches
--base dev --head feature/my-change
```

```bash
# two specific commits
--base a1b2c3d --head d4e5f6g
```

For PR review, use:

- `base` = the target branch (`main`, `dev`, etc.)
- `head` = your feature branch tip

Notes:

- `HEAD` means the commit currently checked out.
- `HEAD~1` means the previous commit.
- Any valid git ref works: branch names, tags, or commit SHAs.
- This tool compares git refs, not uncommitted working-tree changes.

## Supported Environments

- WSL / Linux x86_64
- Native Windows x86_64

## Run From Source

### WSL / Linux

Install Rust, then source your Cargo env:

```bash
source "$HOME/.cargo/env"
```

If you built as `root`, use:

```bash
source "/root/.cargo/env"
```

Then go to the repo and run it:

```bash
cd /path/to/vOpenAI
cargo run -p analyzer-cli -- \
  --repo /path/to/your/repo \
  --base main \
  --head HEAD \
  --format pr-comment
```

### Native Windows

Install the Rust MSVC toolchain and Visual Studio Build Tools with the C++ workload.

Then run:

```powershell
cd C:\path\to\vOpenAI
cargo run -p analyzer-cli -- `
  --repo C:\path\to\your\repo `
  --base main `
  --head HEAD `
  --format pr-comment
```

If Windows complains about missing runtime DLLs, install the Microsoft Visual C++ Redistributable.

## Verify The Tool

```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## Output Modes

- `markdown` - full human-readable report
- `json` - machine-readable report
- `pr-comment` - compact GitHub PR comment format
- `html` - self-contained browser viewer with Compare and Timeline tabs

## Example Commands

Compare one commit:

```bash
cargo run -p analyzer-cli -- \
  --repo /path/to/repo \
  --base HEAD~1 \
  --head HEAD \
  --format json
```

Compare a branch to main:

```bash
cargo run -p analyzer-cli -- \
  --repo /path/to/repo \
  --base main \
  --head HEAD \
  --format pr-comment
```

Generate the HTML viewer and save it to a file:

```bash
cargo run -p analyzer-cli -- \
  --repo /path/to/repo \
  --base main \
  --head HEAD \
  --format html > report.html
```

Then open `report.html` in your browser.

## Build Release Binaries

### WSL / Linux Release Build

```bash
cargo build --release -p analyzer-cli
```

Artifact:

```bash
target/release/analyzer-cli
```

Package it for another WSL or Linux machine:

```bash
tar -C target/release -czf analyzer-cli-linux-x86_64.tar.gz analyzer-cli
sha256sum analyzer-cli-linux-x86_64.tar.gz > analyzer-cli-linux-x86_64.tar.gz.sha256
```

### Native Windows Release Build

```powershell
cargo build --release -p analyzer-cli
```

Artifact:

```powershell
target\release\analyzer-cli.exe
```

Package it for another Windows machine:

```powershell
Compress-Archive -Path target\release\analyzer-cli.exe -DestinationPath analyzer-cli-windows-x86_64.zip
Get-FileHash analyzer-cli-windows-x86_64.zip -Algorithm SHA256
```

## Distribute To Another Machine

### Another WSL Or Linux Machine

1. Build the Linux release binary.
2. Copy `analyzer-cli-linux-x86_64.tar.gz` to the target machine.
3. Extract it:

```bash
tar -xzf analyzer-cli-linux-x86_64.tar.gz
```

4. Run the binary directly or move it into `~/.local/bin` or `/usr/local/bin`.

### Native Windows Machine

1. Build the Windows release binary.
2. Copy `analyzer-cli-windows-x86_64.zip` to the target machine.
3. Extract it.
4. Run `analyzer-cli.exe` from the extracted folder or add that folder to `PATH`.

## Distribution Notes

- The tool is a single CLI binary; no service or database is required.
- The CLI accepts repo paths and git refs, plus optional repeatable `--path` filters for monorepo scoping.
- `pr-comment` output is meant to be pasted into a PR comment.
- Generated and vendor code is ignored by default.

## What The Findings Mean

- `api.removed` - highest risk, likely breaking change
- `api.changed` - medium/high risk, signature or body changed
- `api.added` - lower risk, new public surface
- `clone.drift` - copy/paste code has diverged

## Notes

- Files with parser recovery errors no longer abort the whole run; they may be partially analyzed instead.
- If a file is still unreadable, the analyzer skips the bad nodes and keeps going.
- This workspace currently focuses on C# and TypeScript.

## Troubleshooting

- If you run this in WSL as a different user, use that user's `~/.cargo/env`.
- If native Windows linking fails, make sure the MSVC Build Tools and Windows SDK are installed.
- If a repo path uses Windows form (`C:\...`) inside WSL, convert it to `/mnt/c/...`.
