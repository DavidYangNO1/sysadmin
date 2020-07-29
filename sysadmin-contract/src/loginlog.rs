use serde::{Serialize,Deserialize};
use basemodel::BaseMode;
use chrono::DateTime;

#[derive(Serialize,Deserialize)]
pub struct LoginLog {
    pub infoid : i32,
    pub username : String,
    pub status : String,
    pub ipaddr : String,
    pub loginlocation : String,
    pub browser : String,
    pub os : String,
    pub platform : String,
    pub logintime : DateTime,
    pub basemode : BaseMode,
    pub datascope : String,
    pub params : String,
    pub remark : String,
    pub msg : String,
}