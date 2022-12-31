use crate::formats::traits::Formatter;

pub struct Aim;

impl Formatter for Aim {
    fn get_config<'a>() -> (&'a str, &'a str) {
        let regex_groups = "^(.*?) - - (.*?) \\[(.*?)\\] (.*?) (.*)";
        let colors = "bgreen white yellow cyan blue";

        (regex_groups, colors)
    }

    fn print() {
        let (regex_groups, color) = Aim::get_config();
        println!("Aim");
        println!("Regex: {}", regex_groups);
        println!("Color: {}", color);
    }
}
