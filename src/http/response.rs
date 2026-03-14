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
            200 => "Ok",
            201 => "Created",
            204 => "No Content",
            301 => "Moved Permantely",
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

        let content_length = self.body.len();
        
        // So http repsonse contain 
        //  this four major part
        // STATUS LINE
        // HEADERS
        // (blank line)
        // BODY

        let status_line = format!("HTTP/1.1 {} {}",self.status,Self::reason_phrases(self.status));

        let mut headers_str = String::new();

        for(key,vaule) in &self.headers {
            headers_str.push_str(&format!("{}:{}\r\n",key,vaule));
        }

        headers_str.push_str(&format!("Content-Length :{}\r\n",content_length));

        headers_str.push_str("\r\n");

        let mut bytes = Vec::new();

        bytes.extend_from_slice(status_line.as_bytes());
        bytes.extend_from_slice(headers_str.as_bytes());
        bytes.extend_from_slice(self.body.as_bytes());
        
        bytes
    }
}