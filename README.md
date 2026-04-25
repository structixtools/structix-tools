# Structix Tools Monorepo

Open-source structural code analysis tools for TypeScript and C#.

This repository hosts two Rust codebases:

- **vAnthropic / Structix** — structural diffing, duplicate detection, HTML reports, and optional Claude-powered explanations.
- **vOpenAI / Analyzer** — base/head diff analysis, API-change reporting, clone drift detection, PR-comment output, and HTML reports.

## Projects

### 1. Structix
Path: [`vAnthropic/`](./vAnthropic)

Best for:
- structural diffs between git refs
- duplicate detection
- prompt generation for AI review
- HTML and JSON export

### 2. Analyzer
Path: [`vOpenAI/`](./vOpenAI)

Best for:
- PR-oriented repo analysis
- API change tracking
- clone drift detection
- Markdown, JSON, PR-comment, and HTML output

## Landing Page

A static landing page is available at:

- [`site/index.html`](./site/index.html)

Open it in a browser to view:
- feature comparisons
- OS download/install guidance
- detailed manuals
- open-source positioning and repo layout

## Manuals

- [`docs/structix-manual.md`](./docs/structix-manual.md)
- [`docs/analyzer-manual.md`](./docs/analyzer-manual.md)

## Real Example Reports

Generated sample outputs based on the in-repo TypeScript + C# Tetris demo refactor range (`f2123e8..7561b1e`) are available at:

- [`site/examples/tetris-structix-report.html`](./site/examples/tetris-structix-report.html)
- [`site/examples/tetris-analyzer-report.html`](./site/examples/tetris-analyzer-report.html)
- [`site/examples/tetris-analyzer-pr-comment.txt`](./site/examples/tetris-analyzer-pr-comment.txt)

## Repo Layout

```text
.
├─ README.md
├─ docs/
│  ├─ structix-manual.md
│  └─ analyzer-manual.md
├─ site/
│  ├─ index.html
│  └─ styles.css
├─ vAnthropic/
└─ vOpenAI/
```

## Suggested GitHub Setup

If you want this as a public open-source repo, a good setup would be:

- repo name: `structix-tools`
- topics: `rust`, `tree-sitter`, `typescript`, `csharp`, `code-analysis`, `git`, `developer-tools`
- GitHub Releases for downloadable binaries
- Actions workflow for building:
  - Windows x86_64
  - Linux x86_64
  - macOS Apple Silicon / Intel

## Next Good Steps

Completed in this repo:

1. ✅ GitHub Actions release builds are configured in [`.github/workflows/release.yml`](./.github/workflows/release.yml), including Windows, Linux, macOS Intel, and macOS Apple Silicon assets plus SHA256 files.
2. ✅ The landing page now includes versioned release download links and release-link templates in [`site/index.html`](./site/index.html).
3. ✅ GitHub Pages deployment is configured in [`.github/workflows/pages.yml`](./.github/workflows/pages.yml).
4. ✅ Example outputs are now linked from the landing page in [`site/examples/`](./site/examples/).
5. ✅ MIT license files are included for public distribution.

Remaining manual setup:

- Replace the placeholder GitHub repo string (`OWNER/structix-tools`) in `site/index.html` after you publish the repo.
- Create your first tagged release, e.g. `v0.1.0`, so the download buttons resolve to real assets.
- If you want actual screenshots in addition to sample outputs, capture them from the generated HTML reports and add them under `site/`.
