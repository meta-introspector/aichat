use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const GEMINI_DIR: &str = ".zos";
const CREDENTIAL_FILENAME: &str = "oauth_creds.json";

use super::oauth_split::user_info::UserInfo;

use std::fmt;

#[derive(Clone, Deserialize, Serialize)]
pub struct Credentials {
    // These fields should match the structure of oauth_creds.json
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub token_type: Option<String>,
    pub expiry_date: Option<i64>,
    pub user_info: Option<UserInfo>,
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials")
            .field("access_token", &"********************") // Redact access token
            .field("refresh_token", &self.refresh_token.as_ref().map(|_| "********************")) // Redact refresh token
            .field("token_type", &self.token_type)
            .field("expiry_date", &self.expiry_date)
            .field("user_info", &self.user_info)
            .finish()
    }
}

#[derive(Debug)]
pub struct CredentialStore {
    credentials_path: PathBuf,
}

impl CredentialStore {
    pub fn new() -> Result<Self> {
        let home_dir = dirs::home_dir().context("Could not find home directory")?;
        let credentials_path = home_dir.join(GEMINI_DIR).join(CREDENTIAL_FILENAME);
        Ok(Self { credentials_path })
    }

    pub fn read_credentials(&self) -> Result<Credentials> {
        let contents = fs::read_to_string(&self.credentials_path)
            .with_context(|| format!("Could not read credentials from {}", self.credentials_path.display()))?;
        let parsed_json: Credentials = serde_json::from_str(&contents)
            .with_context(|| format!("Could not parse JSON from {}", self.credentials_path.display()))?;
        Ok(parsed_json)
    }

    pub fn write_credentials(&self, credentials: &Credentials) -> Result<()> {
        let parent_dir = self.credentials_path.parent().context("Invalid credentials path")?;
        fs::create_dir_all(parent_dir).with_context(|| format!("Could not create directory {}", parent_dir.display()))?;

        let cred_string = serde_json::to_string_pretty(credentials)
            .context("Could not serialize credentials")?;
        fs::write(&self.credentials_path, cred_string)
            .with_context(|| format!("Could not write credentials to {}", self.credentials_path.display()))?;
        Ok(())
    }

    pub fn clear_credentials(&self) -> Result<()> {
        if self.credentials_path.exists() {
            fs::remove_file(&self.credentials_path)
                .with_context(|| format!("Could not remove credentials file {}", self.credentials_path.display()))?;
        }
        Ok(())
    }
}