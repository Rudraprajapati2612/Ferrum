use std::collections::HashMap;


// This is used for serialize it convert (struct->bytes)
// because again it need to pass it to the anothe layer 
#[derive(Debug,Clone)]
pub struct Response{
    pub status : u16,
    pub headers : HashMap<String,String>,
    pub body : String
}


impl Response{
    pub fn new() -> Self{
        let mut headers = HashMap::new();

        headers.insert("Connection".to_string(), "keep-alive".to_string());
        headers.insert("X-powred-By".to_string(), "Ferrium".to_string());

        Self{
            status : 200,
            headers,
            body : String::new()
        }
    }

    pub fn reason_phrases(status:u16) -> &'static str{
        match  status {
            200 => "OK",
            201 => "Created",
            204 => "No Content",
            301 => "Moved Permanently",
            302 => "Found",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            422 => "Unprocessable Entity",
            429 => "Too Many Requests",
            500 => "Internal Server Error",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            _   => "Unknown",
        }
    }

    // Serialize the response to the raw http bytes 
    // so we send this to the tcp stream to send the response to the client that is requested by the user

    pub fn to_bytes(&self) -> Vec<u8>{

        let mut out = Vec::new();

         
        
        // So http repsonse contain 
        //  this four major part
        // STATUS LINE ---> HTTP/1.1 200 OK \r\n
        // HEADERS ------> Content-Type : text/plain
        // (blank line) --> \r\n
        // BODY -> hello 

        let status_line = format!("HTTP/1.1 {} {}\r\n",self.status,Self::reason_phrases(self.status));

        // covnert the response into bytes
        out.extend_from_slice(status_line.as_bytes());

        //  parse headers and 
       
       for (key,value) in &self.headers {
        let header_line = format!("{}: {}\r\n", key, value);  // ← remove space before :
        out.extend_from_slice(header_line.as_bytes());
       }

    //  Calculate Content length 

     let content_length = format!("Content-Length: {}\r\n", self.body.len());
     out.extend_from_slice(content_length.as_bytes());

    //  convert blank line to bytes 

    out.extend_from_slice(b"\r\n");
    // convert body to bytes
    out.extend_from_slice(self.body.as_bytes()); 
    
    out  
    }
}