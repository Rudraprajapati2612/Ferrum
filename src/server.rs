use std::io::{Read,Write};

use std::net::{TcpListener,TcpStream};

use crate::Context;
use crate::http::request::Request;
use crate::http::response;
use crate::router::Router;



pub fn start(port:u16,mut router:Router){
    let addr = format!("127.0.0.1:{}",port);

    let listner = TcpListener::bind(&addr).expect(&format!("failed to bind to {}",addr));

    println!("─────────────────────────────────────");
    println!("  Ferrum running on http://{}", addr);
    println!("  Press Ctrl+C to stop");
    println!("─────────────────────────────────────");
 
    for stream in listner.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream,&mut router),
            Err(e)=> eprintln!("Connection error {}",e)
        }
    }
}


fn handle_connection(mut stream:TcpStream,router:&mut Router) {
    let mut buffer = [0u8;4096];
    //  so when data reads from the tcp stream 
    let bytes_read = match stream.read(&mut buffer) {
        Ok(n) => n,
        Err(e) => { eprintln!("Read error :{}",e); return;}
    };

    if bytes_read == 0 {
         return;
    }

    let raw_bytes = &buffer[..bytes_read];

    println!("\n ------ Incoming request ({} bytes)----",bytes_read); 
    println!("{}", String::from_utf8_lossy(raw_bytes));
    println!("---------------------------");

    // Parse Raw bytes ----> Request Struct 

    let request = Request::from_bytes(raw_bytes);
    println!("Parsed → method: {:?}  path: {}", request.method, request.path);

    // Dispatch to the Route

    let method_str = format!("{:?}", request.method);
    let path = request.path.clone();
    let ctx = Context::new(request);
    let ctx = router.dispatch(&method_str, &path, ctx);


    // serialize response to the bytes 

    let response_bytes = ctx.response.to_bytes();

     
    println!(
        "Response → {} ({} bytes)",
        ctx.response.status,
        response_bytes.len()
    );
 
    // ── Step 5: Write bytes back to TCP stream ───────────────────────
    if let Err(e) = stream.write_all(&response_bytes) {
        eprintln!("Write error: {}", e);
    }
}