use self::super::{ERR_HTSI, ERR_INVALID, MAP, OK};
use self::super::super::http;

pub fn close(ctn: u16) -> i8 {
    // Do we know this CTN?
    if !MAP.lock().unwrap().contains_key(&ctn) {
        error!(
            "Card terminal has not been opened. Returning {}",
            ERR_INVALID
        );
        return ERR_INVALID;
    }

    let pn = MAP.lock().unwrap().get(&ctn).unwrap().clone();
    let path = format!("ct_close/{}/{}", ctn, pn);
    let response = http::request().post(&path, None).response();
    if response.status() != 200 {
        error!("Request failed! Returning {}", ERR_HTSI);
        return ERR_HTSI;
    }

    handle_ok_status(response.body(), ctn)
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
