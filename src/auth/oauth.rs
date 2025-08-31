use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use std::io::{BufReader, Write, BufRead};
use std::net::TcpListener;
use url::Url;
use chrono::{Utc, Duration};

use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl, RefreshToken};
use open;

use crate::auth::credential_store::{CredentialStore, Credentials};
use crate::auth::Authenticator;

const OAUTH_CLIENT_ID: &str = "never.apps.googleusercontent.com";
const OAUTH_CLIENT_SECRET: &str = "inamillionyears";
const OAUTH_SCOPE: &[&str] = &[
    "https://www.googleapis.com/auth/cloud-platform",
    "https://www.googleapis.com/auth/userinfo.email",
    "https://www.googleapis.com/auth/userinfo.profile",
];

// Placeholder for OAuth configuration
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    // Add other OAuth related configs as needed
}

pub struct OAuthAuthenticator {
    config: OAuthConfig,
    credential_store: Arc<CredentialStore>,
}

impl OAuthAuthenticator {
    pub fn new(config: OAuthConfig, credential_store: Arc<CredentialStore>) -> Self {
        Self { config, credential_store }
    }

    async fn refresh_token(&self, refresh_token: RefreshToken) -> Result<Credentials> {
        let client = BasicClient::new(
            ClientId::new(self.config.client_id.clone()),
            Some(ClientSecret::new(self.config.client_secret.clone())),
        )
        .set_auth_uri(AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?)
        .set_token_uri(TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?);

        let token_response = client
            .exchange_refresh_token(&refresh_token)
            .request_async(async_http_client)
            .await
            .context("Failed to refresh token")?;

        Ok(Credentials {
            access_token: token_response.access_token().secret().to_string(),
            refresh_token: token_response.refresh_token().map(|t| t.secret().to_string()),
            token_type: token_response.token_type().map(|t| t.to_string()),
            expiry_date: token_response.expires_in().map(|d| Utc::now().timestamp() + d.as_secs() as i64),
        })
    }

    async fn get_token_from_web_flow(&self) -> Result<Credentials> {
        let client = BasicClient::new(
            ClientId::new(self.config.client_id.clone()),
            Some(ClientSecret::new(self.config.client_secret.clone())),
        )
        .set_auth_uri(AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?)
        .set_token_uri(TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?);

        let (pkce_code_verifier, pkce_code_challenge) =
            oauth2::PkceCodeChallenge::new_random_sha256();

        let (authorize_url, csrf_state) = client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(OAUTH_SCOPE.iter().map(|s| Scope::new(s.to_string())))
            .set_pkce_challenge(pkce_code_challenge)
            .url();

        println!("Open this URL in your browser:\n{}\n", authorize_url);
        open::that(authorize_url.as_str()).context("Failed to open browser")?;

        let listener = TcpListener::bind("127.0.0.1:8080")?;
        let mut stream = listener.incoming().flatten().next().context("Listener terminated without accepting a connection")?;

        let mut reader = BufReader::new(&stream);
        let mut request_line = String::new();
        reader.read_line(&mut request_line)?;

        let redirect_url = request_line.split_whitespace().nth(1).context("Invalid redirect URL")?;
        let url = Url::parse(&format!("http://localhost:8080{}", redirect_url))?;

        let code = url
            .query_pairs()
            .find(|(key, _)| key == "code")
            .map(|(_, code)| AuthorizationCode::new(code.into_owned()))
            .context("No code found in redirect URL")?;

        let state = url
            .query_pairs()
            .find(|(key, _)| key == "state")
            .map(|(_, state)| CsrfToken::new(state.into_owned()))
            .context("No state found in redirect URL")?;

        if state.secret() != csrf_state.secret() {
            return Err(anyhow::anyhow!("State mismatch. Possible CSRF attack"));
        }

        let token_response = client
            .exchange_code(code)
            .set_pkce_verifier(pkce_code_verifier)
            .request_async(async_http_client)
            .await
            .context("Failed to exchange code for token")?;

        let message = "Authentication successful! You can close this tab.";
        let response = format!(
            "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\nਤਾ",
            message.len(),
            message
        );
        stream.write_all(response.as_bytes())?;

        Ok(Credentials {
            access_token: token_response.access_token().secret().to_string(),
            refresh_token: token_response.refresh_token().map(|t| t.secret().to_string()),
            token_type: token_response.token_type().map(|t| t.to_string()),
            expiry_date: token_response.expires_in().map(|d| Utc::now().timestamp() + d.as_secs() as i64),
        })
    }
}

#[async_trait::async_trait]
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
