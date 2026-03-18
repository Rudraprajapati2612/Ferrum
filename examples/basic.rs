
use ferrum::{App, Context, Next};


fn logger(ctx: &mut Context, next: Next) {
    // BEFORE handler runs
    println!(
        "[LOG] --> {} {}",
        ctx.request.method.as_str(),
        ctx.request.path
    );

    // call next middleware (or handler if no more middlewares)
    next(ctx);

    // AFTER handler runs — response is now filled
    println!("[LOG] <-- {}", ctx.response.status);
}


fn auth(ctx: &mut Context, next: Next) {
    let token = ctx.request.header("authorization").cloned();

    match token {
        Some(t) => {
            println!("[AUTH] token found: {}", t);
            // valid — continue chain
            next(ctx);
        }
        None => {
            println!("[AUTH] no token — blocking request");
            // stop chain — handler never runs
            ctx.unauthorized("missing authorization header");
        }
    }
}


fn request_id(ctx: &mut Context, next: Next) {
    // run handler first
    next(ctx);

    // AFTER handler — add header to response
    ctx.response.headers.insert(
        "X-Request-Id".to_string(),
        "req-abc-123".to_string()
    );
}


fn main() {
    let mut app = App::new();

    
    // order matters — logger runs first, then auth, then request_id
    app.use_middleware(logger);
    app.use_middleware(auth);
    app.use_middleware(request_id);

    // ── public routes (no auth needed) ───────────────────────────

    app.get("/", |ctx: &mut Context| {
        ctx.send(200, "Welcome to Ferrum!");
    });

    app.get("/health", |ctx: &mut Context| {
        ctx.json(200, r#"{"status": "ok"}"#);
    });

    // ── routes with params ────────────────────────────────────────

    app.get("/users", |ctx: &mut Context| {
        let page  = ctx.request.query("page").map(|s| s.as_str()).unwrap_or("1");
        let limit = ctx.request.query("limit").map(|s| s.as_str()).unwrap_or("10");
        ctx.json(200, &format!(r#"{{"users":[],"page":{},"limit":{}}}"#, page, limit));
    });

    app.post("/users", |ctx: &mut Context| {
        match &ctx.request.body.clone() {
            Some(body) => ctx.json(201, &format!(r#"{{"created":true,"data":{}}}"#, body)),
            None       => ctx.bad_request("body is required"),
        }
    });

    app.get("/users/:id", |ctx: &mut Context| {
        let id = ctx.request.param("id").map(|s| s.as_str()).unwrap_or("unknown");
        ctx.json(200, &format!(r#"{{"id":"{}"}}"#, id));
    });

    app.delete("/users/:id", |ctx: &mut Context| {
        let id = ctx.request.param("id").map(|s| s.as_str()).unwrap_or("unknown");
        ctx.json(200, &format!(r#"{{"deleted":true,"id":"{}"}}"#, id));
    });

    app.listen(8080);
}