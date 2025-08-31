use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::net::TcpListener;
use std::io::{Write, Read};
use std::time::Duration;
use tempfile::tempdir;

// Helper function to find an available port
fn find_available_port() -> Result<u16> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    Ok(port)
}

#[tokio::test]
async fn test_oauth_login_success() -> Result<()> {
    let temp_dir = tempdir()?;
    let home_dir = temp_dir.path().to_path_buf();

    // Set HOME environment variable for the test to use a temporary directory
    // This ensures that oauth_creds.json is written to a controlled location
    std::env::set_var("HOME", &home_dir);

    let port = find_available_port()?;
    let redirect_uri = format!("http://localhost:{}", port);

    // Start the aichat process in the background
    let mut cmd = Command::cargo_bin("aichat")?;
    let mut child = cmd.arg("auth").arg("login").spawn()?;

    // Wait for the aichat process to print the auth URL
    // This is a simplified approach; in a real test, you might parse stderr/stdout
    // to extract the exact URL and state.
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Simulate the OAuth callback
    let auth_code = "test_auth_code";
    let state = "test_state"; // This should ideally be extracted from aichat's output

    let callback_url = format!("{}/oauth2callback?code={}&state={}", redirect_uri, auth_code, state);

    // Make an HTTP request to the temporary web server
    let client = reqwest::Client::new();
    let response = client.get(&callback_url).send().await?;
    assert!(response.status().is_success());

    // Wait for the aichat process to finish
    let output = child.wait_with_output().await?;

    // Assert that the command exited successfully
    assert!(output.status.success());

    // Verify that oauth_creds.json was created
    let creds_path = home_dir.join(".gemini").join("oauth_creds.json");
    assert!(fs::metadata(&creds_path).is_ok());

    // Read and verify the content of oauth_creds.json (simplified check)
    let creds_content = fs::read_to_string(&creds_path)?;
    assert!(creds_content.contains("access_token"));
    assert!(creds_content.contains("refresh_token"));

    // Clean up temporary directory
    temp_dir.close()?;

    Ok(())
}

// TODO: Add test case for OAuth Login with Port Conflict
// TODO: Add test case for OAuth Login with Invalid Callback (e.g., incorrect state, missing code)
// TODO: Add test case for OAuth Refresh (simulating expired token and automatic refresh)
// TODO: Add test case for OAuth Logout (verifying deletion of oauth_creds.json)
