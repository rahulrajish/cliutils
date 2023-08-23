use std::error::Error;

pub trait Config{
    fn build(args: impl Iterator<Item = String>) -> Result<Box<Self>, &'static str>;
    fn run(&self) -> Result<(), Box<dyn Error>>;
}