
use ferrum::{App, Context};

fn main() {
    let mut app = App::new();

   

    app.get("/", |ctx: &mut Context| {
        ctx.send(200, "Welcome to Ferrum!");
    });

    app.get("/health", |ctx: &mut Context| {
        ctx.json(200, r#"{"status": "ok"}"#);
    });

   

    // GET /users?page=1&limit=10
    app.get("/users", |ctx: &mut Context| {
        let page  = ctx.request.query("page").map(|s| s.as_str()).unwrap_or("1");
        let limit = ctx.request.query("limit").map(|s| s.as_str()).unwrap_or("10");
        ctx.json(200, &format!(r#"{{"users": [], "page": {}, "limit": {}}}"#, page, limit));
    });

    // POST /users
    app.post("/users", |ctx: &mut Context| {
        match &ctx.request.body.clone() {
            Some(body) => ctx.json(201, &format!(r#"{{"created": true, "data": {}}}"#, body)),
            None       => ctx.bad_request("body is required"),
        }
    });

    // Phase 4: LITERAL route — must register BEFORE the param route
    // /users/profile will match this, NOT /users/:id
    app.get("/users/profile", |ctx: &mut Context| {
        ctx.json(200, r#"{"page": "profile", "tip": "literal beats param"}"#);
    });

    // Phase 4: PARAM route — matches /users/42, /users/abc, anything
    app.get("/users/:id", |ctx: &mut Context| {
        let id = ctx.request.param("id").map(|s| s.as_str()).unwrap_or("unknown");
        ctx.json(200, &format!(r#"{{"id": "{}"}}"#, id));
    });

    // Phase 4: DELETE with param
    app.delete("/users/:id", |ctx: &mut Context| {
        let id = ctx.request.param("id").map(|s| s.as_str()).unwrap_or("unknown");
        ctx.json(200, &format!(r#"{{"deleted": true, "id": "{}"}}"#, id));
    });

    // Phase 4: multiple params in one route
    app.get("/users/:userId/posts/:postId", |ctx: &mut Context| {
        let user_id = ctx.request.param("userId").map(|s| s.as_str()).unwrap_or("");
        let post_id = ctx.request.param("postId").map(|s| s.as_str()).unwrap_or("");
        ctx.json(200, &format!(
            r#"{{"userId": "{}", "postId": "{}"}}"#,
            user_id, post_id
        ));
    });

    // Phase 4: method not allowed test
    // only GET registered — POST will return 405
    app.get("/ping", |ctx: &mut Context| {
        ctx.send(200, "pong");
    });

    app.listen(8080);
}