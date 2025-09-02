use std::fs;
use serde_json::Value;
use anyhow::{Result, anyhow};

pub fn load_meta_client_oauth_config(_config: &crate::config::Config) -> Result<(String, String)> {
    let path = crate::config::Config::config_dir().join("clients").join("meta-client").join("client_secret.json");
    let content = fs::read_to_string(path)
        .map_err(|e| anyhow!("Failed to read Meta Client secret file: {}", e))?;
    let json: Value = serde_json::from_str(&content)
        .map_err(|e| anyhow!("Failed to parse Meta Client secret JSON: {}", e))?;

    let client_id = json["web"]["client_id"]
        .as_str()
        .ok_or_else(|| anyhow!("client_id not found in Meta Client JSON"))?
        .to_string();
    let client_secret = json["web"]["client_secret"]
        .as_str()
        .ok_or_else(|| anyhow!("client_secret not found in Meta Client JSON"))?
        .to_string();

    Ok((client_id, client_secret))
}