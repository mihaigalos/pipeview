use crate::formats::traits::Formatter;

pub struct Nginx;

impl Formatter for Nginx {
    fn get_config<'a>() -> (&'a str, &'a str) {
        let regex_groups = "^(.*?) - - \\[(.*?)\\] \"(.*?) .*?\" (.*?) .*? \".*?\" \"(.*?)\"";
        let colors = "bgreen white yellow cyan blue";

        (regex_groups, colors)
    }

    fn print() {
        let (regex_groups, color) = Nginx::get_config();
        println!("Nginx");
        println!("Regex: {}", regex_groups);
        println!("Color: {}", color);
    }
}
