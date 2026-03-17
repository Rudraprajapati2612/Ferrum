
use ferrum::{App, Context};

fn main() {
    let mut app = App::new();

    // GET /
    app.get("/", |ctx: &mut Context| {
        ctx.send(200, "Welcome to Ferrum!");
    });

    // GET /health
    app.get("/health", |ctx: &mut Context| {
        ctx.json(200, r#"{"status": "ok"}"#);
    });

    // GET /users
    app.get("/users", |ctx: &mut Context| {
        let page  = ctx.request.query("page").map(|s| s.as_str()).unwrap_or("1");
        let limit = ctx.request.query("limit").map(|s| s.as_str()).unwrap_or("10");
        let body  = format!(r#"{{"users": [], "page": {}, "limit": {}}}"#, page, limit);

        // Phase 3: set custom headers before responding
        ctx.set_header("X-Total-Count", "0");
        ctx.set_header("Cache-Control", "max-age=60");
        ctx.json(200, &body);
    });

    // POST /users
    app.post("/users", |ctx: &mut Context| {
        match &ctx.request.body.clone() {
            Some(body) => {
                ctx.set_header("X-Request-Id", "abc-123");
                ctx.json(201, &format!(r#"{{"message": "created", "body": {}}}"#, body));
            }
            // Phase 3: clean error helper instead of manual json
            None => ctx.bad_request("body is required"),
        }
    });

    // GET /old-path — Phase 3: redirect
    app.get("/old-path", |ctx: &mut Context| {
        ctx.redirect(301, "/");
    });

    // GET /secret — Phase 3: unauthorized helper
    app.get("/secret", |ctx: &mut Context| {
        ctx.unauthorized("you need to login first");
    });

    // GET /crash — Phase 3: internal error helper
    app.get("/crash", |ctx: &mut Context| {
        ctx.internal_error("something went wrong");
    });

    app.listen(8080);
}