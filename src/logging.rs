extern crate log;
extern crate log4rs;

use log::LogLevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::sync::{Once, ONCE_INIT};
use super::config;

static INIT: Once = ONCE_INIT;

pub fn init() {
    INIT.call_once(|| init_logger());
}

fn init_logger() {
    let config = match config::log_path() {
        Some(path) => init_file_logger(String::from(path)),
        _ => init_stdout_logger(),
    };

    log4rs::init_config(config).unwrap();
}

fn init_file_logger(mut path: String) -> Config {
    path.push_str("ctehxk2.log");

    let appender_id = "file";

    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} {M}: {m}{n}")))
        .build(path)
        .unwrap();

    Config::builder()
        .appender(Appender::builder().build(appender_id, Box::new(file)))
        .logger(Logger::builder()
                    .appender(appender_id)
                    .additive(false)
                    .build("ctehxk2", LogLevelFilter::Debug))
        .build(Root::builder()
                   .appender(appender_id)
                   .build(LogLevelFilter::Error))
        .unwrap()
}

fn init_stdout_logger() -> Config {
    let stdout = ConsoleAppender::builder().build();

    let appender_id = "stdout";

    Config::builder()
        .appender(Appender::builder().build(appender_id, Box::new(stdout)))
        .logger(Logger::builder()
                    .appender(appender_id)
                    .additive(false)
                    .build("ctehxk2", LogLevelFilter::Debug))
        .build(Root::builder()
                   .appender(appender_id)
                   .build(LogLevelFilter::Error))
        .unwrap()
}
