use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;

use crate::{Context, Middleware};
use crate::http::request::Request;
use crate::router::Router;

pub async fn start(port: u16, router: Router, middlewares: Vec<Middleware>) {
    let addr = format!("127.0.0.1:{}", port);

    let listener = TcpListener::bind(&addr)
        .await
        .expect(&format!("Failed to bind to {}", addr));

    println!("─────────────────────────────────────");
    println!("  Ferrum running on http://{}", addr);
    println!("─────────────────────────────────────");

    let router      = Arc::new(router);
    let middlewares = Arc::new(middlewares);

    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                let router      = Arc::clone(&router);
                let middlewares = Arc::clone(&middlewares);

                tokio::spawn(async move {
                    handle_connection(stream, &router, &middlewares).await;
                });
            }
            Err(e) => eprintln!("Accept error: {}", e),
        }
    }
}

async fn handle_connection(
    mut stream:  TcpStream,
    router:      &Router,
    middlewares: &[Middleware],
) {
    // allocate buffer ONCE outside loop — reuse across requests
    let mut buffer = vec![0u8; 4096];

    loop {
        // clear buffer from previous request
        buffer.fill(0);

        let bytes_read = match stream.read(&mut buffer).await {
            Ok(0) => break,   // client disconnected cleanly
            Ok(n) => n,
            Err(_) => break,  // connection reset — just exit silently
        };
        // eprintln!("DEBUG raw: {:?}", String::from_utf8_lossy(&buffer[..bytes_read]));

        let raw_bytes = &buffer[..bytes_read];
        // convert raw bytes to Struct 
        let request = match Request::from_bytes(raw_bytes) {
            Ok(r)  => r,
            Err(_) => {
                let _ = stream.write_all(
                    b"HTTP/1.1 400 Bad Request\r\nContent-Length: 11\r\n\r\nBad Request"
                ).await;
                break;
            }
        };

        let should_close = request
            .header("connection")
            .map(|v| v.to_lowercase() == "close")
            .unwrap_or(false);

        let method_str = request.method.as_str().to_string();
        let path       = request.path.clone();
        let ctx        = Context::new(request);
        let ctx        = router.dispatch(&method_str, &path, ctx, middlewares);

        let response_bytes = ctx.response.to_bytes();

        if let Err(_) = stream.write_all(&response_bytes).await {
            break;
        }

        if should_close { break; }
    }
}