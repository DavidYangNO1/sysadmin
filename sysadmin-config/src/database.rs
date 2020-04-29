extern crate serde;
use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub  struct DataBase {
    pub dbtype : String,
    pub host : String,
    pub port : i32,
    pub database : String,
    pub username : String,
    pub password : String,
}

impl Default for DataBase {
    
    fn default() -> DataBase {

        DataBase {
            dbtype : "mysql".to_string(),
            host :  "127.0.0.1".to_string(),
            port : 3306 ,
            database : "systemadmin".to_string(),
            username : "root".to_string(),
            password : "root".to_string(),
        }

    }
}