# Contributing

Thanks for your interest in Structix Tools.

## Development setup

### Structix
```bash
cd vAnthropic
cargo test
cargo doc --no-deps
```

### Analyzer
```bash
cd vOpenAI
cargo test
cargo doc --no-deps
```

## Contribution guidelines

- keep changes focused and reviewable
- prefer small pull requests
- include docs updates when behavior changes
- add tests when possible
- keep CLI output stable unless intentionally changed

## Commit suggestions

Examples:
- `feat(site): improve landing page downloads section`
- `fix(structix): correct moved entity detection`
- `docs(analyzer): clarify pr-comment format`

## Pull requests

Please include:
- what changed
- why it changed
- how to test it
- screenshots for UI/site changes
