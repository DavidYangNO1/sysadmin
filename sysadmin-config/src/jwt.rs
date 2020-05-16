extern crate serde;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Jwt {
    pub secret: String,
    pub timeout: i64,
}
