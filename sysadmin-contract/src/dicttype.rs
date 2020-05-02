use serde::{Serialize,Deserialize};

use basemodel::BaseMode;

#[derive(Serialize,Deserialize)]
pub struct DictType {
    pub dictid : i32,
    pub dictname : String,
    pub dicttype : String,
    pub status : String,
    pub datascope : String,
    pub params : String,
    pub remark : String,
    pub basemode : BaseMode,
}