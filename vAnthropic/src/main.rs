pub mod parser;
pub mod manifest;
pub mod differ;
pub mod duplicates;
pub mod explainer;
pub mod git_reader;
pub mod html_report;
pub mod cli;

fn main() {
    if let Err(e) = cli::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
