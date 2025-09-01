use anyhow::{Result, Context};
use oauth2::basic::BasicTokenResponse;
use oauth2::{AuthorizationCode, PkceCodeVerifier, RedirectUrl};
use std::borrow::Cow;

use crate::auth::oauth_split::constants::GoogleOAuthClient;

pub async fn exchange_code_for_token(
    client: GoogleOAuthClient,
    code: AuthorizationCode,
    pkce_code_verifier: PkceCodeVerifier,
    redirect_uri: String,
) -> Result<BasicTokenResponse> {
    println!("Attempting to exchange code for token...");
    let token_response = client
        .exchange_code(code)
        .set_pkce_verifier(pkce_code_verifier)
        .set_redirect_uri(Cow::Owned(RedirectUrl::new(redirect_uri.clone()).unwrap()))
        .request_async(&reqwest::Client::new())
        .await
        .context("Failed to exchange code for token")?;
    println!("Token exchange successful.");
    Ok(token_response)
}
