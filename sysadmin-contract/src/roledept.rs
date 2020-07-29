use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct  SysRoleDept {
    pub roleId : i32,
    pub deptId : i32,
}

