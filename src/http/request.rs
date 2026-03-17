use std::{collections::{HashMap}, usize};

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


    pub fn as_str(&self)-> &str{
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::PATCH => "PATCH",
            Method::DELETE => "DELETE",
            Method::HEAD => "HEAD",
            Method::OPTIONS => "OPTIONS",
            Method::Unknown(s) => s.as_str()
        }
    }
}
#[derive(Debug)]
pub enum ParseError{
    EmptyRequest,
    NotEnoughData, // missing \r\n or we can say body is missing 
    InvalidRequestLine, // in this first line must contain METHOD PATH VERSION if from this any of the thing is not present  then this error occur
    InvalidUtf8 // not valid bytes
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
  

    pub fn from_bytes(bytes : &[u8]) -> Result<Self,ParseError>{
        //  check for empty bytes 

        if bytes.is_empty() {
            return Err(ParseError::EmptyRequest);
        }

        //  Split header section from the body section by finding \r\n\r\n
        let split_pos = find_header_end(bytes).ok_or(ParseError::NotEnoughData)?;

        // before balnk there is header section 
        let header_section  = &bytes[..split_pos];
        // after blank body section 

        let body_section = &bytes[split_pos+ 4..];
        
        // convert the header text to the string

        let header_text = std::str::from_utf8(header_section).map_err(|_| ParseError::InvalidUtf8)?;
        
        // split the line  
        let mut lines = header_text.split("\r\n");

        // after spiliting the line parse first line that is (Request line)
        //  "GET /user HTTP/1.1" "\r\n" ---> this is the response from the split line 
        let request_line = lines.next().ok_or(ParseError::InvalidRequestLine)?;

        let (method,raw_path,version) = parse_request_line(request_line)?;

        //  Split path from query string
        let (path,query_params) = parse_query_and_string(&raw_path);
        //  parse the header 

        let headers = parse_header(lines);
  // parse the body 
        let body = parse_body(body_section, &headers);
        
        Ok(Request {
            method,
            path,
            version,
            headers,
            query_params,
            body,
            raw: bytes.to_vec(),
        })
    }


   
    // so if our header contain token then key is token so 
   // in this it takes token as a input and it gives back a value that is presnt at that key 
    pub fn header(&self , name:&str) -> Option<&String>{
        self.headers.get(&name.to_lowercase())
    }

    pub fn query(&self, key: &str) -> Option<&String> {
        self.query_params.get(key)
    }
}

pub fn find_header_end(buffer : &[u8])->Option<usize>{
    buffer.windows(4).position(|window| window == b"\r\n\r\n")
}

pub fn parse_request_line(line : &str) -> Result<(Method,String,String),ParseError>{
    let mut parts = line.splitn(3, ' ');

    let method = parts.next().ok_or(ParseError::InvalidRequestLine)?;
    let path = parts.next().ok_or(ParseError::InvalidRequestLine)?;
    let version  = parts.next().ok_or(ParseError::InvalidRequestLine)?;

 
    Ok((
        Method::from_str(method),
        path.to_string(),
        version.trim().to_string(),
    ))
}

pub fn parse_query_and_string(raw_path :&str) -> (String,HashMap<String,String>){
    let mut parts = raw_path.splitn(2, '?');
    let path = parts.next().unwrap_or("/").to_string();
    let query = parts.next().unwrap_or(""); 

    (path,parse_query_string(query))
}

pub fn parse_query_string(query : &str) -> HashMap<String,String>{
    let mut map = HashMap::new();

    if query.is_empty() {return  map;}

    for pairs in query.split('&'){
        let mut kv = pairs.splitn(2, '=');
        let key = kv.next().unwrap_or("").to_string();
        let value = kv.next().unwrap_or("").to_string();

        if !key.is_empty() {
            map.insert(key, value);
        }
    }

    map
}

 fn parse_header<'a>(lines:impl Iterator <Item = &'a str>) -> HashMap<String,String>{
    let mut map = HashMap::new();

    for line in lines {
        if line.is_empty() {continue;}

        let mut parts = line.splitn(2, ':');
        let key = parts.next().unwrap_or("").trim().to_lowercase();
        let value = parts.next().unwrap_or("").trim().to_string();

        if !key.is_empty(){
            map.insert(key, value);
        }
    }
    map
 } 

 fn parse_body(body_section:&[u8],headers:&HashMap<String,String>) -> Option<String>{
    let content_length = headers.get("content-length")?.parse::<usize>().ok()?;

    if content_length == 0 || body_section.is_empty() {
        return None
    }

    let body_bytes = if body_section.len()>=content_length{
        &body_section[..content_length]
    }else {
        body_section
    };

    std::str::from_utf8(body_bytes).ok().map(|s| s.to_string())
 }


#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn test_get_request_no_body() {
        let raw = b"GET /users HTTP/1.1\r\nHost: localhost:8080\r\nAccept: application/json\r\n\r\n";
        let req = Request::from_bytes(raw).unwrap();
 
        assert_eq!(req.method,  Method::GET);
        assert_eq!(req.path,    "/users");
        assert_eq!(req.version, "HTTP/1.1");
        assert_eq!(req.header("host"),   Some(&"localhost:8080".to_string()));
        assert_eq!(req.header("accept"), Some(&"application/json".to_string()));
        assert_eq!(req.body, None);
    }
 
    #[test]
    fn test_post_request_with_body() {
        let body = r#"{"name": "Rudra", "age": 20}"#;
        let raw  = format!(
            "POST /users HTTP/1.1\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(), body
        );
        let req = Request::from_bytes(raw.as_bytes()).unwrap();
 
        assert_eq!(req.method, Method::POST);
        assert_eq!(req.path,   "/users");
        assert_eq!(req.body,   Some(body.to_string()));
    }
 
    #[test]
    fn test_query_params() {
        let raw = b"GET /users?page=1&limit=10 HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let req = Request::from_bytes(raw).unwrap();
 
        assert_eq!(req.path,          "/users");
        assert_eq!(req.query("page"),  Some(&"1".to_string()));
        assert_eq!(req.query("limit"), Some(&"10".to_string()));
    }
 
    #[test]
    fn test_find_header_end() {
        let buf = b"GET / HTTP/1.1\r\nHost: x\r\n\r\nbody";
        let pos = find_header_end(buf).unwrap();
        assert_eq!(&buf[pos..pos+4], b"\r\n\r\n");
    }
 
    #[test]
    fn test_missing_header_end() {
        let buf = b"GET / HTTP/1.1\r\nHost: x\r\n";
        assert!(find_header_end(buf).is_none());
    }
}