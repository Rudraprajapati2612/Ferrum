
use ferrum::{App, SubRouter, Context, Next};


fn logger(ctx: &mut Context, next: Next) {
    next(ctx);
    if ctx.response.status >= 400 {
        println!("[LOG] {} {} -> {}", 
            ctx.request.method.as_str(),
            ctx.request.path,
            ctx.response.status
        );
    }
}

fn auth(ctx: &mut Context, next: Next) {
    let token = ctx.request.header("authorization").cloned();
    match token {
        Some(_) => next(ctx),
        None    => ctx.unauthorized("missing authorization header"),
    }
}


fn auth_routes() -> SubRouter {
    let mut router = SubRouter::new();

    router.post("/login", |ctx: &mut Context| {
        match &ctx.request.body.clone() {
            Some(_) => ctx.json(200, r#"{"token": "abc123"}"#),
            None    => ctx.bad_request("email and password required"),
        }
    });

    router.post("/register", |ctx: &mut Context| {
        match &ctx.request.body.clone() {
            Some(body) => ctx.json(201, &format!(r#"{{"message": "registered", "data": {}}}"#, body)),
            None       => ctx.bad_request("body required"),
        }
    });

    router.post("/logout", |ctx: &mut Context| {
        ctx.json(200, r#"{"message": "logged out"}"#);
    });

    router
}


fn user_routes() -> SubRouter {
    let mut router = SubRouter::new();

    // GET /api/v1/users
    router.get("/", |ctx: &mut Context| {
        let page  = ctx.request.query("page").map(|s| s.as_str()).unwrap_or("1");
        let limit = ctx.request.query("limit").map(|s| s.as_str()).unwrap_or("10");
        ctx.json(200, &format!(r#"{{"users": [], "page": {}, "limit": {}}}"#, page, limit));
    });

    // GET /api/v1/users/:id
    router.get("/:id", |ctx: &mut Context| {
        let id = ctx.request.param("id").map(|s| s.as_str()).unwrap_or("");
        ctx.json(200, &format!(r#"{{"id": "{}"}}"#, id));
    });

    // PUT /api/v1/users/:id
    router.put("/:id", |ctx: &mut Context| {
        let id   = ctx.request.param("id").map(|s| s.as_str()).unwrap_or("");
        let body = ctx.request.body.clone().unwrap_or_default();
        ctx.json(200, &format!(r#"{{"updated": true, "id": "{}", "data": {}}}"#, id, body));
    });

    // DELETE /api/v1/users/:id
    router.delete("/:id", |ctx: &mut Context| {
        let id = ctx.request.param("id").map(|s| s.as_str()).unwrap_or("");
        ctx.json(200, &format!(r#"{{"deleted": true, "id": "{}"}}"#, id));
    });

    router
}


fn product_routes() -> SubRouter {
    let mut router = SubRouter::new();

    router.get("/", |ctx: &mut Context| {
        ctx.json(200, r#"{"products": []}"#);
    });

    router.get("/:id", |ctx: &mut Context| {
        let id = ctx.request.param("id").map(|s| s.as_str()).unwrap_or("");
        ctx.json(200, &format!(r#"{{"product_id": "{}"}}"#, id));
    });

    router.post("/", |ctx: &mut Context| {
        match &ctx.request.body.clone() {
            Some(body) => ctx.json(201, &format!(r#"{{"created": true, "data": {}}}"#, body)),
            None       => ctx.bad_request("body required"),
        }
    });

    router
}


#[tokio::main]
async fn main() {
    let mut app = App::new();

    // global middleware
    app.use_middleware(logger);

    // direct routes
    app.get("/", |ctx: &mut Context| {
        ctx.json(200, r#"{"message": "Welcome to Ferrum API", "version": "1.0"}"#);
    });

    app.get("/health", |ctx: &mut Context| {
        ctx.json(200, r#"{"status": "ok"}"#);
    });

    // mount sub routers at prefixes
    // exactly like Express: app.use('/api/v1/auth', authRouter)
    app.mount("/api/v1/auth",     auth_routes());
    app.mount("/api/v1/users",    user_routes());
    app.mount("/api/v1/products", product_routes());

    app.listen(8080).await;
}