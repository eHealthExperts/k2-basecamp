extern crate hyper;

pub use self::super::{ERR_HOST, ERR_HTSI, ERR_INVALID, MAP, OK};
pub use self::super::super::{http, logging};

use hyper::client::response::Response;
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
    let path = get_request_path(ctn, pn);

    // Perform the request
    let response = match http::simple_post(path) {
        Ok(response) => response,
        Err(error) => {
            debug!("Error: {:?}", error);
            error!("CT_data: Request failed! Returning {}", ERR_HTSI);
            return ERR_HTSI;
        }
    };

    debug!("{:?}", response); // TODO enrich output

    match response.status {
        StatusCode::Ok => handle_ok_status(response, ctn, pn),
        _ => {
            error!("CT_init: Response not OK! Returning {}", ERR_HOST);
            ERR_HOST
        }
    }
}

fn get_request_path(ctn: u16, pn: u16) -> String {
    let mut path = String::from("ct_init");
    path.push_str("/");
    path.push_str(&ctn.to_string());
    path.push_str("/");
    path.push_str(&pn.to_string());

    path
}

fn handle_ok_status(mut response: Response, ctn: u16, pn: u16) -> i8 {
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
