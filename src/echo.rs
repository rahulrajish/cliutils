use std::error::Error;
use crate::config::Config;

pub struct EchoConfig {
    pub input: String,
}

impl Config for EchoConfig {

    fn build(args: impl Iterator<Item = String>) -> Result<Box<Self>, &'static str> {
        let mut inputs = args;
        let input = match inputs.next() {
            Some(input) => input,
            None => return Err("Didn't get input string"),
        };
        Ok(Box::new(EchoConfig { input }))
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("{0}", self.input);

        Ok(())
    }
}