use crate::manifest::schema::ChangeManifest;

const MODEL: &str = "claude-sonnet-4-6";
const SYSTEM: &str = "You are a code change analyst. Summarize the structural code changes in \
the JSON manifest. Focus on: what changed, API surface changes (signature changes), potential \
impact on callers. Be concrete, reference specific names. Under 300 words.";

pub fn narrate_changes(manifest: &ChangeManifest, api_key: &str) -> anyhow::Result<String> {
    let client = reqwest::blocking::Client::new();
    let body = serde_json::json!({
        "model": MODEL,
        "max_tokens": 1024,
        "system": SYSTEM,
        "messages": [{
            "role": "user",
            "content": format!(
                "Summarize these structural code changes:\n\n```json\n{}\n```",
                serde_json::to_string_pretty(manifest)?
            )
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
    let text = resp["content"][0]["text"]
        .as_str()
        .unwrap_or("(no response)")
        .to_string();
    Ok(text)
}
