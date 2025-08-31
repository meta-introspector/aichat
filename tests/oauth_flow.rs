use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::net::TcpListener;
use std::io::{Write, Read};
use std::time::Duration;
use tempfile::tempdir;

mod oauth_flow_parts;

#[tokio::test]
async fn test_oauth_login_success() -> Result<()> {
    // Setup
    let (temp_dir, home_dir) = oauth_flow_parts::setup::setup_test_environment()?;
    let port = oauth_flow_parts::oauth_callback::find_available_port()?;

    // Command Execution
    let output = oauth_flow_parts::command_execution::run_aichat_login_command().await?;

    // Simulate OAuth Callback
    oauth_flow_parts::oauth_callback::simulate_oauth_callback(port).await?;

    // Verification
    oauth_flow_parts::verification::verify_oauth_success(&home_dir, &output)?; // Pass output here

    // Cleanup
    oauth_flow_parts::cleanup::cleanup_test_environment(temp_dir)?; // Pass temp_dir here

    Ok(())
}

// TODO: Add test case for OAuth Login with Port Conflict
// TODO: Add test case for OAuth Login with Invalid Callback (e.g., incorrect state, missing code)
// TODO: Add test case for OAuth Refresh (simulating expired token and automatic refresh)
// TODO: Add test case for OAuth Logout (verifying deletion of oauth_creds.json)