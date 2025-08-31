use anyhow::{Result, Context};

use crate::auth::oauth_split::user_info::UserInfo;
use crate::auth::credential_store::CredentialStore;
use crate::auth::oauth_split::oauth_authenticator_struct::OAuthAuthenticator;

impl OAuthAuthenticator {
    pub async fn fetch_and_cache_user_info(&self, access_token: &str) -> Result<()> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .error_for_status()?;

        let user_info: UserInfo = response.json().await?;

        // Update the stored credentials with user info
        if let Ok(mut creds) = self.credential_store.read_credentials() {
            creds.user_info = Some(user_info);
            self.credential_store.write_credentials(&creds)?;
        }
        Ok(())
    }
}
