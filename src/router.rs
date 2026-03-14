
use std::collections::HashMap;

use crate::{Context, Handler, http::response::Response};
struct Route{
    handler  : Handler
}


pub struct  Router{
    //  key is "METHOD /path"  eg GET/user
    routes : HashMap<String,Route>,
}

impl Router {
    pub fn new()-> Self{
        Self { 
            routes: HashMap::new()
        }
    }

    pub fn add(&mut self,method : &str,path:&str,handler: Handler){
        // take key 
        let key =  format!("{} {}",method.to_uppercase(),path);

        println!("↳ registered route: {}",key);

        self.routes.insert(key, Route { handler });
    }

    pub fn dispatch(&mut self, method : &str, path:&str, mut ctx:Context) ->Context {
        let key = format!("{} {}",method.to_string(),path.to_string());

        match self.routes.get(&key) {
            Some(route)=>{
                (route.handler)(&mut ctx);
            }

            None => {
                // no route found — 404
                ctx.response = Response::new();
                ctx.response.status = 404;
                ctx.response.body = format!(
                    "Cannot {} {}",
                    method.to_uppercase(),
                    path
                );
                ctx.response
                    .headers
                    .insert("Content-Type".to_string(), "text/plain".to_string());
            }
        }
        ctx
    }
}
