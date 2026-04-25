# Structix

Structural code analysis CLI for TypeScript and C# codebases. Diffs code at the entity level (functions, classes, interfaces, methods) across git commits, and detects duplicate code — with optional AI narration via the Claude API.

## Build

Requires Rust 2021 edition.

```bash
cargo build --release
# Binary: target/release/structix
```

## Commands

### `diff` — Structural diff between two git refs

Compares the structural entities (functions, classes, methods, interfaces) between two git commits. Shows what was added, removed, modified, moved, or renamed — not line-by-line diffs.

```bash
structix diff [REPO_PATH] [OPTIONS]
```

| Option | Default | Description |
|--------|---------|-------------|
| `REPO_PATH` | `.` | Path to the git repository |
| `--from <REF>` | `HEAD~1` | Base git ref |
| `--to <REF>` | `HEAD` | Target git ref |
| `--json` | — | Output raw JSON manifest (saved to file) |
| `--prompt` | — | Output a ready-to-paste Claude chat prompt (saved to file) |
| `--html` | — | Output a self-contained interactive HTML report (open in browser) |
| `--explain` | — | Call Claude API to narrate changes (requires `ANTHROPIC_API_KEY`) |
| `-o, --output <FILE>` | timestamped | Custom output filename for `--json` / `--prompt` / `--html` |

**Examples:**

```bash
# Compare last two commits in current repo
structix diff

# Compare specific refs
structix diff /path/to/repo --from main --to feature-branch

# Save JSON manifest (writes structix-2026-03-26T14-22-05.json)
structix diff --json

# Save JSON to a specific file
structix diff --json -o changes.json

# Save a Claude chat prompt (writes structix-2026-03-26T14-22-05.md)
structix diff --prompt

# Open an interactive HTML report in your browser
structix diff --html
structix diff --from main --to feature-branch --html -o report.html

# Get AI narration inline (requires API key)
ANTHROPIC_API_KEY=sk-ant-... structix diff --explain
```

**Terminal output (no flags):**
```
Structural diff: HEAD~1 → HEAD
Files affected: 2
Total changes: 4

  [+] function createUser (src/api/users.ts)
  [~] method findAll (src/services/UserService.ts)
      before: (): Promise<User[]>
      after:  (filter?: Partial<User>): Promise<User[]>
  [-] interface LegacyUser (src/models/user.ts)
  [→] method Configure (src/new/Module.cs)
      from: src/old/Module.cs
      to:   src/new/Module.cs
```

Change markers: `[+]` added · `[-]` removed · `[~]` modified · `[→]` moved · `[↩]` renamed

---

### `duplicates` — Detect duplicate code

Scans the working directory for duplicate code at two levels:

- **Exact clones** — identical normalized source (comments stripped, whitespace collapsed)
- **Structural clones** — same AST shape with different variable names (e.g. two fetch-and-parse functions)

```bash
structix duplicates [REPO_PATH] [OPTIONS]
```

| Option | Default | Description |
|--------|---------|-------------|
| `REPO_PATH` | `.` | Path to search |
| `--prompt` | — | Output a ready-to-paste Claude chat prompt (saved to file) |
| `--explain` | — | Call Claude API for refactoring advice (requires `ANTHROPIC_API_KEY`) |
| `-o, --output <FILE>` | timestamped | Custom output filename for `--prompt` |

**Examples:**

```bash
# Scan current directory
structix duplicates

# Scan a specific path
structix duplicates /path/to/repo

# Save a Claude chat prompt with clone details
structix duplicates --prompt

# Get AI refactoring advice inline
ANTHROPIC_API_KEY=sk-ant-... structix duplicates --explain
```

**Terminal output:**
```
Exact clones: 1 group(s)
  validateEmail (src/auth/login.ts:12) ↔ validateEmail (src/auth/register.ts:8)

Structural clones: 2 group(s)
  fetchUser (src/api/users.ts:34) ↔ fetchOrder (src/api/orders.ts:21)
  mapToDto (src/users/mapper.ts:5) ↔ mapToDto (src/orders/mapper.ts:5)
```

---

## File Output

When using `--json`, `--prompt`, or `--html`, output is written to a file instead of stdout. This avoids terminal flooding and makes it easy to share or paste.

- **Default filename:** `structix-YYYY-MM-DDTHH-MM-SS.json` / `.md` / `.html`
- **Custom filename:** `-o my-output.html`
- **Path printed to stderr** so it doesn't interfere with piped output

---

## AI Features

Both `--explain` and `--prompt` give you Claude's analysis of the results. They differ in how Claude is invoked:

