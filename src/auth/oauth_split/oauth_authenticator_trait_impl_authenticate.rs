use anyhow::{Result, Context};
use chrono::{Utc, Duration};

use oauth2::RefreshToken;

use crate::auth::credential_store::{Credentials};
use crate::auth::Authenticator;
use crate::auth::oauth_split::oauth_authenticator_struct::OAuthAuthenticator;
use async_trait::async_trait;

#[async_trait]
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
        use crate::auth::oauth_split::oauth_client_setup;
        use crate::auth::oauth_split::web_auth_flow;
        use crate::auth::oauth_split::web_flow_token_exchange;
        use crate::auth::oauth_split::constants::OAUTH_SCOPE;
        use oauth2::{CsrfToken, RedirectUrl, Scope, TokenResponse, AuthUrl, TokenUrl};
        use std::borrow::Cow;

        let (client, pkce_code_challenge, pkce_code_verifier) =
            oauth_client_setup::setup_oauth_client(
                self.config.client_id.clone(),
                self.config.client_secret.clone(),
            )?;

        let redirect_uri_from_config = self.config.redirect_uri.clone().unwrap_or_else(|| "http://localhost:37387/".to_string());
        let port = crate::auth::oauth_split::find_available_port::find_available_port(&redirect_uri_from_config)?;
        let redirect_uri = format!("http://localhost:{}/", port);

        let (authorize_url, csrf_state) = client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(OAUTH_SCOPE.iter().map(|s| Scope::new(s.to_string())))
            .set_pkce_challenge(pkce_code_challenge)
            .set_redirect_uri(Cow::Owned(RedirectUrl::new(redirect_uri.clone()).unwrap()))
            .url();

        let (code, received_state) = web_auth_flow::run_web_auth_flow(
            authorize_url,
            csrf_state.clone(),
            port,
        ).await?;

        if received_state.secret() != csrf_state.secret() {
            return Err(anyhow::anyhow!("State mismatch. Possible CSRF attack"));
        }

        let token_response = web_flow_token_exchange::exchange_code_for_token(
            client,
            code,
            pkce_code_verifier,
            redirect_uri.clone(),
        )
        .await?;

        let access_token = token_response.access_token().secret().to_string();

        // Fetch and cache user info
        if let Err(e) = self.fetch_and_cache_user_info(&access_token).await {
            eprintln!("Failed to fetch and cache user info: {}", e);
        }

        let new_creds = Credentials {
            access_token,
            refresh_token: token_response.refresh_token().map(|t| t.secret().to_string()),
            token_type: Some(format!("{:?}", token_response.token_type())),
            expiry_date: token_response.expires_in().map(|d| Utc::now().timestamp() + d.as_secs() as i64),
            user_info: None, // User info is handled by fetch_and_cache_user_info
        };
        self.credential_store.write_credentials(&new_creds)?;
        Ok(new_creds.access_token)
    }
}
