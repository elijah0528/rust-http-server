use super::http::{Request, Response};

pub fn route(request: &Request) -> Response {
    match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/") => Response::new(200, "OK", "Hello, World!"),
        ("GET", "/health") => Response::new(200, "OK", "OK"),
        _ => Response::new(404, "Not Found", "Not Found"),
    }
}