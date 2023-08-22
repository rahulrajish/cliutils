use std::fs;
use std::error::Error;
pub trait Config{
    fn build(inputs: Vec<String>) -> Result<Box<Self>, &'static str>;
    fn run(&self) -> Result<(), Box<dyn Error>>;
}

pub struct EchoConfig {
    pub input: String,
}

pub struct CatConfig {
    pub file_path_1: String,
    pub file_path_2: String,
}

impl Config for EchoConfig {

    fn build(inputs: Vec<String>) -> Result<Box<Self>, &'static str> {
        let input = match inputs.into_iter().next() {
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

impl Config for CatConfig {

    fn build(inputs: Vec<String>) ->  Result<Box<Self>, &'static str>{
        let mut inputs_iter = inputs.into_iter();
        let file_path_1 = match inputs_iter.next() {
            Some(input) => input,
            None => return Err("Didn't get input string"),
        };

        let file_path_2 = match inputs_iter.next() {
            Some(input) => input,
            None => return Err("Didn't get input string"),
        };

        Ok(Box::new(CatConfig{file_path_1, file_path_2}))
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        let file_name_1 = match fs::read_to_string(&self.file_path_1) {
            Ok(file_path) => file_path,
            Err(e) => return Err(Box::new(e)),
        };

        let file_name_2 = match fs::read_to_string(&self.file_path_2) {
            Ok(file_path) => file_path,
            Err(e) => return Err(Box::new(e)),
        };

        println!("{0}",file_name_1 + &file_name_2);

        Ok(())
    }
}

pub fn run_config(mut args: impl Iterator<Item = String>) -> Result<(), &'static str> {
    args.next();

    let config_type = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get an input string"),
    };

    match config_type.as_str() {
        "echo" => {
            let mut inputs = Vec::new();
            match args.next() {
                Some(arg) => inputs.push(arg),
                None => return Err("Didn't get an input string"),
            }

            let echo_config = EchoConfig::build(inputs);

            let echo_run = match echo_config {
                Ok(echo) => *echo,
                Err(e) => return Err(e), 
            };

            echo_run.run().unwrap();
        },
        "cat" => {
            let mut inputs = Vec::new();
            match args.next() {
                Some(arg) => inputs.push(arg),
                None => return Err("Didn't get an input string"),
            }
            match args.next() {
                Some(arg) => inputs.push(arg),
                None => return Err("Didn't get an input string"),
            }

            let cat_config = CatConfig::build(inputs);

            let cat_run = match cat_config {
                Ok(cat) => *cat,
                Err(e) => return Err(e),
            };

            cat_run.run().unwrap();
        }
        _ => return Err("No operation found."),
    };

    Ok(())

}

