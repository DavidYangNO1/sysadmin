use serde::{Serialize,Deserialize};
use syscolumns;

#[derive(Serialize,Deserialize)]
pub struct SysTables {
    pub tableId : i32,
    pub tableName : String,
    pub tableComment : String,
    pub className : String,
    pub tplCategory : String,
    pub packageName : String,
    pub moduleName : String,
    pub businessName : String,
    pub functionName : String,
    pub functionAuthor : String,
    pub pkColumn : String,
    pub pkGoField : String,
    pub pkJsonField : String,
    pub options : String,
    pub treeCode : String,
    pub treeParentCode : String,
    pub treeName : String,
    pub tree : bool,
    pub crud : bool,
    pub remark : String,
    pub isLogicalDelete : String,
    pub LogicalDelete : bool,
    pub logicalDeleteColumn : String,
    pub createBy : String,
    pub updateBy : String,
    pub dataScope : String,
    pub params : Params,
    pub columns : &[syscolumns],
}

#[derive(Serialize,Deserialize,Queryable)]
pub struct Params {
    pub treeCode : String,
    pub treeParentCode : String,
    pub treeName :  String
}