#![deny(unused_features)]
#![deny(deprecated)]
#![warn(unused_variables)]
#![warn(unused_imports)]
#![warn(dead_code)]
#![warn(missing_copy_implementations)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod ctapi;
mod http;
mod logging;
mod settings;

use crate::ctapi::close::close;
use crate::ctapi::data::data;
use crate::ctapi::init::init;
use crate::ctapi::status::Status;
use crate::settings::Settings;
use antidote::RwLock;
use std::panic;

lazy_static! {
    pub(crate) static ref CONFIG: RwLock<Settings> =
        RwLock::new(Settings::init().expect("Failed to init configuration!"));
}

#[no_mangle]
pub extern "system" fn CT_init(ctn: u16, pn: u16) -> i8 {
    logging::init();

    debug!("CT_init(ctn: {}, pn: {})", ctn, pn);
    let status: i8 = match init(ctn, pn) {
        Ok(status) => status.value(),
        Err(why) => {
            error!("Failure during CT_init!");
            debug!("{}", why);
            Status::ERR_HTSI.value()
        }
    };

    debug!("Returning {}", status);
    status
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

    if dad.is_null() {
        error!("Null pointer passed into CT_data() as dad");
        return Status::ERR_HTSI.value();
    }

    if sad.is_null() {
        error!("Null pointer passed into CT_data() as sad");
        return Status::ERR_HTSI.value();
    }

    if lenr.is_null() {
        error!("Null pointer passed into CT_data() as lenr");
        return Status::ERR_HTSI.value();
    }

    if response.is_null() {
        error!("Null pointer passed into CT_data() as response");
        return Status::ERR_HTSI.value();
    }

    debug!("CT_data(ctn: {})", ctn);
    let status: i8 =
        match panic::catch_unwind(|| data(ctn, dad, sad, lenc, command, lenr, response)) {
            Ok(result) => match result {
                Ok(status) => status.value(),
                Err(why) => {
                    error!("Failure during CT_data!");
                    debug!("{}", why);
                    Status::ERR_HTSI.value()
                }
            },
            Err(_) => {
                error!("Caught panic!");
                Status::ERR_HTSI.value()
            }
        };

    debug!("Returning {}", status);
    status
}

#[no_mangle]
pub extern "system" fn CT_close(ctn: u16) -> i8 {
    logging::init();

    debug!("CT_close(ctn: {})", ctn);
    let status = match close(ctn) {
        Ok(status) => status.value(),
        Err(why) => {
            error!("Failure during CT_close!");
            debug!("{}", why);
            Status::ERR_HTSI.value()
        }
    };

    debug!("Returning {}", status);
    status
}
