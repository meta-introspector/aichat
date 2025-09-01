use crate::cli::Cli;
use crate::config::{Config, GlobalConfig, load_env_file};
use crate::render::render_error;
use anyhow::Result;
use clap::Parser;
use std::sync::Arc;
use parking_lot::RwLock;
use crate::WorkingMode;

#[tokio::main]
pub async fn main() -> Result<()> {
    load_env_file()?;
    let cli = Cli::parse();
    let text = cli.text()?;
    let working_mode = if cli.serve.is_some() {
        WorkingMode::Serve
    } else if text.is_none() && cli.file.is_empty() {
        WorkingMode::Repl
    } else {
        WorkingMode::Cmd
    };
    let info_flag = cli.info
        || cli.sync_models
        || cli.list_models
        || cli.list_roles
        || cli.list_agents
        || cli.list_rags
        || cli.list_macros
        || cli.list_sessions;
    crate::main_setup_logger::setup_logger(working_mode.is_serve())?;
    let config = Arc::new(RwLock::new(Config::init(working_mode, info_flag).await?));

    if let Some(command) = cli.command {
        match command {
            cli::Commands::Auth(auth_command) => {
                crate::main_handle_auth_command::handle_auth_command(auth_command.command, config.clone()).await?;
            }
        }
        return Ok(());
    }

    if let Err(err) = crate::main_run::run(config, cli, text).await {
        render_error(err);
        std::process::exit(1);
    }
    Ok(())
}
