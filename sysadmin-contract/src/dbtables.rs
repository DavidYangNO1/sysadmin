use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct DBTables {
    pub tableName : String,
    pub engine : String,
    pub tableRows : String,
    pub tableCollation : String,
    pub createTime : String,
    pub updateTime : String,
    pub tableComment : String,
}