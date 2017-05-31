extern crate base64;
extern crate cast;
extern crate envy;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod config;
pub mod http;
pub mod logging;

mod ctapi;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn CT_init(ctn: u16, pn: u16) -> i8 {
    logging::init();

    debug!("CT_init: Called (ctn {}, pn {})", ctn, pn);

    ctapi::init(config::ctn_or(ctn), config::pn_or(pn))
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn CT_data(ctn: u16,
                               dad: *mut u8,
                               sad: *mut u8,
                               lenc: usize,
                               command: *const u8,
                               lenr: *mut usize,
                               response: *mut u8)
                               -> i8 {
    logging::init();

    debug!("CT_data: Called (ctn {})", ctn);

    ctapi::data(config::ctn_or(ctn), dad, sad, lenc, command, lenr, response)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn CT_close(ctn: u16) -> i8 {
    logging::init();

    debug!("CT_close: Called (ctn {})", ctn);

    ctapi::close(config::ctn_or(ctn))
}
