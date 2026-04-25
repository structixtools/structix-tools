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

1. Add GitHub Actions release builds.
2. Add versioned binary downloads to the landing page.
3. Move the site to GitHub Pages / Netlify / Cloudflare Pages.
4. Add screenshots and example reports.
5. Add LICENSE files if you want fully public distribution.
