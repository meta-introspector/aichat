mod cli;
mod client;
mod config;
mod function;
mod rag;
mod render;
mod repl;
mod serve;
mod auth;
#[macro_use]
mod utils;

#[macro_use]
extern crate log;

mod main_main;
mod main_handle_auth_command;
mod main_handle_manage_google_oauth_command;
mod main_run;
mod main_start_directive;
mod main_start_interactive;
mod main_shell_execute;
mod main_create_input;
mod main_setup_logger;

use anyhow::Result;

fn main() -> Result<()> {
    main_main::main()
}