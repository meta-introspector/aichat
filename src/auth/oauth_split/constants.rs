use std::fs;
use serde_json::Value;
use anyhow::{Result, anyhow};
use oauth2::basic::BasicClient;
use oauth2::{EndpointSet, EndpointNotSet};

pub type GoogleOAuthClient = BasicClient<
    EndpointSet, // HasAuthUrl
    EndpointNotSet, // HasDeviceAuthUrl
    EndpointNotSet, // HasIntrospectionUrl
    EndpointNotSet, // HasRevocationUrl
    EndpointSet, // HasTokenUrl
>;

pub const OAUTH_SCOPE: &[&str] = &[
    "https://www.googleapis.com/auth/cloud-platform",
    "https://www.googleapis.com/auth/userinfo.email",
    "https://www.googleapis.com/auth/userinfo.profile",
];

pub fn load_oauth_config() -> Result<(String, String)> {
    let path = "/data/data/com.termux/files/home/storage/github/aichat/clients/zos-solfunmeme/client_secret_637389221985-i3evf22mp7ubfrqkvinv70r379mie3nt.apps.googleusercontent.com.json";
    let content = fs::read_to_string(path)
        .map_err(|e| anyhow!("Failed to read client secret file: {}", e))?;
    let json: Value = serde_json::from_str(&content)
        .map_err(|e| anyhow!("Failed to parse client secret JSON: {}", e))?;

    let client_id = json["web"]["client_id"]
        .as_str()
        .ok_or_else(|| anyhow!("client_id not found in JSON"))?
        .to_string();
    let client_secret = json["web"]["client_secret"]
        .as_str()
        .ok_or_else(|| anyhow!("client_secret not found in JSON"))?
        .to_string();

    Ok((client_id, client_secret))
}
