
use std::{collections::HashMap, ops::Sub};

use crate::{http::{request::Request, response::{ Response}}, router::Router};

mod server;
mod router;

pub mod http;




//  Fnonce(&mut Context)
// Fnonce take contex as a input and called only once 

// dyn FnOnce(..) ---> it means something unknkown at compile time 

//  Box<....> ---> Storing the function on the heap because size is unknown 
//  'a is a lifetime that lives till middleware call

pub type Next<'a> = Box<dyn FnOnce(&mut Context) + 'a>;  // in simple terms it take contex as a input and it run once and memory is allocate dynamically on heap  


//  this is the function point which takes that takes &mut Contex and next as inpu and return nothing

//  Function point  is used because it is easy to store and no heap allocation 
pub type Middleware = fn(&mut Context,Next);

pub type Handler = fn(&mut Context);

pub struct Context{
    pub request : Request,
    pub response : Response,
}

impl Context{
    pub fn new(request:Request) -> Self{
        Self{
            request,
            response : Response::new()
        }
    }
    // send a plain text response 
    pub fn send (&mut self,status:u16,body:&str){
        self.response.status = status;
        self.response.body = body.to_string();
        self.response.headers.insert("Content-Type".to_string(), "text/plain".to_string());
    }

    // send a json response 
    pub fn json(&mut self , status:u16,body:&str){
        self.response.status = status;
        self.response.body = body.to_string();

        self.response.headers.insert("Content-Type".to_string(), "application/json".to_string());
    }
    
    // send Not found 
    pub fn not_found(&mut self,message:&str){
        self.json(404, &format!(r#"{{"error": "{}"}}"#,message));
    }

    // send bad request 
    pub fn bad_request(&mut self,message:&str){
        self.json(400, &format!(r#"{{"error": "{}"}}"#,message));
    }
    // send unauthorized 
    pub fn unauthorized(&mut self, message: &str) {
        self.json(401, &format!(r#"{{"error": "{}"}}"#, message));
    }   
    
    // send internal error 
    pub fn internal_error(&mut self, message: &str) {
        self.json(500, &format!(r#"{{"error": "{}"}}"#, message));
    }

    pub fn forbidden(&mut self, message: &str) {
        self.json(403, &format!(r#"{{"error": "{}"}}"#, message));
    }

    pub fn redirect(&mut self,status:u16,location:&str){
        self.response.status = status;
        self.response.body = String::new();

        self.response.headers.insert("Location".to_string(), location.to_string());
    }

    pub fn set_header(&mut self , key : &str,value :&str){
        self.response.headers.insert(key.to_string(), value.to_string());
    }
}


pub struct SubRouter{
    routes : Vec<(String,String,Handler)>
}

impl SubRouter{
    pub fn new() -> Self {
        Self { 
            routes: Vec::new()
        }
    }

    pub fn get(&mut self,path:&str,handler:Handler){
        self.routes.push(("GET".to_string(),path.to_string(),handler));
    }

    pub fn post(&mut self, path: &str, handler: Handler) {
        self.routes.push(("POST".to_string(), path.to_string(), handler));
    }
 
    pub fn put(&mut self, path: &str, handler: Handler) {
        self.routes.push(("PUT".to_string(), path.to_string(), handler));
    }
 
    pub fn delete(&mut self, path: &str, handler: Handler) {
        self.routes.push(("DELETE".to_string(), path.to_string(), handler));
    }
 
    pub fn patch(&mut self, path: &str, handler: Handler) {
        self.routes.push(("PATCH".to_string(), path.to_string(), handler));
    }
}
pub struct App {
    router: Router,
    middlewares : Vec<Middleware>
}
  
impl App {
    // Create a new Ferrum app
    pub fn new() -> Self {
        println!("⚙  Ferrum framework initialized");
        Self {
            router: Router::new(),
            middlewares : Vec::new()
        }
    }
    
    pub fn use_middleware(&mut self,middleware : Middleware){
        self.middlewares.push(middleware);
    }
    // Register a GET route
    pub fn get(&mut self, path: &str, handler: Handler) {
        self.router.add("GET", path, handler);
    }
 
    // Register a POST route
    pub fn post(&mut self, path: &str, handler: Handler) {
        self.router.add("POST", path, handler);
    }
 
    // Register a PUT route
    pub fn put(&mut self, path: &str, handler: Handler) {
        self.router.add("PUT", path, handler);
    }
 
    // Register a DELETE route
    pub fn delete(&mut self, path: &str, handler: Handler) {
        self.router.add("DELETE", path, handler);
    }
    
    pub fn mount(&mut self,prefix:&str,sub_router:SubRouter){
        println!("Mounting routes at prefix: {}", prefix);
 
        for (method, path, handler) in sub_router.routes {
            // combine prefix + sub path
            let full_path = if path == "/" {
            //    if in path contain "/" then it directly attach prefix to the path 
                prefix.to_string()
            } else {
                
                // → registers as "/api/v1/auth/login"
                // else api/v1/auth is prefix and /login is path then path to the prefix 
                format!("{}{}", prefix, path)
            };
 
            self.router.add(&method, &full_path, handler);
        }
    }
    // Start the HTTP server on the given port
    pub async fn listen(self, port: u16) {
        println!("🦀 Ferrum listening on http://127.0.0.1:{}", port);
        server::start(port, self.router,self.middlewares).await;
    }
    

   
}



pub fn run_middleware_chain(
    middlewares : &[Middleware],
    handler: Handler,
    ctx: &mut Context
){
    if middlewares.is_empty() {
        handler(ctx);
        return;
    }

    // take first middleware as a current and other as a remaning 
    let current = middlewares[0];
    let remanings = &middlewares[1..];


    //  build the next function and 


    // initialy it next is like this 
    // run_middleware_chain([logger, auth, request_id], handler, ctx)
    // then logger(ctx,next)  will run and 
    // then logger function will execute
    // run_middleware_chain([ auth, request_id], handler, ctx)
    // auth(ctx,next) ---> will run 
    // run_middleware_chain([request_id], handler, ctx) at last request id will run 
    let next:Next  = Box::new(|ctx:&mut Context|{
        run_middleware_chain(remanings,handler,ctx)
    });
    
    current(ctx,next);
}