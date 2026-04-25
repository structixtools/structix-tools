use crate::parser::types::CodeEntity;

const MODEL: &str = "claude-sonnet-4-6";
const SYSTEM: &str = "You are a code quality advisor. For each clone group, confirm if they \
are functionally equivalent and suggest a concrete refactoring strategy (extract shared \
function, use generics, etc.). Name a suggested shared abstraction. Be brief and actionable. \
One paragraph per clone group.";

pub fn advise_duplicates(
    clone_groups: &[Vec<&CodeEntity>],
    api_key: &str,
) -> anyhow::Result<String> {
    let mut content = String::new();
    for (i, group) in clone_groups.iter().enumerate() {
        content.push_str(&format!("\n### Clone Group {}\n", i + 1));
        for e in group {
            let snippet = if e.source.len() > 400 { &e.source[..400] } else { &e.source };
            content.push_str(&format!(
                "\n**{}** ({}:{})\n```\n{}\n```\n",
                e.name, e.file_path, e.start_line, snippet
            ));
        }
    }
    let client = reqwest::blocking::Client::new();
    let body = serde_json::json!({
        "model": MODEL,
        "max_tokens": 1500,
        "system": SYSTEM,
        "messages": [{
            "role": "user",
            "content": format!("Advise on these duplicate code groups:{}", content)
        }]
    });
    let resp: serde_json::Value = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&body)
        .send()?
        .json()?;
    Ok(resp["content"][0]["text"]
        .as_str()
        .unwrap_or("(no response)")
        .to_string())
}
