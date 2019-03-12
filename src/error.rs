use std::{error, fmt, result};

use config::ConfigError;
use reqwest::UrlError;

pub type Result<T> = result::Result<T, BasecampError>;

#[derive(Debug)]
pub enum BasecampError {
    SettingsError(ConfigError),
    UrlError(UrlError),
}

impl fmt::Display for BasecampError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BasecampError::SettingsError(ref err) => write!(f, "Settings error: {}", err),
            BasecampError::UrlError(ref err) => write!(f, "URL error: {}", err),
        }
    }
}

impl error::Error for BasecampError {
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            BasecampError::SettingsError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<config::ConfigError> for BasecampError {
    fn from(err: config::ConfigError) -> BasecampError {
        BasecampError::SettingsError(err)
    }
}

impl From<reqwest::UrlError> for BasecampError {
    fn from(err: reqwest::UrlError) -> BasecampError {
        BasecampError::UrlError(err)
    }
}
