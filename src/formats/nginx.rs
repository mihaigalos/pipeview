use crate::formats::traits::Formatter;

pub struct Nginx;

impl Formatter for Nginx {
    fn get_config() -> (String, String) {
        let regex_groups = "^(.*?) - - \\[(.*?)\\] \"(.*?) .*?\" (.*?) .*? \".*?\" \"(.*?)\"".to_string();
        let colors = "bgreen white yellow cyan blue".to_string();

        (regex_groups, colors)
    }

    fn print() {
        let (regex_groups, color) = Nginx::get_config();
        println!("Nginx");
        println!("Regex: {}", regex_groups);
        println!("Color: {}", color);
    }
}
