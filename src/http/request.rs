#[derive(Debug,Clone)]
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

#[derive(Debug,Clone)]
pub struct Request{
    pub method : Method,
}