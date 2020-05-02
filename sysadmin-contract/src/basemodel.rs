use serde::{Serialize,Deserialize};
use chrono::DateTime;

#[derive(Serialize,Deserialize)]
pub struct BaseMode {
    pub createat : DateTime,
    pub updateat : DateTime,
    pub deleteat : DateTime,
}