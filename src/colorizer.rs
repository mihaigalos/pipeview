use colored::*;
use regex::Regex;

pub struct Colorizer;

pub fn colorize<'a>(
    input: &'a str,
    regex: &'a str,
    colors: &'a str,
) -> Result<Vec<ColoredString>, &'static str> {
    let colors: Vec<&'a str> = colors.split(" ").collect();

    let re = Regex::new(regex).unwrap();
    let caps = re.captures(input).ok_or("Cannot apply regex")?;
    let mut result: Vec<ColoredString> = vec![];

    let caps = all_captures_except_first(caps)?;

    for (pos, e) in caps.iter().enumerate() {
        let colored_group = match colors[pos] {
            "cyan" => e.cyan(),
            "magenta" => e.magenta(),
            _ => ColoredString::from(""),
        };
        println!("{} -> {}", e, colored_group);
        result.push(colored_group);
    }

    Ok(result)
}

fn all_captures_except_first<'a>(input: regex::Captures) -> Result<Vec<String>, &'static str> {
    let mut result: Vec<String> = vec![];
    let mut i = 0;
    for e in input.iter() {
        if i == 0 {
            i = i + 1;
            continue;
        }
        let element = e.ok_or("No element")?.as_str();
        result.push(element.to_string());
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colorize_works_when_typical() {
        let input = "abc de";
        let expected1 = ColoredString::from("abc").cyan();
        let expected2 = ColoredString::from("de").magenta();

        let result: Vec<ColoredString> = colorize(input, "(.*) (.*)", "cyan magenta").unwrap();

        assert_eq!(result[0], expected1);
        assert_eq!(result[1], expected2);
    }
    #[test]
    fn test_all_except_first_works_when_typical() {
        let expected: Vec<String> = vec!["ab".to_string(), "cd".to_string()];
        let re = Regex::new("(.*) (.*)").unwrap();
        let caps = re.captures("ab cd").ok_or("Cannot apply regex").unwrap();
        let result = all_captures_except_first(caps).unwrap();
        assert_eq!(result, expected);
    }
}
