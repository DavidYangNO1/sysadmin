use serde::{Serialize,Deserialize};
use basemodel;

#[derive(Serialize,Deserialize)]
pub struct Post {
    pub postId : i32,
    pub postName : String,
    pub postCode : String,
    pub sort : i32,
    pub status : String,
    pub remark : String,
    pub dataScope: String,
    pub params : String,
    pub baseMode : BaseMode,
}