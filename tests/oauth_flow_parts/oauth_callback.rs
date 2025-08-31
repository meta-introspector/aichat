use anyhow::Result;
use std::net::TcpListener;
use std::time::Duration;

// Helper function to find an available port
pub fn find_available_port() -> Result<u16> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    Ok(port)
}

pub async fn simulate_oauth_callback(port: u16) -> Result<()> {
    let redirect_uri = format!("http://localhost:{}", port);
    let auth_code = "test_auth_code";
    let state = "test_state"; // This should ideally be extracted from aichat's output

    let callback_url = format!("{}/oauth2callback?code={}&state={}", redirect_uri, auth_code, state);

    // Make an HTTP request to the temporary web server
    let client = reqwest::Client::new();
    let response = client.get(&callback_url).send().await?;
    assert!(response.status().is_success());

    Ok(())
}
