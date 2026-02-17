// String is an owned, heap-allocated string (mutable)
// &str is a borrowed (immutable) string slice
pub struct Request { 
    pub method: String,
    pub path: String,
    pub version: String,
}

// Rust struct mutability is all or nothing
// If one field is muutable then the entire struct is mutable
pub struct Response {
    pub status: u16,
    // 'static lifetime lives for the entire duration of the program
    // Fixed because there is a small set of possible status reasons
    pub reason: &'static str,
    pub body: String,
}

pub enum ParseError {
    Empty,
    MalformedRequestLine,
    InvalidUtf8,
}

impl Response {
    // Into<String> means the caller can pass in &str or String
    // Into string converts it automatically
    pub fn new(status: u16, reason: &'static str, body: impl Into<String>) -> Self {
        Self { status, reason, body: body.into() }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // Formats HTTP/1.1 status line + headers + body
        // Must include correct Content-Length header
        // format! produces a string
        // \r\n is required for HTTP 1.1 spec
        let headers = [
            format!("HTTP/1.1 {} {}", self.status, self.reason),
            format!("Content-Type: text/plain; charset=utf-8"),
            format!("Content-Length: {}", self.body.len()),
            format!("Connection: close"),
        ];
        format!("{} \r\n\r\n{}", headers.join("\r\n"), self.body).into_bytes()
    }
}

// Vec<u8> is an owned collection of bytes
// &[u8] is a borrowed slice of bytes
pub fn parse_request(buf: &[u8]) -> Result<Request, ParseError> {
    // Split on \r\n to get request line
    // Split request line on whitespace to get method/path/version
    // Return an error if malformed

    let raw = std::str::from_utf8(buf).map_err(|_| ParseError::InvalidUtf8)?;
    let request_line = raw.split("\r\n").next().ok_or(ParseError::Empty)?;
    // Split GET /index.html HTTP/1.1 into method, path, version
    let mut parts = request_line.splitn(3, ' ');
    let method = parts.next().ok_or(ParseError::MalformedRequestLine)?.to_string();
    let path = parts.next().ok_or(ParseError::MalformedRequestLine)?.to_string();
    let version = parts.next().ok_or(ParseError::MalformedRequestLine)?.to_string();
    if method.is_empty() || path.is_empty() || version.is_empty() {
        return Err(ParseError::Empty);
    }
    Ok(Request { method, path, version })
}