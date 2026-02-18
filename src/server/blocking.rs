use std::io::{Read, Write};
use std::net::TcpStream;

use super::http;
use super::router::{route};

// TcpStream must be mut because reading and writing changes its internal state
pub fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    // Declares buffer on the stack
    let mut buf = [0u8; 8192];
    let n = stream.read(&mut buf)?;

    if n == 0 {
        return Ok(());
    }

    let response = match http::parse_request(&buf[..n]) {
        Ok(request) => route(&request),
        Err(_) => http::Response::new(500, "Internal Server Error", "Internal Server Error"),
    };

    stream.write_all(&response.to_bytes())?;
    stream.flush()?;
    // TcpStream is dropped here, connection is closed
    Ok(())
}