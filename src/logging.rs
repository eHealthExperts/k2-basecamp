extern crate log;
extern crate log4rs;

use log::LogLevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::env::var;
use std::sync::{Once, ONCE_INIT};

static INIT: Once = ONCE_INIT;

pub fn init() {
    INIT.call_once(|| init_logger());
}

fn init_logger() {
    let config = match var("K2_LOG_PATH") {
        Ok(path) => init_file_logger(String::from(path)),
        _ => init_stdout_logger(),
    };

    log4rs::init_config(config).unwrap();
}

fn init_file_logger(mut path: String) -> Config {
    if !path.trim().ends_with("/") {
        path.push_str("/");
    }

    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} {M}: {m}{n}")))
        .build(path + &"ctehxk2.log".to_string())
        .unwrap();

    Config::builder()
        .appender(Appender::builder().build("file", Box::new(file)))
        .logger(Logger::builder()
                    .appender("file")
                    .additive(false)
                    .build("ctehxk2", LogLevelFilter::Debug))
        .build(Root::builder()
                   .appender("file")
                   .build(LogLevelFilter::Error))
        .unwrap()
}

fn init_stdout_logger() -> Config {
    let stdout = ConsoleAppender::builder().build();

    Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder()
                    .appender("stdout")
                    .additive(false)
                    .build("ctehxk2", LogLevelFilter::Debug))
        .build(Root::builder()
                   .appender("stdout")
                   .build(LogLevelFilter::Error))
        .unwrap()
}
