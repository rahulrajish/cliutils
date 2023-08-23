use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct ConfigError{
    details: String,
}

impl ConfigError {
    pub fn new(msg: &str) -> ConfigError {
        ConfigError { details: msg.to_string() }
    }
}

impl fmt::Display for ConfigError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ConfigError {
    fn description(&self) -> &str {
        &self.details
    }
}