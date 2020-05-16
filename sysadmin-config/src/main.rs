#[path = "config.rs"]
mod configparse;
use configparse::parseyaml;
use configparse::settings::Settings;

fn main() {
    println!("parse yaml file!");

    let s: Settings = parseyaml(&"src/settings.yaml".to_string());

    println!("app is :{:?}", s.app);

    println!("database is :{:?}", s.database);

    println!("jwt is :{:?}", s.jwt);
}
