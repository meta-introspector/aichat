use anyhow::{Result, Context};
use chrono::Utc;

use oauth2::basic::BasicClient;

use oauth2::{ClientId, ClientSecret, AuthUrl, TokenUrl, RefreshToken, TokenResponse};
use reqwest;

use crate::auth::credential_store::{Credentials};
use crate::auth::oauth_split::oauth_authenticator_struct::OAuthAuthenticator;

impl OAuthAuthenticator {
    pub async fn refresh_token(&self, refresh_token: RefreshToken) -> Result<Credentials> {
        let client = BasicClient::new(
            ClientId::new(self.config.client_id.clone()),
        )
        .set_client_secret(ClientSecret::new(self.config.client_secret.clone()))
        .set_auth_uri(AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?)
        .set_token_uri(TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?);

        let token_response = client
            .exchange_refresh_token(&refresh_token)
            .request_async(&reqwest::Client::new())
            .await
            .context("Failed to refresh token")?;

        Ok(Credentials {
            access_token: token_response.access_token().secret().to_string(),
            refresh_token: token_response.refresh_token().map(|t| t.secret().to_string()),
            token_type: Some(format!("{:?}", token_response.token_type())),
            expiry_date: token_response.expires_in().map(|d| Utc::now().timestamp() + d.as_secs() as i64),
            user_info: None,
        })
    }
}
