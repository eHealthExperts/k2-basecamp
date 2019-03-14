use crate::CONFIG;
use log::LevelFilter;
use log4rs;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::str::FromStr;
use std::sync::{Once, ONCE_INIT};

static INIT: Once = ONCE_INIT;

#[cfg(target_os = "windows")]
const FILENAME: &str = "ctehxk2.log";
#[cfg(not(target_os = "windows"))]
const FILENAME: &str = "libctehxk2.log";

pub fn init() {
    INIT.call_once(|| {
        let config = match CONFIG.log_path.clone() {
            Some(path) => init_file_logger(path),
            _ => init_stdout_logger(),
        };

        log4rs::init_config(config).expect("Failed to initialize logging!");
        info!("Logging initialized!");
    })
}

fn init_file_logger(mut path: String) -> Config {
    let appender_id = "file";
    path.push_str(FILENAME);

    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} {M}: {m}{n}")))
        .build(path)
        .expect("Failed to build file appender!");

    Config::builder()
        .appender(Appender::builder().build(appender_id, Box::new(file)))
        .logger(
            Logger::builder()
                .appender(appender_id)
                .additive(false)
                .build("ctehxk2", log_level()),
        )
        .build(
            Root::builder()
                .appender(appender_id)
                .build(LevelFilter::Error),
        )
        .expect("Failed to build config for file logger!")
}

fn init_stdout_logger() -> Config {
    let appender_id = "stdout";
    let stdout = ConsoleAppender::builder().build();

    Config::builder()
        .appender(Appender::builder().build(appender_id, Box::new(stdout)))
        .logger(
            Logger::builder()
                .appender(appender_id)
                .additive(false)
                .build("ctehxk2", log_level()),
        )
        .build(
            Root::builder()
                .appender(appender_id)
                .build(LevelFilter::Error),
        )
        .expect("Failed to build config for stdout logger!")
}

fn log_level() -> LevelFilter {
    match LevelFilter::from_str(&CONFIG.log_level) {
        Ok(log_level) => log_level,
        _ => LevelFilter::Error,
    }
}
