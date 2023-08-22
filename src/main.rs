use std::env;
use std::process;
use cliutils::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments : {err}");
        process::exit(1);
    });

    cliutils::run(config).unwrap_or_else(|err|{
        eprintln!("Problem while running command : {err}");
        process::exit(1);
    });
}