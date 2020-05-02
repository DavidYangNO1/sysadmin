use serde::{Serialize,Deserialize};
use basemodel::BaseMode;

#[derive(Serialize,Deserialize)]
pub struct sysconfig {
    pub configid : i32,
    pub configname : String,
    pub configkey : String,
    pub  configvalue : String,
    pub  configtype : String,
    pub  remark : String,
    pub createby : String,
    pub updateby : String,
    pub datascope : String,
    pub params : String,
    pub basemode : BaseMode,
}