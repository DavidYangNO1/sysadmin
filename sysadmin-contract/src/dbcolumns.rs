use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct DBColumns  {
    pub tableScheme : String,
    pub tableName : String,
    pub columnName : String,
    pub columnDefault : String,
    pub isNullAble : String,
    pub dataType : String,
    pub characterMaximumLength : String,
    pub characterSetName : String,
    pub columnType : String,
    pub columnKey : String,
    pub extra : String,
    pub columnComment : String,
}
