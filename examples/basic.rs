// examples/basic.rs
// This is how a USER of your framework writes their app.
// Clean, simple — just like Express.
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

    // POST /users
    app.post("/users", |ctx: &mut Context| {
        // Phase 2: ctx.request.body will have the real parsed body
        // For now, just acknowledge the request
        ctx.json(201, r#"{"message": "user created"}"#);
    });

    // GET /users
    app.get("/users", |ctx: &mut Context| {
        ctx.json(200, r#"[{"id": 1, "name": "Rudra"}]"#);
    });

    // start the server
    app.listen(8080);
}