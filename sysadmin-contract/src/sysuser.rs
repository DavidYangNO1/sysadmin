use serde::{Serialize,Deserialize};
use basemodel;

#[derive(Serialize,Deserialize)]
pub struct  User {
    pub identiryKey : String,
    pub userName: String,
    pub firstName : String,
    pub lastName : String,
    pub role : String,
}

#[derive(Serialize,Deserialize)]
pub struct  UserName {
    pub userName : String,
}

#[derive(Serialize,Deserialize)]
pub struct  Password {
    pub password : String,
}

#[derive(Serialize,Deserialize)]
pub struct LoginM {
    pub userName : UserName,
    pub password : Password,
}

#[derive(Serialize,Deserialize)]
pub struct SysUserId {
    pub userId : i32,
}

#[derive(Serialize,Deserialize)]
pub struct  SysUserB {
    pub nickName: String,
    pub phone : String,
    pub roleId : i32,
    pub salt : String,
    pub avatar : String,
    pub sex : String ,
    pub email : String ,
    pub deptId : i32,
    pub postId : i32,
    pub remark : String,
    pub status : String,
    pub dataScope : String,
    pub params : String,
    pub baseModel : BaseModel,
}
#[derive(Serialize,Deserialize)]
pub struct SysUser {
    pub sysUserId : SysUserId,
    pub sysUserB : SysUserB,
    pub loginM : LoginM,
}

#[derive(Serialize,Deserialize)]
pub struct SysUserPwd {
    pub oldPassword : String ,
    pub newPassword : Stirng ,
}


#[derive(Serialize,Deserialize)]
pub struct SysUserPage {
    pub sysUserId : SysUserId,
    pub sysUserB : SysUserB,
    pub loginM : LoginM, 
    pub deptName : String,
}

#[derive(Serialize,Deserialize)]
pub struct SysUserView {
    pub sysUserId : SysUserId,
    pub sysUserB : SysUserB,
    pub loginM : LoginM, 
    pub roleName : String,
}

