use self::super::{ERR_HOST, ERR_HTSI, ERR_INVALID, MAP, OK};
use self::super::super::http;

use std::u16;

pub fn close(ctn: u16) -> i8 {
    let checked_ctn = match ctn {
        ctn if ctn >= u16::MIN && ctn <= u16::MAX => ctn,
        _ => {
            error!("ctn is not an u16!. Returning {}", ERR_INVALID);
            return ERR_INVALID;
        }
    };

    // Do we know this CTN?
    if !MAP.lock().unwrap().contains_key(&checked_ctn) {
        error!("Card terminal has not been opened. Returning {}",
               ERR_INVALID);
        return ERR_INVALID;
    }

    let path = get_request_path(checked_ctn);
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
        http::HttpStatus::Ok => handle_ok_status(response_body, checked_ctn),
        _ => {
            error!("Response not OK! Returning {}", ERR_HOST);
            ERR_HOST
        }
    }
}

fn get_request_path(ctn: u16) -> String {
    let pn = MAP.lock().unwrap().get(&ctn).unwrap().clone();

    format!("ct_close/{}/{}", ctn, pn)
}

fn handle_ok_status(body: String, ctn: u16) -> i8 {
    let status = body.parse::<i8>().unwrap();
    if status == OK {
        // Remove CTN
        MAP.lock().unwrap().remove(&ctn);
        debug!("Card terminal has been closed.");
    }

    debug!("Returning {}", status);
    status
}
