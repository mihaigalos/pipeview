pub trait Formatter{
    fn get_config<'a>() -> (&'a str, &'a str);
    fn print();
}