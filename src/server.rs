use std::io::{Read,Write};

use std::net::{TcpListener,TcpStream};

use crate::Context;
use crate::http::request::Request;
use crate::router::Router;



pub fn start(port:u16,mut router:Router){
    let addr = format!("127.0.0.1:{}",port);

    let listner = TcpListener::bind(&addr).expect(&format!("failed to bind to {}",addr));

    println!("─────────────────────────────────────");
    println!("  Ferrum running on http://{}", addr);
    
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
    //  so when data reads from the tcp stream it comes in bytes and this bytes read return size and it say till which 
    // index data is present 
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

    let request = match Request::from_bytes(raw_bytes){
        Ok(r) => r,
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
            let _ = stream.write_all(
                b"HTTP/1.1 400 Bad Request\r\nContent-Length: 11\r\n\r\nBad Request"
            );
            return;
        }  
    };

     // print what was parsed - great for learning
     println!("Method  -> {}", request.method.as_str());
     println!("Path    -> {}", request.path);
     if !request.query_params.is_empty() {
         println!("Query   -> {:?}", request.query_params);
     }
     if !request.headers.is_empty() {
         println!("Headers -> {:?}", request.headers);
     }
     if let Some(body) = &request.body {
         println!("Body    -> {}", body);
     }

    // serialize response to the bytes 

   // Step 3: Dispatch to router
   let method_str = request.method.as_str().to_string();
   let path       = request.path.clone();
   let ctx        = Context::new(request);
   let ctx        = router.dispatch(&method_str, &path, ctx);

   let response_bytes = ctx.response.to_bytes();
   println!("Response -> {} ({} bytes)\n", ctx.response.status, response_bytes.len());
     
   
 
    // ── Step 5: Write bytes back to TCP stream ───────────────────────
    if let Err(e) = stream.write_all(&response_bytes) {
        eprintln!("Write error: {}", e);
    }
}