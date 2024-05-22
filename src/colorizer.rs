use colored::*;
use regex::Regex;

pub fn run <'a>(
    input: &'a str,
    regex: &'a str,
    colors: &'a str,
) -> Result<Vec<ColoredString>, &'static str> {

    if regex.len() > 0 && colors.len() > 0 {
        return colorize(input, regex, colors);
    } else {
        let num_commas = input.chars().filter(|&c| c == ',').count();
        //if num_commas > 0
        {
            let mut regex: String = "(.*)".to_string();
            let possible_colors: Vec<String> = vec!["green".to_string(), "cyan".to_string(), "red".to_string(), "white".to_string(), "yellow".to_string() ];
            let mut colors: String = possible_colors[0].clone();
            for i in 0..num_commas {
              regex.push_str(",(.*)");
              colors.push_str(" ");
              colors.push_str(&possible_colors[(i+1) % possible_colors.len()]);
              println!("--> regex: {}  colors: {}", regex, colors);
            }
            colorize(input, &regex, &colors)
        }
    }
}

fn colorize<'a>(
    input: &'a str,
    regex: &'a str,
    colors: &'a str,
) -> Result<Vec<ColoredString>, &'static str> {
    let colors: Vec<&'a str> = colors.split(' ').collect();

    let re = Regex::new(regex).unwrap();
    let caps = re.captures(input);
    if caps.is_none() {
        return Ok(vec![ColoredString::from("abc")]);
    }
    let mut result: Vec<ColoredString> = vec![];

    let binding = caps.unwrap();
    let caps = all_captures_except_first(&binding)?;

    if colors.len() != caps.len() {
        panic!(
            "Length of colors: {} != length of regex match patterns: {}",
            colors.len(),
            caps.len()
        );
    }

    for (pos, e) in caps.iter().enumerate() {
        let colored_group = match colors[pos] {
            "bblue" => e.bright_blue(),
            "bcyan" => e.bright_cyan(),
            "bgreen" => e.bright_green(),
            "blue" => e.blue(),
            "bmagenta" => e.bright_magenta(),
            "bred" => e.bright_red(),
            "byellow" => e.bright_yellow(),
            "cyan" => e.cyan(),
            "green" => e.green(),
            "magenta" => e.magenta(),
            "red" => e.red(),
            "white" => e.white(),
            "yellow" => e.yellow(),
            _ => ColoredString::from(""),
        };
        print!("{colored_group} ");
        result.push(colored_group);
    }

    Ok(result)
}

fn all_captures_except_first<'a>(input: &'a regex::Captures) -> Result<Vec<&'a str>, &'static str> {
    let mut result: Vec<&str> = vec![];
    let mut i = 0;
    for e in input.iter() {
        if i == 0 {
            i += 1;
            continue;
        }
        let element = e.ok_or("No element")?.as_str();
        result.push(element);
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
        let result = all_captures_except_first(&caps).unwrap();
        assert_eq!(result, expected);
    }
    #[test]
    fn test_run_works_when_empty_regex_and_colors() {
        let input = "abc,de";
        let expected1 = ColoredString::from("abc").green();
        let expected2 = ColoredString::from("de").cyan();

        let result: Vec<ColoredString> = run(input, "", "").unwrap();

        assert_eq!(result[0], expected1);
        assert_eq!(result[1], expected2);
    }
}
