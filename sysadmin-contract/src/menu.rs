use serde::{Serialize,Deserialize};
use basemodel;

#[derive(Serialize,Deserialize)]
pub struct Menu {
    pub menuId : i32,
    pub muneName : String,
    pub title : String,
    pub icon : String,
    pub path : String,
    pub paths : String,
    pub menuType : String,
    pub action : String,
    pub permission : String,
    pub parentId : i32,
    pub noCache : bool,
    pub breadCrumb : String,
    pub component : String,
    pub sort : i32,
    pub visible : String,
    pub isFrame : String,
    pub dataScope : String,
    pub params : String,
    pub roleId : i32,
    pub children : &[Menu],
    pub isSelect : bool,
    pub baseMode : BaseMode,
}


#[derive(Serialize,Deserialize)]
pub struct  MenuLabel {
    pub id: i32,
    pub label: String,
    pub children :&[MenuLabel],
}

#[derive(Serialize,Deserialize)]
pub struct Menus {
    pub munuId : i32,
    pub menuName : String,
    pub title : String,
    pub icon : String,
    pub path : String,
    pub menuType : String,
    pub action : String,
    pub permission : String,
    pub parentId : i32,
    pub noCache : bool,
    pub breadCrumb: String,
    pub compoment : String,
    pub sort : i32,
    pub visible : String,
    pub children :&[menu],
    pub params : String,
    pub dadaScope: String,
    pub baseMode : BaseMode,
}


#[derive(Serialize,Deserialize)]
pub struct MenuRole {
    pub menus : Menus,
    pub isSelect : bool,
}
