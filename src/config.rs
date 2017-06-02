extern crate envy;

use std::env::var;
use std::path::MAIN_SEPARATOR;

static BASE_URL_KEY: &str = "K2_BASE_URL";
static DEFAULT_BASE_URL: &str = "http://localhost:8080/k2/ctapi/";
static LOG_PATH_KEY: &str = "K2_LOG_PATH";

#[derive(Deserialize)]
struct Config {
    #[serde(rename = "k2_ctn")]
    ctn: u16,
    #[serde(rename = "k2_pn")]
    pn: u16,
}

pub fn base_url() -> String {
    let mut url = var(BASE_URL_KEY).unwrap_or(DEFAULT_BASE_URL.to_string());
    if !url.trim().ends_with("/") {
        url.push_str("/");
    }

    url
}

pub fn log_path() -> Option<String> {
    match var(LOG_PATH_KEY) {
        Ok(mut path) => {
            if !path.trim().ends_with(MAIN_SEPARATOR) {
                path.push(MAIN_SEPARATOR);
            }

            Some(path)
        }
        _ => None,
    }
}

pub fn ctn_or(ctn: u16) -> u16 {
    match envy::from_env::<Config>() {
        Ok(config) => {
            debug!("From env: ctn {}", config.ctn);
            config.ctn
        }
        _ => ctn,
    }
}

pub fn pn_or(pn: u16) -> u16 {
    match envy::from_env::<Config>() {
        Ok(config) => {
            debug!("From env: pn {}", config.pn);
            config.pn
        }
        _ => pn,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn ctn_or_return_env_value_if_pn_is_set_too() {
        env::set_var("K2_CTN", "2");
        env::set_var("K2_PN", "4");
        let ctn = ctn_or(1);

        assert_eq!(ctn, 2);
    }

    #[test]
    fn ctn_or_return_given_value_if_pn_not_set() {
        env::remove_var("K2_CTN");
        env::remove_var("K2_PN");
        let ctn = ctn_or(1);

        assert_eq!(ctn, 1);

        env::set_var("K2_CTN", "2");
        let ctn = ctn_or(1);

        assert_eq!(ctn, 1);
    }

    #[test]
    fn pn_or_return_env_value_if_ctn_is_set_too() {
        env::set_var("K2_CTN", "2");
        env::set_var("K2_PN", "4");
        let pn = pn_or(1);

        assert_eq!(pn, 4);
    }

    #[test]
    fn pn_or_return_given_value_if_ctn_not_set() {
        env::remove_var("K2_CTN");
        env::remove_var("K2_PN");
        let pn = pn_or(1);

        assert_eq!(pn, 1);

        env::set_var("K2_PN", "2");
        let pn = pn_or(1);

        assert_eq!(pn, 1);
    }

    #[test]
    fn base_url_return_env_value() {
        env::set_var(BASE_URL_KEY, "a/");
        let url = base_url();

        assert_eq!(url, "a/");

        env::set_var(BASE_URL_KEY, "1");
        let url = base_url();

        assert_eq!(url, "1/");
    }

    #[test]
    fn base_url_return_default_value_if_no_env() {
        env::remove_var(BASE_URL_KEY);
        let url = base_url();

        assert_eq!(url, DEFAULT_BASE_URL);
    }

    #[test]
    fn log_path_returns_env_value() {
        env::set_var(LOG_PATH_KEY, "a");
        let path = log_path();

        assert_eq!(path, Some(String::from("a/")));
    }

    #[test]
    fn log_path_return_none_if_no_env() {
        env::remove_var(LOG_PATH_KEY);
        let path = log_path();

        assert!(path.is_none());
    }
}
