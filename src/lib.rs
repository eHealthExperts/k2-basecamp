extern crate base64;
extern crate envy;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate log4rs;
#[cfg(test)]
extern crate mockito;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;

mod config;
mod ctapi;
mod http;
mod logging;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn CT_init(ctn: u16, pn: u16) -> i8 {
    logging::init();

    debug!("CT_init(ctn: {}, pn: {})", ctn, pn);
    let status = ctapi::init(config::ctn_or(ctn), config::pn_or(pn));

    debug!("Returning {}", status);
    status.to_i8()
}

#[no_mangle]
#[allow(non_snake_case)]
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
    let status = ctapi::data(config::ctn_or(ctn), dad, sad, lenc, command, lenr, response);

    debug!("Returning {}", status);
    status.to_i8()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn CT_close(ctn: u16) -> i8 {
    logging::init();

    debug!("CT_close(ctn: {})", ctn);
    let status = ctapi::close(config::ctn_or(ctn));

    debug!("Returning {}", status);
    status.to_i8()
}
