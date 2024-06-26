use crate::formats::traits::FormatterFromToml;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use toml::Value;

const DEFAULT_CONFIG: &str = "pipeview.toml";
const DEFAULT_CONFIG_IN_HOME_PATH: &str = ".config/pipeview.toml";

pub struct Custom;

fn extract_key<'a>(
    config: &'a HashMap<String, HashMap<String, String>>,
    key: &'a str,
) -> (bool, &'a str) {
    if config.contains_key(key) {
        return (true, key);
    }

    if config.keys().len() == 1 && key.is_empty() {
        return (true, (config.keys().next().unwrap()));
    }

    (false, "")
}

fn read_toml(path: &str) -> HashMap<String, HashMap<String, String>> {
    let mut result = HashMap::<String, HashMap<String, String>>::new();
    let mut f = File::open(path).expect("Cannot open config file.");
    let mut file_contents = String::new();
    let _ = f.read_to_string(&mut file_contents);
    let value: toml::Value = toml::from_str::<Value>(&file_contents).unwrap();
    match value {
        Value::Table(table) => {
            for (config_name, v) in table.iter() {
                let mut config_settings = HashMap::<String, String>::new();
                match v {
                    Value::Table(table) => {
                        for (k, v) in table.iter() {
                            let v = &v.to_string()[1..v.to_string().len() - 1]; // remove leading and trailing "
                            config_settings.insert(k.to_string(), v.to_string());
                        }
                        result.insert(config_name.to_string(), config_settings);
                    }
                    _ => todo!(),
                }
            }
        }
        _ => todo!(),
    }
    result
}

impl FormatterFromToml for Custom {
    fn get_config(custom_config_name: &str) -> (String, String) {
        fn extract_config_from_toml(path: &str, custom_config_name: &str) -> (String, String) {
            let config = read_toml(path);
            let (found, key) = extract_key(&config, custom_config_name);
            if found {
                let toml_settings = &config[key];
                return (
                    toml_settings["regex"].clone(),
                    toml_settings["colors"].clone(),
                );
            }
            ("".to_string(), "".to_string())
        }

        let mut path = std::env::current_dir().unwrap();
        path.push(DEFAULT_CONFIG);
        if path.exists() {
            return extract_config_from_toml(&path.to_string_lossy(), custom_config_name);
        } else if let Some(mut path) = dirs::home_dir() {
            path.push(DEFAULT_CONFIG_IN_HOME_PATH);
            if path.exists() {
                return extract_config_from_toml(&path.to_string_lossy(), custom_config_name);
            }
        }
        ("".to_string(), "".to_string())
    }

    fn print() {
        let (regex_groups, color) = Custom::get_config("foo");
        println!("Custom");
        println!("Regex: {regex_groups}");
        println!("Color: {color}");
    }
}
