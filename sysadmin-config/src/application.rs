extern crate serde;
use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Application {
	pub isinit        : bool,
	pub readtimeout   : i32,
	pub writetimeout : i32,
	pub host          : String,
	pub port          : String,
	pub name          : String,
	pub logpath       : String,
	pub env           : String,
	pub envmsg        : String,
}

impl Default for Application {
    
    fn default() -> Application {

        Application {
            isinit : false,
            readtimeout : 1,
            writetimeout : 2,
            host : "0.0.0.0".to_string(),
            port : "8000".to_string(),
            name : "testApp".to_string(),
            logpath : "temp/logs/log.txt".to_string(),
            env : "dev".to_string(),
            envmsg : "谢谢您的参与，但为了大家更好的体验，所以本次提交就算了吧".to_string(),
        }
        
    }
}