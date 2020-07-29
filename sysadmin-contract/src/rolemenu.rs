use serde::{Serialize,Deserialize};
use basemodel;

#[derive(Serialize,Deserialize)]
pub struct  RoleMenu {
    pub releId : i32,
    pub menuId : i32,
    pub roleName : String,
    pub baseMode : BaseModel,
}