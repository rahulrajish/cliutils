use std::fs;
use std::error::Error;
use crate::config::Config;
pub struct CatConfig {
    pub file_path_1: String,
    pub file_path_2: String,
}

impl Config for CatConfig {

    fn build(args: impl Iterator<Item = String>) ->  Result<Box<Self>, &'static str>{
        let mut inputs_iter = args;
        let file_path_1 = match inputs_iter.next() {
            Some(input) => input,
            None => return Err("Didn't get first input string"),
        };

        let file_path_2 = match inputs_iter.next() {
            Some(input) => input,
            None => return Err("Didn't get second input string"),
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