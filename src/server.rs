use std::io::{Read,Write};

use std::net::{TcpListener,TcpStream};




fn handle_connection(mut stream:TcpStream) {
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

    
     
}