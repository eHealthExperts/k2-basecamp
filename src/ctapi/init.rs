extern crate hyper;

pub use self::super::{ERR_HOST, ERR_HTSI, ERR_INVALID, MAP, OK};
pub use self::super::super::{http, logging};

use hyper::status::StatusCode;
use std::io::Read;

pub fn init(ctn: u16, pn: u16) -> i8 {
    logging::init();

    debug!("CT_init: Called (ctn {}, pn {})", ctn, pn);

    // Do we know this CTN?
    if MAP.lock().unwrap().contains_key(&ctn) {
        debug!("CT_init: Card terminal has already been opened. Returning {}",
               ERR_INVALID);
        return ERR_INVALID;
    }

    // Build the request URL
    let endpoint = "ct_init".to_string();
    let path = endpoint + "/" + &ctn.to_string() + "/" + &pn.to_string();

    // Perform the request
    let mut response = match http::simple_post(&path) {
        Ok(response) => response,
        Err(error) => {
            debug!("Error: {:?}", error);
            error!("CT_data: Request failed! Returning {}", ERR_HTSI);
            return ERR_HTSI;
        }
    };

    debug!("{:?}", response); // TODO enrich output

    match response.status {
        StatusCode::Ok => {
            // Cast server response
            let mut body = String::new();
            response.read_to_string(&mut body).unwrap();

            let status = body.parse::<i8>().unwrap();
            if status == OK {
                // Store CTN
                MAP.lock().unwrap().insert(ctn, pn);
                debug!("CT_init: Card terminal opened.");
            }

            debug!("CT_init: Returning {}", status);
            status
        }
        _ => {
            error!("CT_init: Response not OK! Returning {}", ERR_HOST);
            ERR_HOST
        }
    }
}
