use anyhow::{bail, Result};
use clap::Parser;
use inquire::Text;
use parking_lot::RwLock;
use simplelog::{format_description, ConfigBuilder, LevelFilter, SimpleLogger, WriteLogger};
use std::{env, process, sync::Arc};

use crate::cli;
use crate::config::{Config, GlobalConfig, WorkingMode, CODE_ROLE, EXPLAIN_SHELL_ROLE, SHELL_ROLE, TEMP_SESSION_NAME};
use crate::render::render_error;
use crate::repl::Repl;
use crate::utils::*;
use crate::auth::{Authenticator, ApiKeyAuthenticator};
use crate::auth::oauth_split::oauth_authenticator_struct::OAuthAuthenticator;
use crate::auth::oauth_split::oauth_config::OAuthConfig;
use crate::auth::credential_store::CredentialStore;
use crate::auth::oauth_split::constants;

pub async fn handle_auth_command(command: cli::AuthSubcommands, config: GlobalConfig) -> Result<()> {
    match command {
        cli::AuthSubcommands::Login => {
            let (client_id, client_secret) =
                crate::auth::oauth_split::constants::load_oauth_config(&config.read())?;
            let oauth_config = OAuthConfig {
                client_id,
                client_secret,
                redirect_uri: Some(config.read().oauth.redirect_uri.clone().unwrap_or_else(|| "http://localhost:37387/".to_string())),
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
    }
    Ok(())
}
