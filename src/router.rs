
use std::{collections::HashMap};

use crate::{Context, Handler, Middleware, http::response::Response, run_middleware_chain};


struct  Node{
    // what segemnt this node matching like "user" "helath" "posts"
    segment : String,
    // is this params node 
    is_params : bool,

    children : Vec<Node>,
    
    // handler registered at this node like 
    // key = http method  ("GET" , "POST")
    // value is a hanlder function 
    handlers : HashMap<String,Handler>
}

impl Node{
    pub fn new_literal(segment:&str) -> Self{
        Self{
            segment : segment.to_string(),
            is_params : false,
            children : Vec::new(),
            handlers : HashMap::new()
        }
    }

    pub fn new_params(params_name:&str)->Self{
        Self{
            segment : params_name.to_string(),
            is_params : true,
            children : Vec::new(),
            handlers : HashMap::new()
        }
    }
}


pub struct Router{
    root : Node
}

impl Router {
    pub fn new()->Self{
        Self { 
            root: Node::new_literal("")
        }
    }

    pub fn add(&mut self,method : &str ,path :&str,handler:Handler){

        println!("registered : {} {}",method.to_uppercase(),path);
        // split the path 
        let segments = split_path(path);


        //  start from root and move down 

        let mut current = &mut self.root;

        for segment in &segments {
            let is_params = segment.starts_with(':');

            let seg_name = if is_params{
                &segment[1..]
            }else{
                segment.as_str()
            };


            let child_pos = current.children.iter().position(|child|{
                child.is_params == is_params && child.segment == seg_name
            });

            match  child_pos {
                Some(pos) => {
                    // child exitst and goes into the child node 
                    current = &mut current.children[pos]
                }

                None => {
                    // if child node doesent exist then create it 
                    let new_node = if is_params {
                        Node::new_params(seg_name)
                    } else{
                        Node::new_literal(seg_name)
                    };
                    
                    // push Children to the new node 
                    current.children.push(new_node);
                    let last = current.children.len() -1;
                    current =&mut current.children[last];
                }
            }
            // attach handler to the Corresponding  method 
            

        }
        current.handlers.insert(method.to_uppercase(), handler);
        // traverst to the tree and find matching handelr 
        // collect the  params value in this along the way and 
        // return contex with params  populated + handler called 
        
    }
    pub fn dispatch(&self,method : &str,path:&str,mut ctx:Context,middlewares:&[Middleware]) -> Context{
        let segments = split_path(path); //seprate out path and stroed in vector

        match search(&self.root, &segments, 0, HashMap::new()) {
            
            // check for handler is present for the method and if not then return mathod is not presnt 

           Some((handler,params)) => {
            //  check for handler 
            match handler.get(&method.to_uppercase()) {
                Some(handler)=>{
                    // extract the request params 
                    ctx.request.params = params;
                    // run middleware chain and then handler 

                    let handler = *handler;
                    run_middleware_chain(middlewares, handler, &mut ctx);
                }
                None =>{
                    method_not_allowed(&mut ctx, method, path);
                }
            }
           }
           None => {
            not_found(&mut ctx, method, path);
           }
        } 

        ctx
        
    }
}


fn split_path(path:&str)->Vec<String>{
    path.split("/")
    .filter(|s| !s.is_empty()) // filter out the empty string
    .map(|s|s.to_string())
    .collect()
}

fn not_found(ctx:&mut Context,method:&str,path:&str){
    ctx.response = Response::new();
    ctx.response.status = 404;
    ctx.response.body = format!(
        r#"{{"error": "Cannot  {} {}"}}"#,
        method.to_uppercase(),
        path
    );
    
    ctx.response.headers.insert(
        "Content-Type".to_string(),
         "application/json".to_string()
    );
}

fn method_not_allowed(ctx:&mut Context,method:&str,path:&str){
    ctx.response = Response::new();
    ctx.response.status = 405;
    ctx.response.body = format!(
        r#"{{"error": "Method {} not allowed for {}"}}"#,
        method.to_uppercase(),
        path
    );

    
    ctx.response.headers.insert(
        "Content-Type".to_string(),
         "application/json".to_string()
    );

}

fn search<'a>(
    node : &'a Node, //current node 
    segments : &[String], // ["users","26","posts"]
    depth : usize, // current segment 
    params : HashMap<String,String>  
)->Option<(&'a HashMap<String,Handler>,HashMap<String,String>)>{
    // Base Case means if this hit traversal is done till all  the end 
    if depth == segments.len(){
        if node.handlers.is_empty(){
            return  None;
        }
        return Some((&node.handlers,params));
    }

    let current_segment = &segments[depth];

    // try literal children first 
    for child in &node.children {
        if !child.is_params && child.segment == current_segment.as_str() {
            if let Some(result) = search(child, segments, depth+1, params.clone()) {
                return Some(result);
            }
        }
    }

    // try parmas children 

    for child in &node.children {
        if child.is_params {
            let mut new_params = params.clone();
            new_params.insert(
                child.segment.clone()   // key is params name that is "id" 
            , current_segment.clone()   // valuie is present in current segment and that is 42
        );

        if let Some(result) = search(child, segments, depth+1, new_params){
            return  Some(result);
        }
        }
    }

    None


}