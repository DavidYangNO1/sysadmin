use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct CasbinRule {
    pub ptype : String,
    pub v0 : String,
    pub v1 : String,
    pub v2 : String,
    pub v3 : String,
    pub v4 : String,
    pub v5 : String,
}