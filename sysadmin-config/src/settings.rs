extern crate serde;
use serde::Deserialize;

#[path = "application.rs"]
mod application;
use application::Application;

#[path = "database.rs"]
mod database;
use database::DataBase;

#[path = "jwt.rs"]
mod jwt;
use jwt::Jwt;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub app: Application,
    pub database: DataBase,
    pub jwt: Jwt,
}
