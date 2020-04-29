extern crate serde;
extern crate serde_json;

use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Queryable)]
pub struct SysColumns {
    pub columnId : i32,
    pub tableId : i32,
    pub columnName : String,
    pub columnComment : String,
    pub columnType : String,
    pub goType : String,
    pub goField : String,
    pub jsonField : String,
    pub isPk : String,
    pub isIncrement : String,
    pub isRequired : String,
    pub isInsert : String,
    pub isEdit : String,
    pub isList : String,
    pub isQuery : String,
    pub queryType : String,
    pub htmlType : String,
    pub dictType : String,
    pub sort : i32,
    pub list : String,
    pub pk : bool,
    pub required : bool,
    pub superColumn : bool,
    pub usableColumn : bool,
    pub increment : bool,
    pub insert : bool,
    pub edit : bool,
    pub query : bool,
    pub remark : String,
    pub createBy : String,
    pub updateBy : String,
}