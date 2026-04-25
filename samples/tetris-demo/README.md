# Tetris Demo Sample

A small showcase app for the Structix tools:

- **Frontend:** TypeScript
- **Backend:** C# / ASP.NET Core minimal API
- **Persistence:** SQLite

This sample is meant to generate meaningful reports for both tools:

- structural frontend changes in TypeScript
- API and backend changes in C#
- HTML and PR-comment outputs with real repo content

## Layout

```text
samples/tetris-demo/
├─ backend/
│  └─ TetrisDemo.Api/
└─ frontend/
```

## Run locally

### 1. Build the frontend

```bash
cd samples/tetris-demo/frontend
npm install
npm run build
```

This compiles TypeScript into:

```text
samples/tetris-demo/backend/TetrisDemo.Api/wwwroot/assets/
```

### 2. Run the backend

```bash
cd ../backend/TetrisDemo.Api
dotnet run
```

Open:

```text
http://localhost:5000
```

or the HTTPS URL shown by ASP.NET Core.

## API

### Get high scores

```http
GET /api/highscores?limit=10
```

### Submit a high score

```http
POST /api/highscores
Content-Type: application/json

{
  "playerName": "MARIO",
  "score": 12000,
  "lines": 18,
  "level": 2
}
```

## Suggested demo workflow

This repo now supports repo-relative path filters, so you can analyze this sample while keeping it inside the main monorepo.

### Structix

```bash
cd vAnthropic
cargo run -- diff .. --from ecce58c --to f2123e8 --path samples/tetris-demo --html -o ../site/examples/tetris-structix-report.html
```

### Analyzer

```bash
cd vOpenAI
cargo run -p analyzer-cli -- --repo .. --base ecce58c --head f2123e8 --path samples/tetris-demo --format html > ../site/examples/tetris-analyzer-report.html
cargo run -p analyzer-cli -- --repo .. --base ecce58c --head f2123e8 --path samples/tetris-demo --format pr-comment > ../site/examples/tetris-analyzer-pr-comment.txt
```

### Generated samples checked into the repo

- [`../../site/examples/tetris-structix-report.html`](../../site/examples/tetris-structix-report.html)
- [`../../site/examples/tetris-analyzer-report.html`](../../site/examples/tetris-analyzer-report.html)
- [`../../site/examples/tetris-analyzer-pr-comment.txt`](../../site/examples/tetris-analyzer-pr-comment.txt)
