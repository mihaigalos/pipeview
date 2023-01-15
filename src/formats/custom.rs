use crate::formats::traits::Formatter;

use serde_derive::Deserialize;
use std::fs::File;
use std::io::Read;

const DEFAULT_CONFIG_IN_HOME_PATH: &str = ".config/pipeview.toml";

#[derive(Deserialize)]
struct Config {
    groups: String,
    colors: String,
}

pub struct Custom;

fn read_toml(path: &str) -> Config {
    let mut f = File::open(&path).expect("Cannot open config file.");
    let mut result = String::new();
    let _ = f.read_to_string(&mut result);
    let config: Config = toml::from_str(&result)
        .map_err(|_| format!("Failed to parse toml '{}'", path)).unwrap();
    config
}

impl Formatter for Custom {

    fn get_config<'a>() -> (String, String) {
         if let Some(mut path) = dirs::home_dir() {
            path.push(DEFAULT_CONFIG_IN_HOME_PATH);
            if path.exists() {
                let config = read_toml(&path.to_string_lossy());
                return (config.groups, config.colors)
            }
        }
        ("".to_string(), "".to_string())
    }

    fn print() {
        let (regex_groups, color) = Custom::get_config();
        println!("Custom");
        println!("Regex: {}", regex_groups);
        println!("Color: {}", color);
    }
}
