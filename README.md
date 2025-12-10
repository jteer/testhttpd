# testhttpd

A simple HTTP server for testing and debugging HTTP requests, built with Rust and Hyper.

## Features

- Logs all incoming HTTP requests with method, URI, headers, and body
- Supports two modes: test/log mode and static file serving mode
- Displays request bodies for POST, PUT, and other requests with payloads
- Configurable port binding (defaults to 8080)
- Async I/O using Tokio for efficient connection handling
- Structured logging with tracing for clear, formatted output
- Serves static files from a specified directory with automatic index.html handling

## Installation

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))

### Build from source

```bash
git clone https://github.com/yourusername/testhttpd.git
cd testhttpd
cargo build --release
```

The binary will be available at `target/release/testhttpd`.

## Usage

### Test/Log Mode

Run the server to log all incoming requests:

```bash
cargo run
```

Or with a custom port:

```bash
cargo run -- --port 3000
```

### File Serving Mode

Serve static files from a directory:

```bash
cargo run -- --serve-dir ./public
```

The server will serve files from the specified directory, defaulting to `index.html` for the root path.

### Command-line Options

```
Options:
  -p, --port <PORT>              Port to listen on [default: 8080]
      --serve-dir <SERVE_DIR>    Serve files from this directory
  -h, --help                     Print help
  -V, --version                  Print version
```

## Example Requests

Test the server with curl:

```bash
# Simple GET request
curl http://localhost:8080/

# POST with JSON data
curl -X POST http://localhost:8080/api/test \
  -H "Content-Type: application/json" \
  -d '{"key": "value"}'

# Request with custom headers
curl http://localhost:8080/ \
  -H "X-Custom-Header: test" \
  -H "Authorization: Bearer token123"

# Verbose output to see full request/response
curl -v http://localhost:8080/
```

## Use Cases

- Debugging webhook integrations by logging incoming payloads
- Testing HTTP clients and their request formatting
- Quick static file server for local development
- Learning and experimenting with HTTP requests and headers
- Verifying API request structure before implementing endpoints
