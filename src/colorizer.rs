use colored::*;
use regex::Regex;

pub struct Colorizer;

pub fn colorize<'a>(
    input: &'a str,
    regex: &'a str,
    colors: &'a str,
) -> Result<ColoredString, &'static str> {
    let colors: Vec<&'a str> = colors.split(" ").collect();

    let re = Regex::new(regex).unwrap();
    let caps = re.captures(input).ok_or("Cannot apply regex")?;
    let mut result = "".to_string();

    let mut i = 0;
    for e in caps.iter() {
        if i == 0 {
            i = i + 1;
            continue;
        }
        let group = e.ok_or("No element")?.as_str();
        println!("{}", group);

        let colored_group = match colors[i] {
            "cyan" => group.cyan(),
            _ => ColoredString::from(""),
        };
        result += &colored_group;
    }

    Ok(result.cyan())
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
        let expected2 = ColoredString::from("de").cyan();

        let result: Vec<ColoredString> = colorize(input, "(.*) (.*)", "cyan magenta")
            .unwrap()
            .split(' ')
            .map(|s| ColoredString::from(s))
            .collect();

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
