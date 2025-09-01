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
use crate::auth::oauth_split::constants::{OAUTH_SCOPE};
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
        let redirect_uri = format!("http://localhost:{}/", port); // Added trailing slash for consistency
        println!("Listening on port: {}", port);
        println!("Redirect URI: {}", redirect_uri);

        let (authorize_url, csrf_state) = client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(OAUTH_SCOPE.iter().map(|s| Scope::new(s.to_string())))
            .set_pkce_challenge(pkce_code_challenge)
            .set_redirect_uri(Cow::Owned(RedirectUrl::new(redirect_uri.clone()).unwrap()))
            .url();

        println!("Open this URL in your browser:\n{}\n", authorize_url);
        open::that(authorize_url.as_str()).context("Failed to open browser")?;

        println!("Attempting to bind TcpListener to 127.0.0.1:{}", port);
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
        println!("TcpListener bound successfully.");

        let token_response = loop {
            println!("Waiting for incoming connection...");
            let mut stream = listener.incoming().flatten().next().context("Listener terminated without accepting a connection")?;
            println!("Incoming connection received.");

            let mut reader = BufReader::new(&stream);
            let mut request_line = String::new();
            reader.read_line(&mut request_line)?;
            println!("Request line: {}", request_line);

            let redirect_url_path = request_line.split_whitespace().nth(1).context("Invalid redirect URL")?;
            let url = Url::parse(&format!("http://localhost:{}{}", port, redirect_url_path))?;
            println!("Parsed URL: {}", url);

            let code = url
                .query_pairs()
                .find(|(key, _)| key == "code")
                .map(|(_, code)| AuthorizationCode::new(code.into_owned()));

            let state = url
                .query_pairs()
                .find(|(key, _)| key == "state")
                .map(|(_, state)| CsrfToken::new(state.into_owned()));

            if let (Some(code), Some(state)) = (code, state) {
                if state.secret() != csrf_state.secret() {
                    let msg = "State mismatch. Possible CSRF attack.";
                    let response = format!(
                        "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                        msg.len(),
                        msg
                    );
                    stream.write_all(response.as_bytes())?;
                    continue; // Continue to next iteration to wait for a valid request
                } else {
                    println!("Attempting to exchange code for token...");
                    match client
                        .exchange_code(code)
                        .set_pkce_verifier(pkce_code_verifier) // pkce_code_verifier is moved here, so it's only used once
                        .set_redirect_uri(Cow::Owned(RedirectUrl::new(redirect_uri.clone()).unwrap()))
                        .request_async(&reqwest::Client::new())
                        .await
                    {
                        Ok(token_response) => {
                            println!("Token exchange successful.");
                            let msg = "Authentication successful! You can close this tab.";
                            let response = format!(
                                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                                msg.len(),
                                msg
                            );
                            stream.write_all(response.as_bytes())?;
                            break token_response;
                        }
                        Err(e) => {
                            eprintln!("Failed to exchange code for token: {}", e);
                            let msg = "Failed to exchange code for token.";
                            let response = format!(
                                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                                msg.len(),
                                msg
                            );
                            stream.write_all(response.as_bytes())?;
                            continue; // Continue to next iteration on error
                        }
                    }
                }
            } else {
                let msg = "No code or state found in redirect URL.";
                let response = format!(
                    "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                    msg.len(),
                    msg
                );
                stream.write_all(response.as_bytes())?;
                continue; // Continue to next iteration if no code/state
            };
        };

        let access_token = token_response.access_token().secret().to_string();

        // Fetch and cache user info
        if let Err(e) = self.fetch_and_cache_user_info(&access_token).await {
            eprintln!("Failed to fetch and cache user info: {}", e);
        }

        Ok(Credentials {
            access_token,
            refresh_token: token_response.refresh_token().map(|t| t.secret().to_string()),
            token_type: Some(format!("{:?}", token_response.token_type())),
            expiry_date: token_response.expires_in().map(|d| Utc::now().timestamp() + d.as_secs() as i64),
            user_info: None, // User info is handled by fetch_and_cache_user_info
        })
    }
}
