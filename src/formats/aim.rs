use crate::formats::traits::Formatter;

pub struct Aim;

impl Formatter for Aim {
    fn get_config() -> (String, String) {
        let regex_groups = "^(.*?) - - (.*?) \\[(.*?)\\] (.*?) (.*)".to_string();
        let colors = "bgreen white yellow cyan blue".to_string();

        (regex_groups, colors)
    }

    fn print() {
        let (regex_groups, color) = Aim::get_config();
        println!("Aim");
        println!("Regex: {regex_groups}");
        println!("Color: {color}");
    }
}
