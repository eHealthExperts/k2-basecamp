extern crate antidote;
extern crate base64;
extern crate config;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate log4rs;
#[cfg(test)]
extern crate rand;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[cfg(test)]
extern crate test_server;

mod ctapi;
mod http;
mod logging;
mod settings;

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
