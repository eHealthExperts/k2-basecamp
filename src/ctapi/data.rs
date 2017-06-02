extern crate serde_json;

use self::super::{ERR_HOST, ERR_HTSI, ERR_INVALID, MAP, OK};
use self::super::super::http;

use base64::{encode, decode};
use std::slice;
use std::u16;

#[derive(Serialize)]
struct RequestData {
    dad: u8,
    sad: u8,
    lenc: usize,
    command: String,
    lenr: usize,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct ResponseData {
    dad: u8,
    sad: u8,
    lenr: usize,
    response: String,
    responseCode: i8,
}

pub fn data(ctn: u16,
            dad: *mut u8,
            sad: *mut u8,
            lenc: usize,
            command: *const u8,
            lenr: *mut usize,
            response: *mut u8)
            -> i8 {

    let _dad: &mut u8 = unsafe { &mut *dad };
    debug!("dad: {}", _dad);

    let _sad: &mut u8 = unsafe { &mut *sad };
    debug!("sad: {}", _sad);
    debug!("lenc: {}", lenc);

    let _command = unsafe { slice::from_raw_parts(command, lenc as usize) };
    debug!("command: {:?}", _command);

    let _lenr: &mut usize = unsafe { &mut *lenr };
    debug!("lenr: {}", _lenr);

    sanitize_lenr(&mut *_lenr);

    let _response = unsafe { slice::from_raw_parts_mut(response, *_lenr) };
    debug!("response with {} slices formed", _response.len());

    if !MAP.lock().unwrap().contains_key(&ctn) {
        debug!("Card terminal has not been opened. Returning {}",
               ERR_INVALID);
        return ERR_INVALID;
    }

    let request_data = RequestData {
        dad: *_dad,
        sad: *_sad,
        lenc: lenc,
        command: encode(_command),
        lenr: *_lenr,
    };

    let path = get_request_path(ctn);
    let http_response = match http::post(path, &request_data) {
        Ok(http_response) => http_response,
        Err(error) => {
            debug!("Error: {:?}", error);
            error!("Request failed! Returning {}", ERR_HTSI);
            return ERR_HTSI;
        }
    };

    let (http_status, response_body) = http::extract_response(http_response);
    match http_status {
        http::HttpStatus::Ok => {
            let data: ResponseData = serde_json::from_str(&response_body).unwrap();

            if data.responseCode == OK {
                *_dad = data.dad;
                *_sad = data.sad;
                *_lenr = data.lenr;

                let decoded = decode(&data.response).unwrap();
                debug!("Decoded response field {:?}", decoded);

                for (place, element) in _response.iter_mut().zip(decoded.iter()) {
                    *place = *element;
                }
            }
            debug!("Returning {}", data.responseCode);
            return data.responseCode;
        }
        _ => {
            error!("Response not OK! Returning {}", ERR_HOST);
            ERR_HOST
        }
    }
}

fn get_request_path(ctn: u16) -> String {
    let pn = MAP.lock().unwrap();
    let pn = pn.get(&ctn).unwrap();

    let mut path = String::from("ct_data");
    path.push_str("/");
    path.push_str(&ctn.to_string());
    path.push_str("/");
    path.push_str(&pn.to_string());

    path
}

fn sanitize_lenr(lenr: &mut usize) {
    let max_usize = u16::MAX as usize;
    if *lenr > max_usize {
        debug!("... sanitize lenr to {}", u16::MAX);
        *lenr = max_usize;
    }
}
