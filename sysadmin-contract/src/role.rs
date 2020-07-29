use serde::{Serialize,Deserialize};
use basemodel;

#[derive(Serialize,Deserialize)]
pub struct SysRole {
    pub roleId : i32,
    pub roleName: String,
    pub status : String,
    pub roleKey : String,
    pub roleSort : i32,
    pub flag : String,
    pub remark : String,
    pub admin : bool,
    pub dataScope : String,
    pub params : String,
    pub menuIds : &[i32],
    pub deptIds : &[i32],
    pub baseMode : BaseModel,
}