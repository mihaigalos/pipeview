pub trait Formatter {
    fn get_config() -> (String, String);
    fn print();
}

pub trait FormatterFromToml {
    fn get_config(custom_config_name: &str) -> (String, String);
    fn print();
}
