extern crate hyper;

pub use self::super::{ERR_HOST, ERR_HTSI, ERR_INVALID, MAP, OK};
pub use self::super::super::{http, logging};

use hyper::status::StatusCode;
use std::io::Read;

pub fn close(ctn: u16) -> i8 {
    logging::init();

    debug!("CT_close: Called (ctn {})", ctn);

    // Do we know this CTN?
    if !MAP.lock().unwrap().contains_key(&ctn) {
        debug!("CT_close: Card terminal has not been opened. Returning {}",
               ERR_INVALID);
        return ERR_INVALID;
    }

    // Build the request URL
    let pn = MAP.lock().unwrap().get(&ctn).unwrap().clone();
    let mut path = String::from("ct_close");
    path.push_str("/");
    path.push_str(&ctn.to_string());
    path.push_str("/");
    path.push_str(&pn.to_string());

    // Perform the request
    let mut response = match http::simple_post(path) {
        Ok(response) => response,
        Err(error) => {
            debug!("Error: {:?}", error);
            error!("CT_close: Request failed! Returning {}", ERR_HTSI);
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
                // Remove CTN
                MAP.lock().unwrap().remove(&ctn);
                debug!("CT_close: Card terminal has been closed.");
            }

            debug!("CT_close: Returning {}", status);
            status
        }
        _ => {
            error!("CT_close: Response not OK! Returning {}", ERR_HOST);
            ERR_HOST
        }
    }
}
