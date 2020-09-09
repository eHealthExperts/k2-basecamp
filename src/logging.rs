use crate::CONFIG;
use log::LevelFilter;
use std::{str::FromStr, sync::Once};

static INIT: Once = Once::new();

#[cfg(windows)]
const FILENAME: &str = "ctehxk2.log";
#[cfg(unix)]
const FILENAME: &str = "libctehxk2.log";

pub fn init() {
    INIT.call_once(|| {
        fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{}[{}][{}] {}",
                    chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S:%f]"),
                    record.level(),
                    record.target(),
                    message
                ))
            })
            .level(log::LevelFilter::Error)
            .level_for("ctehxk2", determine_log_level())
            .chain(determine_logger())
            .apply()
            .expect("Failed to initialize logging!");
        info!("Logging initialized!");
    })
}

fn determine_logger() -> fern::Output {
    match &CONFIG.read().log_path {
        Some(path) => fern::log_file(format!("{}{}", path, FILENAME))
            .expect("Failed to open log file!")
            .into(),
        None => std::io::stdout().into(),
    }
}

fn determine_log_level() -> LevelFilter {
    match LevelFilter::from_str(&CONFIG.read().log_level) {
        Ok(log_level) => log_level,
        _ => LevelFilter::Error,
    }
}
