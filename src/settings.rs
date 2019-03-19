use config::{Config, Environment, File};
use failure::Error;
use reqwest::Url;
use std::path::{Path, MAIN_SEPARATOR};

#[cfg(target_os = "windows")]
const CFG_FILE: &str = "ctehxk2";
#[cfg(not(target_os = "windows"))]
const CFG_FILE: &str = "libctehxk2";

#[derive(Debug, Deserialize, PartialEq)]
pub struct Settings {
    pub timeout: Option<u64>,
    pub base_url: String,
    pub log_level: String,
    pub log_path: Option<String>,
    pub ctn: Option<u16>,
    pub pn: Option<u16>,
}

impl Settings {
    pub fn init() -> Result<Self, Error> {
        let mut settings = Config::new();

        // set defaults
        settings
            .set_default("base_url", "http://localhost:8088/k2/ctapi/")
            .expect("Failed to set default for base_url!")
            .set_default("log_level", "Error")
            .expect("Failed to set default for log_level!");

        // merge with optional config file and env variables
        settings
            .merge(File::with_name(CFG_FILE).required(false))
            .expect("Failed to merge config file!")
            .merge(Environment::with_prefix("K2").ignore_empty(true))
            .expect("Failed to merge env variables!");

        // force trailing slash for base_url
        if let Ok(mut url) = settings.get::<String>("base_url") {
            let _ = Url::parse(&url)?; // check url

            if !url.trim().ends_with('/') {
                url.push_str("/");
                let _ = settings.set("base_url", url);
            }
        }

        // force trailing slash for log_path
        if let Ok(Some(mut path)) = settings.get::<Option<String>>("log_path") {
            if !Path::new(&path).exists() {
                return Err(format_err!("log_path does not exists"));
            }

            if !path.trim().ends_with(MAIN_SEPARATOR) {
                path.push(MAIN_SEPARATOR);
                let _ = settings.set("log_path", Some(path));
            }
        }

        // enforce a value for pn and ctn
        if let (Ok(Some(_)), Ok(Some(_))) = (
            settings.get::<Option<u16>>("ctn"),
            settings.get::<Option<u16>>("pn"),
        ) {
            // ok
        } else {
            let _ = settings.set("ctn", None::<String>);
            let _ = settings.set("pn", None::<String>);
        }

        settings.try_into().map_err(failure::Error::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand;
    use spectral::assert_that;
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::path::MAIN_SEPARATOR;
    use tempfile::tempdir;
    use test_server::helper::random_string;

    #[test]
    fn default_configuration() {
        let default = Settings::init();

        assert_eq!(
            default.ok(),
            Some(Settings {
                timeout: None,
                base_url: String::from("http://localhost:8088/k2/ctapi/"),
                log_level: String::from("Error"),
                log_path: None,
                ctn: None,
                pn: None,
            })
        );
    }

    #[test]
    fn env_variables_overrides() {
        let timeout = u64::from(rand::random::<u32>()); // panics with u64 random
        env::set_var("K2_TIMEOUT", format!("{}", timeout));

        assert_eq!(
            Settings::init().ok(),
            Some(Settings {
                timeout: Some(timeout),
                base_url: String::from("http://localhost:8088/k2/ctapi/"),
                log_level: String::from("Error"),
                log_path: None,
                ctn: None,
                pn: None,
            })
        );

        let base_url = String::from("ftp://unknown.de/");
        env::set_var("K2_BASE_URL", base_url.clone());

        assert_eq!(
            Settings::init().ok(),
            Some(Settings {
                timeout: Some(timeout),
                base_url: base_url.clone(),
                log_level: String::from("Error"),
                log_path: None,
                ctn: None,
                pn: None,
            })
        );

        let log_level = random_string(10);
        env::set_var("K2_LOG_LEVEL", log_level.clone());

        assert_eq!(
            Settings::init().ok(),
            Some(Settings {
                timeout: Some(timeout),
                base_url: base_url.clone(),
                log_level: log_level.clone(),
                log_path: None,
                ctn: None,
                pn: None,
            })
        );

        let config_file_folder_path = tempdir().unwrap().into_path();
        let log_path = format!(
            "{}{}",
            config_file_folder_path.to_str().unwrap(),
            MAIN_SEPARATOR
        );
        env::set_var("K2_LOG_PATH", log_path.clone());

        assert_eq!(
            Settings::init().ok(),
            Some(Settings {
                timeout: Some(timeout),
                base_url: base_url.clone(),
                log_level: log_level.clone(),
                log_path: Some(log_path.clone()),
                ctn: None,
                pn: None,
            })
        );

        let ctn = rand::random::<u16>();
        env::set_var("K2_CTN", format!("{}", ctn));

        assert_eq!(
            Settings::init().ok(),
            Some(Settings {
                timeout: Some(timeout),
                base_url: base_url.clone(),
                log_level: log_level.clone(),
                log_path: Some(log_path.clone()),
                ctn: None,
                pn: None,
            })
        );

        let pn = rand::random::<u16>();
        env::set_var("K2_PN", format!("{}", pn));

        assert_eq!(
            Settings::init().ok(),
            Some(Settings {
                timeout: Some(timeout),
                base_url: base_url.clone(),
                log_level: log_level.clone(),
                log_path: Some(log_path.clone()),
                ctn: Some(ctn),
                pn: Some(pn),
            })
        );

        env::remove_var("K2_BASE_URL");
        env::remove_var("K2_CTN");
        env::remove_var("K2_LOG_LEVEL");
        env::remove_var("K2_LOG_PATH");
        env::remove_var("K2_PN");
        env::remove_var("K2_TIMEOUT");
    }

    #[test]
    fn config_file_overrides() {
        let config_file_folder = tempdir().unwrap();
        let config_file_folder_path = tempdir().unwrap().into_path();
        let log_path = format!(
            "{}{}",
            config_file_folder_path.to_str().unwrap(),
            MAIN_SEPARATOR
        );

        let config_file_path = config_file_folder.path().join(format!("{}.json", CFG_FILE));
        let mut config_file = File::create(config_file_path.clone()).unwrap();
        let _ = env::set_current_dir(config_file_folder.path().to_owned());

        let config = json!({
            "log_level": "debug",
            "log_path": log_path,
            "base_url": "http://localhost:5050/",
            "timeout": 1000,
            "ctn": 9,
            "pn": 12,
        });

        writeln!(config_file, "{}", config).unwrap();

        assert_eq!(
            Settings::init().ok(),
            Some(Settings {
                timeout: config["timeout"].as_u64(),
                base_url: String::from(config["base_url"].as_str().unwrap()),
                log_level: String::from(config["log_level"].as_str().unwrap()),
                log_path: Some(String::from(config["log_path"].as_str().unwrap())),
                ctn: Some(
                    config["ctn"]
                        .as_u64()
                        .unwrap()
                        .to_string()
                        .parse::<u16>()
                        .unwrap()
                ),
                pn: Some(
                    config["pn"]
                        .as_u64()
                        .unwrap()
                        .to_string()
                        .parse::<u16>()
                        .unwrap()
                ),
            })
        );
    }

    #[test]
    fn env_variable_beats_config_file() {
        let config_file_folder = tempdir().unwrap();
        let config_file_path = config_file_folder.path().join(format!("{}.yaml", CFG_FILE));
        let mut config_file = File::create(config_file_path.clone()).unwrap();
        let _ = env::set_current_dir(config_file_folder.path().to_owned());

        let config = "
timeout: 300
log_level: debug
";

        writeln!(config_file, "{}", config).unwrap();

        env::set_var("K2_LOG_LEVEL", "trace");

        assert_eq!(
            Settings::init().ok(),
            Some(Settings {
                timeout: Some(300),
                base_url: String::from("http://localhost:8088/k2/ctapi/"),
                log_level: String::from("trace"),
                log_path: None,
                ctn: None,
                pn: None,
            })
        );

        env::remove_var("K2_LOG_LEVEL");
    }

    #[test]
    fn force_trailing_slash_for_base_url() {
        let url = "http://127.0.0.1";
        env::set_var("K2_BASE_URL", url);

        assert_eq!(
            Settings::init().ok(),
            Some(Settings {
                timeout: None,
                base_url: format!("{}/", url),
                log_level: String::from("Error"),
                log_path: None,
                ctn: None,
                pn: None,
            })
        );

        env::remove_var("K2_BASE_URL");
    }

    #[test]
    fn force_trailing_slash_for_log_path() {
        let path = "/tmp";
        env::set_var("K2_LOG_PATH", path);

        assert_eq!(
            Settings::init().ok(),
            Some(Settings {
                timeout: None,
                base_url: String::from("http://localhost:8088/k2/ctapi/"),
                log_level: String::from("Error"),
                log_path: Some(format!("{}/", path)),
                ctn: None,
                pn: None,
            })
        );

        env::remove_var("K2_LOG_PATH");
    }

    #[test]
    fn read_config_from_ini_file() {
        let config_file_folder = tempdir().unwrap();
        let config_file_path = config_file_folder.path().join(format!("{}.ini", CFG_FILE));
        let mut config_file = File::create(config_file_path.clone()).unwrap();
        let _ = env::set_current_dir(config_file_folder.path().to_owned());

        let config = "
timeout = 300
log_level = \"debug\"
";

        writeln!(config_file, "{}", config).unwrap();

        assert_eq!(
            Settings::init().ok(),
            Some(Settings {
                timeout: Some(300),
                base_url: String::from("http://localhost:8088/k2/ctapi/"),
                log_level: String::from("debug"),
                log_path: None,
                ctn: None,
                pn: None,
            })
        );
    }

    #[test]
    fn error_with_wrong_log_path() {
        let path = random_string(100);
        env::set_var("K2_LOG_PATH", path);

        assert!(Settings::init().is_err());

        env::remove_var("K2_LOG_PATH");
    }

    #[test]
    fn error_with_wrong_base_url() {
        let url = random_string(100);
        env::set_var("K2_BASE_URL", url);

        assert!(Settings::init().is_err());

        env::remove_var("K2_BASE_URL");
    }

    #[test]
    fn enforce_ctn_and_pn_were_set() {
        let mut settings = Settings::init().unwrap();
        assert_that(&settings)
            .map(|val| &val.ctn)
            .is_equal_to(&None);
        assert_that(&settings).map(|val| &val.pn).is_equal_to(&None);

        let ctn = rand::random::<u16>();
        env::set_var("K2_CTN", format!("{}", ctn));

        settings = Settings::init().unwrap();
        assert_that(&settings)
            .map(|val| &val.ctn)
            .is_equal_to(&None);
        assert_that(&settings).map(|val| &val.pn).is_equal_to(&None);

        env::remove_var("K2_CTN");

        let pn = rand::random::<u16>();
        env::set_var("K2_PN", format!("{}", pn));

        settings = Settings::init().unwrap();
        assert_that(&settings)
            .map(|val| &val.ctn)
            .is_equal_to(&None);
        assert_that(&settings).map(|val| &val.pn).is_equal_to(&None);

        env::set_var("K2_CTN", format!("{}", ctn));

        settings = Settings::init().unwrap();
        assert_that(&settings)
            .map(|val| &val.ctn)
            .is_equal_to(&Some(ctn));
        assert_that(&settings)
            .map(|val| &val.pn)
            .is_equal_to(&Some(pn));

        env::remove_var("K2_CTN");
        env::remove_var("K2_PN");
    }
}
