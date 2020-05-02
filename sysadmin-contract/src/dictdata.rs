use serde::{Serialize,Deserialize};
use basemodel::BaseMode;

#[derive(Serialize,Deserialize)]
pub struct DictData {
    pub dictcode : i32,
    pub dictsort : i32,
    pub dictlabel : String,
    pub dictvalue : String,
    pub dicttype : String,
    pub cssclass : String,
    pub listclass : String,
    pub isdefault : String，
    pub status : String,
    pub default : String,
    pub remark : String ,
    pub params : String,
    pub datascope : String,
    pub basemode : BaseMode,
}