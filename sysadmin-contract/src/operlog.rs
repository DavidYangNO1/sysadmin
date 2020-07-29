use serde::{Serialize,Deserialize};
use chrono::DateTime;
use basemodel;

#[derive(Serialize,Deserialize)]
pub struct SysOperLog {
    pub operId : i32,
    pub title : String,
    pub businessType: String,
    pub businessTypes: String,
    pub method : String,
    pub requestMethod: String,
    pub operatorType: String,
    pub operName : String,
    pub deptName : String,
    pub operUrl : String,
    pub operIp : String,
    pub operLocation: String,
    pub operParam : String,
    pub status : String,
    pub operTime : DateTime,
    pub jsonResult : String,
    pub dataScope : String,
    pub params : String,
    pub remark : String,
    pub latencyTime : String,
    pub userAgent : String,
    pub baseMode: BaseMode,
}