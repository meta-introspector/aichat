use anyhow::{Result, Context};
use std::io::{BufReader, Write, BufRead};
use std::net::TcpListener;
use oauth2::url::Url;

pub async fn run_web_flow_listener(port: u16) -> Result<Url> {
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

        // Send a response to the browser
        let message = "Please return to the application.";
        let response = format!(
            "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
            message.len(),
            message
        );
        stream.write_all(response.as_bytes())?;

        return Ok(url);
    }
}
