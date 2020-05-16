#[cfg(feature = "yaml")]
extern crate config;
use config::*;
extern crate serde;

#[path = "settings.rs"]
pub mod settings;
use settings::Settings;

pub fn parseyaml(filename: &String) -> Settings {
    let mut config = Config::default();
    config.merge(File::new(filename, FileFormat::Yaml)).unwrap();
    let s: Settings = config.try_into().unwrap();
    s
}
