
use crate::{http::{request::Request, response::{ Response}}, router::Router};

mod server;
mod router;

pub mod http;


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

pub type Handler = fn(&mut Context);


pub struct App {
    router: Router,
}
  
impl App {
    // Create a new Ferrum app
    pub fn new() -> Self {
        println!("⚙  Ferrum framework initialized");
        Self {
            router: Router::new(),
        }
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
 
    // Start the HTTP server on the given port
    pub fn listen(self, port: u16) {
        println!("🦀 Ferrum listening on http://127.0.0.1:{}", port);
        server::start(port, self.router);
    }
    
}