| | `--explain` | `--prompt` | `--html` |
|---|---|---|---|
| Requires API key | Yes | No | No |
| How it works | Calls Claude API directly | Writes a formatted prompt you paste into Claude chat | Generates a self-contained browser report |
| Output | Printed to terminal | Saved to a `.md` file | Saved to a `.html` file |

The HTML report includes a file explorer sidebar, color-coded entity cards with before/after source, and an SVG flow diagram showing which entities moved between files. It works offline — no server, no internet connection needed.

---

## Supported Languages

- TypeScript (`.ts`, `.tsx`)
- C# (`.cs`)

Detected entities: functions, methods, classes, interfaces, constructors, enums, properties.

---

## Distribution

Structix compiles to a single self-contained binary with no runtime dependencies. All TLS is handled by a pure-Rust implementation (no OpenSSL). The only external requirement at runtime is `git` being on the PATH.

---

### Option A — Copy a pre-built binary (fastest)

Build once, copy to the target machine. Works as long as the CPU architecture matches (almost always `x86_64`).

| Built on | Runs on |
|----------|---------|
| WSL2 / Ubuntu x86_64 | Any Linux x86_64, any other WSL2 distro |
| Native Linux x86_64 | Any Linux x86_64, WSL2 |
| Native Windows x86_64 | Native Windows x86_64 only |

```bash
# After cargo build --release, the binary is here:
target/release/structix          # Linux / WSL
target/release/structix.exe      # Windows
```

Copy it to any directory on `PATH` on the target machine, e.g. `/usr/local/bin/structix`.

---

### Option B — Build from source on the target machine

Install Rust via [rustup](https://rustup.rs), clone or copy the source, then build:

```bash
# Linux / WSL
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
cargo build --release

# Windows (PowerShell) — downloads and runs the rustup installer
winget install Rustlang.Rustup
# or download rustup-init.exe from https://rustup.rs
cargo build --release
```

No additional C libraries or system packages are needed.

---

### Option C — Cross-compile from WSL to Windows

Build a native Windows `.exe` from inside WSL2 without touching a Windows terminal.

**1. Install the Windows GNU target and cross-linker:**

```bash
rustup target add x86_64-pc-windows-gnu
sudo apt install gcc-mingw-w64-x86-64
```

**2. Build:**

```bash
cargo build --release --target x86_64-pc-windows-gnu
# Output: target/x86_64-pc-windows-gnu/release/structix.exe
```

**3. Copy the `.exe` to Windows and place it on the PATH**, for example:

```powershell
# In PowerShell, copy from WSL filesystem
Copy-Item \\wsl$\Ubuntu\root\...\structix.exe C:\Users\<you>\bin\structix.exe
```

> **Note:** The GNU cross-linker produces a binary that does not require the Visual C++ redistributable. It works on any 64-bit Windows 10/11 machine out of the box.

---

### Installing the binary

**Linux / WSL:**

```bash
sudo cp target/release/structix /usr/local/bin/
structix --help
```

Or without sudo, add `~/.local/bin` to your PATH:

```bash
mkdir -p ~/.local/bin
cp target/release/structix ~/.local/bin/
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**Windows (native):**

Create a folder for personal CLI tools (e.g. `C:\Users\<you>\bin`) and add it to your user PATH:

```powershell
# Add to PATH (run once in PowerShell)
[Environment]::SetEnvironmentVariable(
  "Path",
  "$env:USERPROFILE\bin;" + [Environment]::GetEnvironmentVariable("Path", "User"),
  "User"
)

# Copy the binary
New-Item -ItemType Directory -Force "$env:USERPROFILE\bin"
Copy-Item structix.exe "$env:USERPROFILE\bin\structix.exe"
```

Open a new terminal and run `structix --help` to confirm.

---

### Runtime requirements

| Requirement | Linux / WSL | Windows |
|-------------|-------------|---------|
| `git` on PATH | `sudo apt install git` | Git for Windows (includes git in PATH) |
| `ANTHROPIC_API_KEY` env var | Only for `--explain` flag | Only for `--explain` flag |

---

## How It Works

**Parsing:** [tree-sitter](https://tree-sitter.github.io/) builds an AST for each file. The walker extracts structural entities with their names, signatures, qualifiers (async, static, public…), parent class, and line ranges.

**Diffing:** Entities are matched by `kind::parent::name` across refs. A file change in the path signals a move; a signature or source change signals a modification.

**Exact clones:** Source is normalized (comments stripped, whitespace collapsed) and SHA-256 hashed. Matching hashes across different entities = exact clone.

**Structural clones:** The AST is walked and all identifier nodes are replaced with the token `ID`. The resulting token sequence is SHA-256 hashed. Matching hashes = same structure, different names.

**Git integration:** Uses `git ls-tree` and `git show` to read file contents at any ref without a working tree checkout.
