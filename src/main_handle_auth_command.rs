use anyhow::{Context, Result};
use std::{fs, sync::Arc};

use crate::cli;
use crate::config::{Config, GlobalConfig};
use crate::auth::Authenticator;
use crate::auth::oauth_split::oauth_authenticator_struct::OAuthAuthenticator;
use crate::auth::oauth_split::oauth_config::OAuthConfig;
use crate::auth::credential_store::CredentialStore;

pub async fn handle_auth_command(command: cli::AuthSubcommands, config: GlobalConfig) -> Result<()> {
    match command {
        cli::AuthSubcommands::Login => {
            let (client_id, client_secret) =
                crate::auth::oauth_split::constants::load_oauth_config(&config.read())?;
            let oauth_config = OAuthConfig {
                client_id,
                client_secret,
                redirect_uri: Some(config.read().oauth.redirect_uri.clone().unwrap_or_else(|| "http://localhost:37387/".to_string())),
                scopes: None,
            };
            let credential_store = Arc::new(CredentialStore::new()?);
            let oauth_authenticator = OAuthAuthenticator::new(oauth_config, credential_store);

            match oauth_authenticator.authenticate().await {
                Ok(_) => println!("OAuth authentication successful."),
                Err(e) => eprintln!("OAuth Error: {}", e),
            }
        }
        cli::AuthSubcommands::ManageGoogleOAuth => {
            crate::main_handle_manage_google_oauth_command::handle_manage_google_oauth_command(config).await?;
        }
        cli::AuthSubcommands::ManageGoogleOAuthClient => {
            let (client_id, client_secret) =
                crate::auth::oauth_split::meta_client_constants::load_meta_client_oauth_config(&config.read())?;
            let oauth_config = OAuthConfig {
                client_id,
                client_secret,
                redirect_uri: Some(config.read().oauth.redirect_uri.clone().unwrap_or_else(|| "http://localhost:37387/".to_string())),
                scopes: None,
            };
            let credential_store = Arc::new(CredentialStore::new()?);
            let oauth_authenticator = OAuthAuthenticator::new(oauth_config, credential_store);

            match oauth_authenticator.authenticate().await {
                Ok(_) => println!("Meta Client OAuth authentication successful."),
                Err(e) => eprintln!("Meta Client OAuth Error: {}", e),
            }
        }
        cli::AuthSubcommands::GeminiLogin => {
            let oauth_config = crate::auth::oauth_split::constants::load_gemini_oauth_config(&config.read())?;
            let client_id = oauth_config.client_id.clone();
            let client_secret = oauth_config.client_secret.clone();
            let oauth_config = OAuthConfig {
                client_id,
                client_secret,
                redirect_uri: Some(config.read().oauth.redirect_uri.clone().unwrap_or_else(|| "http://localhost:37387/".to_string())),
                scopes: None,
            };
            let credential_store = Arc::new(CredentialStore::new()?);
            let oauth_authenticator = OAuthAuthenticator::new(oauth_config, credential_store);

            match oauth_authenticator.authenticate().await {
                Ok(_) => println!("Gemini OAuth authentication successful."),
                Err(e) => eprintln!("Gemini OAuth Error: {}", e),
            }
        }
        cli::AuthSubcommands::ImportSecrets { file, client } => {
            let dest_dir = Config::config_dir().join("clients").join(client);
            fs::create_dir_all(&dest_dir)
                .with_context(|| format!("Failed to create directory {}", dest_dir.display()))?;
            let dest_path = dest_dir.join("client_secret.json");
            fs::copy(&file, &dest_path)
                .with_context(|| format!("Failed to copy client secret from {} to {}", file.display(), dest_path.display()))?;
            println!("Successfully imported client secret to {}", dest_path.display());
        }
    }
    Ok(())
}
