use anyhow::Result;
use std::net::TcpListener;
use url::Url;

pub fn find_available_port(redirect_uri: &str) -> Result<u16> {
    if let Ok(url) = Url::parse(redirect_uri) {
        if let Some(port) = url.port() {
            return Ok(port);
        }
    }
    // Fallback to finding a random available port if no port is specified in redirect_uri
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    Ok(port)
}
