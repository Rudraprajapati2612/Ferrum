# Ferrum 🦀

A fast, lightweight HTTP framework for Rust — built from scratch to understand how web frameworks work under the hood.

Ferrum handles raw TCP connections, parses HTTP/1.1 bytes manually, routes requests through a radix trie, runs middleware chains, and serves responses — all without depending on any existing HTTP library.

> **Why "Ferrum"?** Ferrum is the Latin word for iron — Rust's core element.

---

## Why it exists

Most developers use Express, Axum, or Actix without understanding what happens between a browser sending bytes and a handler function running. Ferrum was built to answer that question by implementing every layer from scratch:

- Raw TCP socket handling
- HTTP/1.1 byte parsing
- Radix trie routing
- Middleware chain execution
- Response serialization
- Async I/O with Tokio

---

## Features

| Feature | Details |
|---|---|
| **HTTP/1.1 parser** | Parses method, path, headers, body, query params from raw bytes |
| **Radix trie router** | O(log n) route matching — faster than Express's linear scan |
| **Path parameters** | `/users/:id`, `/users/:userId/posts/:postId` |
| **Query parameters** | `ctx.request.query("page")` |
| **Middleware chain** | Stack-based with `next()` — logger, auth, rate limiter |
| **Sub-routing** | `app.mount("/api/v1/users", user_router)` |
| **Keep-Alive** | Persistent TCP connections across multiple requests |
| **Async I/O** | Tokio-powered — handles 250,000+ RPS |
| **Error helpers** | `ctx.not_found()`, `ctx.unauthorized()`, `ctx.bad_request()` |
| **Custom headers** | `ctx.set_header("X-Request-Id", "abc")` |
| **Redirects** | `ctx.redirect(301, "/new-path")` |

---

## Benchmarks

Tested on a standard developer machine with `wrk -t4 -c100 -d10s`:

```
Ferrum (async, no middleware)   →   294,000 RPS
Ferrum (async, with middleware) →   257,000 RPS
Express (Node.js)               →     5,000 - 15,000 RPS
```

---

## Requirements

- Rust 1.70 or higher
- Cargo (comes with Rust)

