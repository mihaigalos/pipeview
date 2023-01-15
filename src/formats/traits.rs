pub trait Formatter {
    fn get_config<'a>() -> (String, String);
    fn print();
}
