extern crate serde;
use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Jwt {
    pub secret : String,
    pub timeout : i64,
}

impl Default for Jwt {
    
    fn default() -> Jwt {
        Jwt {
            secret : "systemadmin".to_string(),
            timeout : 3600,
        }
    }
}