use self::super::{ERR_HTSI, ERR_INVALID, MAP, OK};
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
        error!(
            "Card terminal has not been opened. Returning {}",
            ERR_INVALID
        );
        return ERR_INVALID;
    }

    let path = get_request_path(checked_ctn);
    let response = http::request().post(&path, None).response();

    if response.status() != 200 {
        error!("Request failed! Returning {}", ERR_HTSI);
        return ERR_HTSI;
    }

    handle_ok_status(response.body(), checked_ctn)
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
