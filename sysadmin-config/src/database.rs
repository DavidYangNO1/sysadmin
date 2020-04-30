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
