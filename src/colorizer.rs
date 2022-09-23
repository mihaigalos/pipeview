use colored::*;
use regex::Regex;

pub struct Colorizer;

pub fn colorize<'a>(
    input: &'a str,
    regex: &'a str,
    color: &'a str,
) -> Result<ColoredString, &'static str> {
    // let re = Regex::new(r"([0-9]+?)\.([0-9]+?)\.([0-9]+?)\.([0-9]+?)").unwrap();
    let re = Regex::new(regex).unwrap();
    let caps = re.captures(input);

    let result = caps.ok_or("Nope")?.get(1).ok_or("No element")?.as_str();

    let result = match color {
        "cyan" => result.cyan(),
        _ => ColoredString::from(result),
    };

    Ok(result.cyan())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colorize_works_when_typical() {
        let input = "abc de fgh";
        let expected = ColoredString::from("abc").cyan();

        let result = colorize(input, "(.*) (.*) (.*)", "cyan").unwrap();

        assert_eq!(result, expected);
    }
}
