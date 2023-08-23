use std::error::Error;

mod config;
use config::Config;

mod echo;
use echo::EchoConfig;

mod cat;
use cat::CatConfig;

mod error;
use error::ConfigError;

pub fn run_config(mut args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
    args.next();

    let config_type = match args.next() {
        Some(arg) => arg,
        None => return Err(Box::new(ConfigError::new( "Didn't get an input string"))),
    };

    match config_type.as_str() {
        "echo" => {
            let echo_config = EchoConfig::build(args);

            let echo_run = match echo_config {
                Ok(echo) => *echo,
                Err(e) => return Err(Box::new(ConfigError::new(e))), 
            };

            match echo_run.run() {
                Ok(()) => (),
                Err(e) => return Err(e),
            }
        },
        "cat" => {
            let cat_config = CatConfig::build(args);

            let cat_run = match cat_config {
                Ok(cat) => *cat,
                Err(e) => return Err(Box::new(ConfigError::new(e))),
            };

            match cat_run.run() {
                Ok(()) => (),
                Err(e) => return Err(e),
            }
        }
        _ => return Err(Box::new(ConfigError::new("No operation found."))),
    };

    Ok(())

}

