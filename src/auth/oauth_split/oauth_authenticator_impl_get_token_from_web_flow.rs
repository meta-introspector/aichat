use anyhow::{Result, Context};
use std::io::{BufReader, Write, BufRead};
use std::net::TcpListener;
use oauth2::url::Url;
use chrono::{Utc, Duration};
use std::borrow::Cow;

use oauth2::basic::BasicClient;

use oauth2::{AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl};
use reqwest;
use open;

use crate::auth::credential_store::{Credentials};
use crate::auth::oauth_split::constants::{OAUTH_CLIENT_ID, OAUTH_CLIENT_SECRET, OAUTH_SCOPE};
use crate::auth::oauth_split::oauth_config::OAuthConfig;
use crate::auth::oauth_split::oauth_authenticator_struct::OAuthAuthenticator;
use crate::auth::oauth_split::find_available_port::find_available_port;
use crate::auth::oauth_split::user_info::UserInfo;

impl OAuthAuthenticator {
    pub async fn get_token_from_web_flow(&self) -> Result<Credentials> {
        let client = BasicClient::new(
            ClientId::new(self.config.client_id.clone()),
        )
        .set_client_secret(ClientSecret::new(self.config.client_secret.clone()))
        .set_auth_uri(AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?)
        .set_token_uri(TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?);

                                                                                let (pkce_code_challenge, pkce_code_verifier) =
            oauth2::PkceCodeChallenge::new_random_sha256();

        let port = find_available_port()?;
        let redirect_uri = format!("http://localhost:{}", port);

        let (authorize_url, csrf_state) = client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(OAUTH_SCOPE.iter().map(|s| Scope::new(s.to_string())))
            .set_pkce_challenge(pkce_code_challenge)
            .set_redirect_uri(Cow::Owned(RedirectUrl::new(redirect_uri.clone()).unwrap()))
            .url();

        println!("Open this URL in your browser:\n{}\n", authorize_url);
        open::that(authorize_url.as_str()).context("Failed to open browser")?;

        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
        let mut stream = listener.incoming().flatten().next().context("Listener terminated without accepting a connection")?;

        let mut reader = BufReader::new(&stream);
        let mut request_line = String::new();
        reader.read_line(&mut request_line)?;

        let redirect_url_path = request_line.split_whitespace().nth(1).context("Invalid redirect URL")?;
        let url = Url::parse(&format!("http://localhost:{}{}", port, redirect_url_path))?;

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
            .set_redirect_uri(Cow::Owned(RedirectUrl::new(redirect_uri.clone()).unwrap()))
            .request_async(&reqwest::Client::new())
            .await
            .context("Failed to exchange code for token")?;

        let access_token = token_response.access_token().secret().to_string();

        // Fetch and cache user info
        if let Err(e) = self.fetch_and_cache_user_info(&access_token).await {
            eprintln!("Failed to fetch and cache user info: {}", e);
        }

        let message = "Authentication successful! You can close this tab.";
        let response = format!(
            "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
            message.len(),
            message
        );
        stream.write_all(response.as_bytes())?;

        Ok(Credentials {
            access_token,
            refresh_token: token_response.refresh_token().map(|t| t.secret().to_string()),
            token_type: Some(format!("{:?}", token_response.token_type())),
            expiry_date: token_response.expires_in().map(|d| Utc::now().timestamp() + d.as_secs() as i64),
            user_info: None, // User info is handled by fetch_and_cache_user_info
        })
    }
}