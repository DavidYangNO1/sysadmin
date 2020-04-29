extern crate serde;
extern crate serde_json;

use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Queryable)]
pub struct DBTables {
    pub tableName : String,
    pub engine : String,
    pub tableRows : String,
    pub tableCollation : String,
    pub createTime : String,
    pub updateTime : String,
    pub tableComment : String,
}