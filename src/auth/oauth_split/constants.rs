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



pub fn load_oauth_config(config: &crate::config::Config) -> Result<(String, String)> {
    let redirect_uri = config.oauth.redirect_uri.clone().unwrap_or_else(|| "http://localhost:37387/".to_string());
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

pub fn get_oauth_redirect_uri(config: &crate::config::Config) -> String {
    config.oauth.redirect_uri.clone().unwrap_or_else(|| "http://localhost:37387/".to_string())
}

pub fn load_gemini_oauth_config(config: &crate::config::Config) -> Result<crate::auth::oauth_split::oauth_config::OAuthConfig> {
    let path = crate::config::Config::config_dir().join("clients").join("gemini").join("client_secret.json");
    let content = fs::read_to_string(path)
        .map_err(|e| anyhow!("Failed to read Gemini client secret file: {}", e))?;
    let json: Value = serde_json::from_str(&content)
        .map_err(|e| anyhow!("Failed to parse Gemini client secret JSON: {}", e))?;

    let client_id = json["web"]["client_id"]
        .as_str()
        .ok_or_else(|| anyhow!("client_id not found in Gemini JSON"))?
        .to_string();
    let client_secret = json["web"]["client_secret"]
        .as_str()
        .ok_or_else(|| anyhow!("client_secret not found in Gemini JSON"))?
        .to_string();
    let scopes = json["web"]["scopes"]
        .as_array()
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect());

    Ok(crate::auth::oauth_split::oauth_config::OAuthConfig {
        client_id,
        client_secret,
        redirect_uri: Some(config.oauth.redirect_uri.clone().unwrap_or_else(|| "http://localhost:37387/".to_string())),
        scopes,
    })
}
