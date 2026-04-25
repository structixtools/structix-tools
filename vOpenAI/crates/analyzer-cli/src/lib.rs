use std::path::PathBuf;

use analyzer_core::analysis::analyze_repo;
use analyzer_core::report::Report;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputFormat {
    Markdown,
    Json,
    PrComment,
    Html,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliError {
    pub message: String,
}

impl CliError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for CliError {}

pub fn run<I, S>(args: I) -> Result<String, CliError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut repo = None;
    let mut base = None;
    let mut head = None;
    let mut format = OutputFormat::Markdown;
    let mut paths = Vec::new();

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_ref() {
            "--repo" => repo = Some(next_value(&mut iter, "--repo")?),
            "--base" => base = Some(next_value(&mut iter, "--base")?),
            "--head" => head = Some(next_value(&mut iter, "--head")?),
            "--format" => format = parse_format(&mut iter)?,
            "--path" => paths.push(next_value(&mut iter, "--path")?),
            "-h" | "--help" => {
                return Ok(help_text());
            }
            other => return Err(CliError::new(format!("unknown argument: {other}"))),
        }
    }

    let repo = repo.ok_or_else(|| CliError::new("missing --repo"))?;
    let base = base.ok_or_else(|| CliError::new("missing --base"))?;
    let head = head.ok_or_else(|| CliError::new("missing --head"))?;

    let report = analyze_repo(PathBuf::from(repo), &base, &head, &paths)
        .map_err(|err| CliError::new(err.to_string()))?;

    render_report(&report, format)
}

fn next_value<I, S>(iter: &mut I, flag: &str) -> Result<String, CliError>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    iter.next()
        .map(|value| value.as_ref().to_string())
        .ok_or_else(|| CliError::new(format!("missing value for {flag}")))
}

fn help_text() -> String {
    [
        "analyzer-cli --repo <path> --base <ref> --head <ref> [--format markdown|json|pr-comment|html] [--path <repo-relative-path>]...",
        "",
        "Prints a report for the diff between two git refs.",
    ]
    .join("\n")
}

fn parse_format<I, S>(iter: &mut I) -> Result<OutputFormat, CliError>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    match next_value(iter, "--format")?.as_str() {
        "markdown" => Ok(OutputFormat::Markdown),
        "json" => Ok(OutputFormat::Json),
        "pr-comment" => Ok(OutputFormat::PrComment),
        "html" => Ok(OutputFormat::Html),
        other => Err(CliError::new(format!("unsupported format: {other}"))),
    }
}

fn render_report(report: &Report, format: OutputFormat) -> Result<String, CliError> {
    match format {
        OutputFormat::Markdown => Ok(report.markdown()),
        OutputFormat::Json => report
            .json()
            .map_err(|err| CliError::new(format!("failed to render JSON: {err}"))),
        OutputFormat::PrComment => Ok(report.pr_comment()),
        OutputFormat::Html => Ok(report.html()),
    }
}
