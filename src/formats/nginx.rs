pub fn get_config<'a>() -> (&'a str, &'a str) {
    let regex_groups = "^(.*?) - - \\[(.*?)\\] \"(.*?) .*?\" (.*?) .*? \".*?\" \"(.*?)\"";
    let colors = "bgreen white yellow cyan blue";

    (regex_groups, colors)
}
