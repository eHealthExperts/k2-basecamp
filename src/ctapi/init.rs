use self::super::{ERR_HOST, ERR_HTSI, ERR_INVALID, MAP, OK};
use self::super::super::http;

use std::u16;

pub fn init(ctn: u16, pn: u16) -> i8 {
    let checked_ctn = match ctn {
        ctn if ctn > u16::MIN && ctn < u16::MAX => ctn,
        _ => {
            error!("ctn is not an u16!. Returning {}", ERR_INVALID);
            return ERR_INVALID;
        }
    };

    let checked_pn = match pn {
        pn if pn > u16::MIN && pn < u16::MAX => pn,
        _ => {
            error!("pn is not an u16!. Returning {}", ERR_INVALID);
            return ERR_INVALID;
        }
    };

    // Do we know this CTN?
    if MAP.lock().unwrap().contains_key(&checked_ctn) {
        error!("Card terminal has already been opened. Returning {}",
               ERR_INVALID);
        return ERR_INVALID;
    }

    // Build the request URL
    let path = get_request_path(checked_ctn, checked_pn);

    // Perform the request
    let http_response = match http::simple_post(path) {
        Ok(http_response) => http_response,
        Err(error) => {
            debug!("Error: {:?}", error);
            error!("Request failed! Returning {}", ERR_HTSI);
            return ERR_HTSI;
        }
    };

    let (http_status, response_body) = http::extract_response(http_response);
    match http_status {
        http::HttpStatus::Ok => handle_ok_status(response_body, checked_ctn, checked_pn),
        _ => {
            error!("Response not OK! Returning {}", ERR_HOST);
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

fn handle_ok_status(body: String, ctn: u16, pn: u16) -> i8 {
    let status = body.parse::<i8>().unwrap();
    if status == OK {
        // Store CTN
        MAP.lock().unwrap().insert(ctn, pn);
        debug!("Card terminal opened.");
    }

    debug!("Returning {}", status);
    status
}
