# Rust HTTP Server From Scratch

Building an HTTP server in Rust without any frameworks — just the standard library and eventually Tokio.

### How it works

A `TcpListener` binds to port 7878 and loops forever calling `accept()`. Each accepted connection gets its own OS thread. That thread reads raw bytes from the socket, parses the HTTP request line, routes to a handler, writes the response, and closes the connection.

```
client connects
    → accept() returns a TcpStream
    → thread spawned
    → read bytes into stack buffer
    → parse request line (METHOD PATH VERSION)
    → match path → response
    → write HTTP response bytes
    → drop TcpStream (connection closed)
```

### Project structure

```
src/
  main.rs              — TcpListener accept loop, spawns one thread per connection
  server/
    mod.rs             — declares the server submodules
    http.rs            — HTTP parsing (parse_request) and response formatting (to_bytes)
    router.rs          — maps (method, path) to a Response
    blocking.rs        — handle_connection: reads socket, parses, routes, writes response
```

### Endpoints

- `GET /` → 200 Hello, World!
- `GET /health` → 200 OK
- anything else → 404 Not Found
- malformed request → 500

### Test it

```bash
cargo run
curl -v http://127.0.0.1:7878/
curl -v http://127.0.0.1:7878/health
curl -v http://127.0.0.1:7878/doesnotexist
```