Install Rust via [rustup](https://rustup.rs):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Getting Started

### 1. Add Ferrum to your project

```toml
# Cargo.toml
[dependencies]
ferrum = { path = "." }
tokio  = { version = "1", features = ["full"] }
```

### 2. Write your first app

```rust
use ferrum::{App, Context};

#[tokio::main]
async fn main() {
    let mut app = App::new();

    app.get("/", |ctx: &mut Context| {
        ctx.send(200, "Hello from Ferrum!");
    });

    app.get("/health", |ctx: &mut Context| {
        ctx.json(200, r#"{"status": "ok"}"#);
    });

    app.listen(8080).await;
}
```

### 3. Run it

```bash
cargo run --example basic
```

### 4. Test it

```bash
curl http://localhost:8080/
# Hello from Ferrum!

curl http://localhost:8080/health
# {"status": "ok"}
```

---

## Installation & Setup

```bash
# clone the repo
git clone https://github.com/yourname/ferrum.git
cd ferrum

# run the example app
cargo run --example basic

# run in release mode (full performance)
cargo run --example basic --release

# run tests
cargo test
```

---

## Usage

### Basic routing

```rust
app.get("/users", |ctx: &mut Context| {
    ctx.json(200, r#"{"users": []}"#);
});

app.post("/users", |ctx: &mut Context| {
    ctx.json(201, r#"{"created": true}"#);
});

app.put("/users/:id", |ctx: &mut Context| {
    ctx.json(200, r#"{"updated": true}"#);
});

app.delete("/users/:id", |ctx: &mut Context| {
    ctx.json(200, r#"{"deleted": true}"#);
});
```

### Path parameters

```rust
// single param
app.get("/users/:id", |ctx: &mut Context| {
    let id = ctx.request.param("id").map(|s| s.as_str()).unwrap_or("");
    ctx.json(200, &format!(r#"{{"id": "{}"}}"#, id));
});

// multiple params
app.get("/users/:userId/posts/:postId", |ctx: &mut Context| {
    let user_id = ctx.request.param("userId").map(|s| s.as_str()).unwrap_or("");
    let post_id = ctx.request.param("postId").map(|s| s.as_str()).unwrap_or("");
    ctx.json(200, &format!(r#"{{"userId": "{}", "postId": "{}"}}"#, user_id, post_id));
});
```

### Query parameters

```rust
// GET /users?page=2&limit=10
app.get("/users", |ctx: &mut Context| {
    let page  = ctx.request.query("page").map(|s| s.as_str()).unwrap_or("1");
    let limit = ctx.request.query("limit").map(|s| s.as_str()).unwrap_or("10");
    ctx.json(200, &format!(r#"{{"page": {}, "limit": {}}}"#, page, limit));
});
```

### Reading request body

```rust
app.post("/users", |ctx: &mut Context| {
    match &ctx.request.body.clone() {
        Some(body) => ctx.json(201, &format!(r#"{{"data": {}}}"#, body)),
        None       => ctx.bad_request("body is required"),
    }
});
```

### Reading headers

```rust
app.get("/protected", |ctx: &mut Context| {
    let token = ctx.request.header("authorization");
    match token {
        Some(t) => ctx.json(200, &format!(r#"{{"token": "{}"}}"#, t)),
        None    => ctx.unauthorized("missing token"),
    }
});
```

### Middleware

```rust
use ferrum::{App, Context, Next};

// logger — runs before and after every request
fn logger(ctx: &mut Context, next: Next) {
    println!("--> {} {}", ctx.request.method.as_str(), ctx.request.path);
    next(ctx);
    println!("<-- {}", ctx.response.status);
}

// auth — stops chain if no token
fn auth(ctx: &mut Context, next: Next) {
    match ctx.request.header("authorization") {
        Some(_) => next(ctx),
        None    => ctx.unauthorized("missing authorization header"),
    }
}

#[tokio::main]
async fn main() {
    let mut app = App::new();

    app.use_middleware(logger);
    app.use_middleware(auth);

    app.get("/protected", |ctx: &mut Context| {
        ctx.json(200, r#"{"message": "you are authorized"}"#);
    });

    app.listen(8080).await;
}
```

### Sub-routing

```rust
use ferrum::{App, SubRouter, Context};

fn user_routes() -> SubRouter {
    let mut router = SubRouter::new();

    router.get("/", |ctx: &mut Context| {
        ctx.json(200, r#"{"users": []}"#);
    });

    router.get("/:id", |ctx: &mut Context| {
        let id = ctx.request.param("id").map(|s| s.as_str()).unwrap_or("");
        ctx.json(200, &format!(r#"{{"id": "{}"}}"#, id));
    });

    router.post("/", |ctx: &mut Context| {
        ctx.json(201, r#"{"created": true}"#);
    });

    router
}

fn auth_routes() -> SubRouter {
    let mut router = SubRouter::new();

    router.post("/login", |ctx: &mut Context| {
        ctx.json(200, r#"{"token": "abc123"}"#);
    });

    router.post("/register", |ctx: &mut Context| {
        ctx.json(201, r#"{"message": "registered"}"#);
    });

    router
}

#[tokio::main]
async fn main() {
    let mut app = App::new();

    // mount routers at prefixes — just like Express
    app.mount("/api/v1/users", user_routes());
    app.mount("/api/v1/auth",  auth_routes());

    app.listen(8080).await;
}
```

Registered routes:
```
GET  /api/v1/users
GET  /api/v1/users/:id
POST /api/v1/users
POST /api/v1/auth/login
POST /api/v1/auth/register
```

### Response helpers

```rust
ctx.send(200, "plain text");                    // text/plain
ctx.json(200, r#"{"key": "value"}"#);           // application/json
ctx.not_found("user not found");                // 404 JSON error
ctx.bad_request("email is required");           // 400 JSON error
ctx.unauthorized("invalid token");              // 401 JSON error
ctx.forbidden("access denied");                 // 403 JSON error
ctx.internal_error("db connection failed");     // 500 JSON error
ctx.redirect(301, "/new-path");                 // redirect
ctx.set_header("X-Request-Id", "abc-123");      // custom header
```

---

## Project structure

```
ferrum/
├── Cargo.toml              # dependencies (tokio)
├── README.md
├── examples/
│   └── basic.rs            # example app — shows how users use the framework
└── src/
    ├── lib.rs              # public API: App, SubRouter, Context, Handler, Middleware
    ├── server.rs           # TCP listener, async connection handler, keep-alive loop
    ├── router.rs           # radix trie: insert routes, search paths, extract params
    └── http/
        ├── mod.rs          # groups http modules
        ├── request.rs      # HTTP parser: bytes → Request struct
        └── response.rs     # HTTP serializer: Response struct → bytes
```

### Component responsibilities

| File | Responsibility |
|---|---|
| `lib.rs` | Public-facing API. Users only import from here |
| `server.rs` | Accepts TCP connections, reads bytes, writes bytes. Tokio async loop |
| `router.rs` | Radix trie data structure. Matches paths, extracts `:param` values |
| `request.rs` | Parses raw HTTP bytes into structured `Request` — method, path, headers, body |
| `response.rs` | Serializes `Response` struct into raw HTTP bytes following HTTP/1.1 spec |

---

## How it works — the full request lifecycle

```
Browser sends: POST /api/v1/users HTTP/1.1...
      ↓
TCP 3-way handshake → connection established
      ↓
server.rs reads raw bytes from TCP stream
      ↓
request.rs finds \r\n\r\n → splits headers and body
  → parses method, path, version from line 1
  → parses headers into HashMap (lowercase keys)
  → reads body using Content-Length
  → parses query params from path
      ↓
router.rs walks radix trie → finds matching handler
  → extracts :param values → puts into request.params
      ↓
middleware chain runs → logger → auth → handler
  → each middleware calls next() to continue
  → or returns early to stop the chain
      ↓
handler fills ctx.response with status + body + headers
      ↓
response.rs serializes Response → raw HTTP bytes
  → status line: HTTP/1.1 200 OK\r\n
  → headers: Key: Value\r\n
  → blank line: \r\n
  → body bytes
      ↓
server.rs writes bytes back to TCP stream
      ↓
connection stays alive (Keep-Alive) → ready for next request
```

---

## Dependencies

| Dependency | Version | Purpose |
|---|---|---|
| `tokio` | 1.x | Async runtime — event loop, TCP, task spawning |

That's it. No HTTP parsing library, no routing library, no middleware library. Everything is built from scratch.

---

## Configuration

Ferrum has no config files. Everything is set in code:

```rust
// change port
app.listen(3000).await;

// register middleware globally
app.use_middleware(logger);

// mount sub-routers
app.mount("/api/v2", v2_routes());
```

Environment variables can be read with Rust's standard library:

```rust
let port: u16 = std::env::var("PORT")
    .unwrap_or("8080".to_string())
    .parse()
    .unwrap_or(8080);

app.listen(port).await;
```

---

## Running benchmarks

```bash
# install wrk
sudo apt install wrk        # Ubuntu
brew install wrk            # macOS

# start server in release mode
cargo run --example basic --release

# benchmark
wrk -t4 -c100 -d10s http://localhost:8080/

# benchmark with more connections
wrk -t4 -c500 -d10s http://localhost:8080/

# benchmark POST with body
# create post.lua:
# wrk.method = "POST"
# wrk.body   = '{"name": "test"}'
# wrk.headers["Content-Type"] = "application/json"
wrk -t4 -c100 -d10s -s post.lua http://localhost:8080/api/v1/users
```

---

## Contributing

Contributions are welcome. Here's how to get started:

```bash
# fork and clone
git clone https://github.com/yourname/ferrum.git
cd ferrum

# create a branch
git checkout -b feature/your-feature

# make changes and test
cargo test
cargo run --example basic

# commit and push
git add .
git commit -m "add: your feature description"
git push origin feature/your-feature

# open a pull request
```

### What to contribute

- Bug fixes
- New response helpers on `Context`
- Better error messages from the parser
- Additional HTTP methods
- Tests for edge cases in the parser or router
- Performance improvements

### Code style

- Keep files focused — each file has one responsibility
- Add comments explaining the *why* not just the *what*
- No external dependencies unless absolutely necessary
- Run `cargo clippy` before submitting

---

## What's next

Planned features:

```
[ ] TLS/HTTPS support via rustls
[ ] JSON body auto-parsing into typed structs (serde integration)
[ ] Route groups with shared middleware
[ ] Static file serving
[ ] WebSocket upgrade support
[ ] HTTP/2 support
```

---

## License

MIT License — free to use, modify, and distribute.

---

## Built with

This framework was built phase by phase as a learning project:

```
Phase 1 → Raw TCP listener, byte reading
Phase 2 → Full HTTP/1.1 parser
Phase 3 → Response serialization
Phase 4 → Radix trie router with :param support
Phase 5 → Middleware chain engine
Phase 6 → Async I/O with Tokio (250k+ RPS)
Phase 7 → Sub-routing with app.mount()
```

Every component — from the TCP socket to the radix trie to the middleware chain — is written from scratch in safe Rust.
