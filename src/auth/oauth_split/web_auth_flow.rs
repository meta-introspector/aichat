use anyhow::{Result, Context};
use std::io::{BufReader, Write, BufRead};
use std::net::TcpListener;
use oauth2::url::Url;
use oauth2::{CsrfToken, AuthorizationCode};
use open;

pub async fn run_web_auth_flow(
    authorize_url: Url,
    csrf_state: CsrfToken,
    port: u16,
) -> Result<(AuthorizationCode, CsrfToken)> {
    let mut redacted_authorize_url = authorize_url.clone();
    {
        let mut query_pairs = redacted_authorize_url.query_pairs_mut();
        query_pairs.clear();
        for (key, value) in authorize_url.query_pairs().into_iter() {
            match key.as_ref() {
                "client_id" | "client_secret" | "code_challenge" | "state" => {
                    query_pairs.append_pair(&key, "REDACTED");
                },
                _ => {
                    query_pairs.append_pair(&key, &value);
                }
            }
        }
    }
    println!("Open this URL in your browser:\n{}\n", redacted_authorize_url);
    open::that(authorize_url.as_str()).context("Failed to open browser")?;

    println!("Attempting to bind TcpListener to 127.0.0.1:{}", port);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
    println!("TcpListener bound successfully.");

    loop {
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

        let message: &str;
        if let (Some(code), Some(state)) = (code, state) {
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                "Authentication successful! You can close this tab.".len(),
                "Authentication successful! You can close this tab."
            );
            stream.write_all(response.as_bytes())?;
            return Ok((code, state));
        } else {
            message = "No code or state found in redirect URL.";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes())?;
            continue; // Continue to next iteration if no code/state
        }
    }
}
