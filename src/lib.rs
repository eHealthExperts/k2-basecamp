extern crate antidote;
extern crate base64;
extern crate config;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate log4rs;
#[macro_use]
extern crate log;
#[cfg(test)]
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[cfg(test)]
extern crate test_server;
extern crate tokio_core;

mod settings;
mod ctapi;
mod http;
mod logging;

use ctapi::close::close;
use ctapi::data::data;
use ctapi::init::init;
use ctapi::status::Status;
use settings::Settings;

#[no_mangle]
pub extern "system" fn CT_init(ctn: u16, pn: u16) -> i8 {
    logging::init();

    debug!("CT_init(ctn: {}, pn: {})", ctn, pn);
    let status = init(Settings::ctn_or(ctn), Settings::pn_or(pn));

    debug!("Returning {}", status);
    status.into()
}

#[no_mangle]
pub extern "system" fn CT_data(
    ctn: u16,
    dad: *mut u8,
    sad: *mut u8,
    lenc: u16,
    command: *const u8,
    lenr: *mut u16,
    response: *mut u8,
) -> i8 {
    logging::init();

    debug!("CT_data(ctn: {})", ctn);
    let status = data(
        Settings::ctn_or(ctn),
        dad,
        sad,
        lenc,
        command,
        lenr,
        response,
    );

    debug!("Returning {}", status);
    status.into()
}

#[no_mangle]
pub extern "system" fn CT_close(ctn: u16) -> i8 {
    logging::init();

    debug!("CT_close(ctn: {})", ctn);
    let status = close(Settings::ctn_or(ctn));

    debug!("Returning {}", status);
    status.into()
}

pub extern "system" fn env() {
    println!(
        "Path used to load config file: {}",
        std::env::current_dir()
            .expect("Failed to get current dir!")
            .to_str()
            .expect("Failed to create string of path!")
    );
}
