use serde::{Serialize,Deserialize};
use basemodel::BaseMode;

#[derive(Serialize,Deserialize)]
pub struct Dept {
    pub deptid : i32,
    pub parentid : i32,
    pub deptpath : String,
    pub  deptname : String,
    pub  sort : i32,
    pub  leader : String,
    pub phone : String,
    pub email : String,
    pub  status : String,
    pub datascope : String,
    pub params : String,
    pub  children : [Dept],
    pub basemode : BaseMode,
}