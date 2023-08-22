use std::env;
use std::process;
use cliutils::run_config;

fn main() {
    run_config(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments : {err}");
        process::exit(1);
    });
}