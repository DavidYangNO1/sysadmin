use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct DataPermission struct {
    datascope : i32,
    userid : i32,
    deptid : i32,
    roleid : i32,
}