use anyhow::{Context, Result};
use std::net::TcpListener;
use url::Url;
use std::io::{Read, Write};

pub async fn run_web_flow_listener(port: u16) -> Result<Url> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .with_context(|| format!("Failed to bind to port {}", port))?;

    println!("Waiting for incoming connection...");
    let mut stream = listener.incoming().flatten().next().context("Listener terminated without accepting a connection")?;
    println!("Incoming connection received.");

    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).context("Failed to read from stream")?;
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

    let response = "HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nAuth success";
    stream.write_all(response.as_bytes()).context("Failed to write response")?;

    let query_string = request.lines().next().and_then(|line| {
        line.split(' ').nth(1).and_then(|path| {
            Url::parse(&format!("http://localhost{}", path)).ok()
        })
    }).context("Failed to parse URL from request")?;

    Ok(query_string)
}
