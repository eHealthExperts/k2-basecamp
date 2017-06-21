use self::super::{ERR_HTSI, ERR_INVALID, MAP, OK};
use self::super::super::http;

use std::u16;

pub fn init(ctn: u16, pn: u16) -> i8 {
    let checked_ctn = match ctn {
        ctn if ctn >= u16::MIN && ctn <= u16::MAX => ctn,
        _ => {
            error!("ctn is not an u16!. Returning {}", ERR_INVALID);
            return ERR_INVALID;
        }
    };

    let checked_pn = match pn {
        pn if pn >= u16::MIN && pn <= u16::MAX => pn,
        _ => {
            error!("pn is not an u16!. Returning {}", ERR_INVALID);
            return ERR_INVALID;
        }
    };

    // Do we know this CTN?
    if MAP.lock().unwrap().contains_key(&checked_ctn) {
        error!(
            "Card terminal has already been opened. Returning {}",
            ERR_INVALID
        );
        return ERR_INVALID;
    }

    // Perform the request
    let path = format!("ct_init/{}/{}", checked_ctn, checked_pn);
    let response = http::request().post(&path, None).response();

    if response.status() != 200 {
        error!("Request failed! Returning {}", ERR_HTSI);
        return ERR_HTSI;
    }

    handle_ok_status(response.body(), checked_ctn, checked_pn)
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
