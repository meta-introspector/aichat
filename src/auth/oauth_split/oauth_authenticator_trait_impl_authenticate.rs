use anyhow::{Result, Context};
use chrono::{Utc, Duration};

use oauth2::RefreshToken;

use crate::auth::credential_store::{Credentials};
use crate::auth::Authenticator;
use crate::auth::oauth_split::oauth_authenticator_struct::OAuthAuthenticator;

impl Authenticator for OAuthAuthenticator {
    async fn authenticate(&self) -> Result<String> {
        // 1. Try to load cached credentials
        if let Ok(mut creds) = self.credential_store.read_credentials() {
            // Check if expired and refresh
            if let Some(expiry) = creds.expiry_date {
                if Utc::now().timestamp() >= expiry {
                    if let Some(refresh_token) = creds.refresh_token.clone() {
                        match self.refresh_token(RefreshToken::new(refresh_token)).await {
                            Ok(new_creds) => {
                                self.credential_store.write_credentials(&new_creds)?;
                                return Ok(new_creds.access_token);
                            }
                            Err(e) => eprintln!("Failed to refresh token: {}", e),
                        }
                    }
                }
            }
            return Ok(creds.access_token);
        }

        // 2. If no credentials or refresh fails, initiate interactive login
        let new_creds = self.get_token_from_web_flow().await?;
        self.credential_store.write_credentials(&new_creds)?;
        Ok(new_creds.access_token)
    }
}
