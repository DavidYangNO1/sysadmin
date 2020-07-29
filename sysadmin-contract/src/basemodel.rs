use serde::{Serialize,Deserialize};
use chrono::DateTime;

#[derive(Serialize,Deserialize)]
pub struct BaseMode {
    pub createAt : DateTime,
    pub updateAt : DateTime,
    pub deleteAt : DateTime,
}