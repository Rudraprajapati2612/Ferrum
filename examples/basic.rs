// examples/basic.rs
// Phase 2: now handlers can read body, headers, query params
//
// Run with: cargo run --example basic

use ferrum::{App, Context};

fn main() {
    let mut app = App::new();

    // GET /
    app.get("/", |ctx: &mut Context| {
        ctx.send(200, "Welcome to Ferrum!");
    });

    // GET /health
    app.get("/health", |ctx: &mut Context| {
        ctx.json(200, r#"{"status": "ok", "framework": "Ferrum"}"#);
    });

    // GET /users?page=1&limit=10
    // Phase 2: query params now work
    app.get("/users", |ctx: &mut Context| {
        let page  = ctx.request.query("page").map(|s| s.as_str()).unwrap_or("1");
        let limit = ctx.request.query("limit").map(|s| s.as_str()).unwrap_or("10");

        let body = format!(
            r#"{{"users": [], "page": {}, "limit": {}}}"#,
            page, limit
        );
        ctx.json(200, &body);
    });

    // POST /users
    // Phase 2: body now parsed and accessible
    app.post("/users", |ctx: &mut Context| {
        match &ctx.request.body {
            Some(body) => {
                println!("Creating user with body: {}", body);
                ctx.json(201, r#"{"message": "user created", "received": true}"#);
            }
            None => {
                ctx.json(400, r#"{"error": "body is required"}"#);
            }
        }
    });

    // GET /headers-demo — shows how to read any header
    app.get("/headers-demo", |ctx: &mut Context| {
        let content_type = ctx.request
            .header("accept")
            .map(|s| s.as_str())
            .unwrap_or("not set");

        let body = format!(r#"{{"accept_header": "{}"}}"#, content_type);
        ctx.json(200, &body);
    });

    app.listen(8080);
}