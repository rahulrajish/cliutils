use std::fs;

pub struct Config {
    pub input_type: String,
    pub query_1: String,
    pub query_2: Option<String>,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>,) -> Result<Config, &'static str> {
        args.next();

        let input_type = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an input string"),
        };

        let query_1 = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let query_2 = args.next();

        //let file_name_2 = args.next();

        Ok(Config { input_type, query_1, query_2})

    }
}

pub fn run(config: Config) -> Result<(), &'static str> {
    let input_type = &config.input_type[..];
    match input_type {
        "echo" => println!("{0}", config.query_1),
        "cat" => {
            match concat(config) {
                Ok(str) => println!("{str}"),
                Err(e) => return Err(e),
            };
        }
        _ => return Err("Unable to process input"),
    };

    Ok(())
}

fn concat(config: Config) -> Result<String, &'static str>{
    let file_name_1 = config.query_1;
    
    let file_name_2 = match config.query_2 {
        Some(file_name) => file_name,
        None => return Err("Didn't get filePath string"),
    };

    let file_name_1 = match fs::read_to_string(file_name_1) {
        Ok(file_path) => file_path,
        Err(_) => return Err("Error while reading file from file path 1"),
    };
    let file_name_2 = match fs::read_to_string(file_name_2) {
        Ok(file_path) => file_path,
        Err(_) => return Err("Error while reading file from file path 2"),
    };

    Ok(file_name_1 + &file_name_2)

}