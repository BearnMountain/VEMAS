// use serde::{Deserialize, Serialize};
//
// #[derive(Deserialize)]
// pub struct Config {
//
// }
//
// static CONFIG: Config;
//
// impl Config {
//     pub fn init() {
//         let contents = std::fs::read_to_string(path).expect("Failed to read config.toml");
//
//         CONFIG = toml::from_str(&contents).expect("Failed to parse {path}");
//     }
// }
