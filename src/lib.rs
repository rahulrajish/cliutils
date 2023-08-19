pub struct Config {
    pub input: String,
    pub query: String,
    pub file_path: Option<String>,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>,) -> Result<Config, &'static str> {
        args.next();

        let input = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an input string"),
        };

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = args.next();

        Ok(Config { input, query, file_path })

    }
}

pub fn run(config: Config) {
    let input_type = &config.input[..];
    match input_type {
        "echo" => println!("{0}", config.query),
        _ => (),
    };
}