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
    INIT.call_once(|| match var("K2_LOG_PATH") {
                       Ok(path) => {
        let file = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d} {l} {M}: {m}{n}")))
            .build(path + &"/ctehxk2.log".to_string())
            .unwrap();

        let config = Config::builder()
            .appender(Appender::builder().build("file", Box::new(file)))
            .logger(Logger::builder()
                        .appender("file")
                        .additive(false)
                        .build("ctehxk2", LogLevelFilter::Debug))
            .build(Root::builder()
                       .appender("file")
                       .build(LogLevelFilter::Error))
            .unwrap();

        log4rs::init_config(config).unwrap();
    }
                       _ => {
        let stdout = ConsoleAppender::builder().build();

        let config = Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .logger(Logger::builder()
                        .appender("stdout")
                        .additive(false)
                        .build("ctehxk2", LogLevelFilter::Debug))
            .build(Root::builder()
                       .appender("stdout")
                       .build(LogLevelFilter::Error))
            .unwrap();

        log4rs::init_config(config).unwrap();
    }
                   })
}
