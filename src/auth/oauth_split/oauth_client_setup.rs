use anyhow::Result;
use oauth2::{AuthUrl, ClientId, ClientSecret, TokenUrl, PkceCodeChallenge, PkceCodeVerifier};

use crate::auth::oauth_split::constants::GoogleOAuthClient;

pub fn setup_oauth_client(
    client_id: String,
    client_secret: String,
) -> Result<(GoogleOAuthClient, PkceCodeChallenge, PkceCodeVerifier)> {
    let client = GoogleOAuthClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
    )
    .set_auth_uri(AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?)
    .set_token_uri(TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?);

    let (pkce_code_challenge, pkce_code_verifier) =
        oauth2::PkceCodeChallenge::new_random_sha256();

    Ok((client, pkce_code_challenge, pkce_code_verifier))
}
