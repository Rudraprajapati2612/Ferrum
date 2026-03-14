use std::{collections::HashMap, path};

#[derive(Debug,Clone,PartialEq)]
pub enum  Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
    Unknown(String)
}

impl Method{
    pub fn from_str(s:&str) -> Self{
        match s {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "PATCH" => Method::PATCH,
            "DELETE" => Method::DELETE,
            "HEAD" => Method::HEAD,
            "OPTIONS" =>Method::OPTIONS,
            other => Method::Unknown(other.to_string())
        }
    }
}

#[derive(Debug,Clone)]
pub struct Request{
    pub method : Method,
    pub path  : String, // request path /users
    pub version : String, //HTTP version 
    pub headers : HashMap<String,String>,
    pub query_params : HashMap<String,String>,
    pub body : Option<String>,
    pub raw : Vec<u8> // raw bytes used for debuging 
}

impl Request{

    // empty  request
    pub fn empty()->Self{
        Self{
            method  : Method::GET,
            path : "/".to_string(),
            version : "HTTP/1.1".to_string(),
            headers : HashMap::new(),
            query_params : HashMap::new(),
            body : None,
            raw : Vec::new()
        }
    }

    // Parser For phase 1  -> Replace with real parser in future 

    pub fn from_bytes(bytes : &[u8]) -> Self{
        // create a new empty request 
        let mut req = Request::empty();
        
        // so what ever the way bytes is present in the request add it to the bytes variable   
        req.raw = bytes.to_vec();
        // convert the bytes to the string 
        
        let text = match std::str::from_utf8(bytes) {
            Ok(s) => s,
            Err(_) => return  req,
        };

        // let parse the first line 

        let first_line = match text.lines().next() {
            Some(l) => l,
            None => return req,
        };

        let mut parts = first_line.splitn(3, ' ');

        if let Some(method) = parts.next() { req.method = Method::from_str(method);}
        if let Some(path) = parts.next() {req.path = path.to_string();}
        if let Some(version) = parts.next() {req.version = version.to_string();}
        req
    }

    // so if out header contain token then key is token so 
   // in this it takes token as a input and it gives back a value that is presnt at that key 
    pub fn header(&self , name:&str) -> Option<&String>{
        self.headers.get(&name.to_lowercase())
    }
}