extern crate serde_json;

use self::super::{ERR_HOST, ERR_HTSI, ERR_INVALID, MAP, OK};
use self::super::super::http;

use base64::{encode, decode};
use std::slice;
use std::u16;
use std::u8;

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

    debug!("ctn: {}", ctn);
    if ctn < u16::MIN && ctn > u16::MAX {
        error!("ctn is not an u16. Returning {}", ERR_INVALID);
        return ERR_INVALID;
    }

    let safe_dad: &mut u8 = unsafe { &mut *dad };
    debug!("dad: {}", safe_dad);
    if safe_dad < &mut u8::MIN && safe_dad > &mut u8::MAX {
        error!("dad is not an u8. Returning {}", ERR_INVALID);
        return ERR_INVALID;
    }

    let safe_sad: &mut u8 = unsafe { &mut *sad };
    debug!("sad: {}", safe_sad);
    if safe_sad < &mut u8::MIN && safe_sad > &mut u8::MAX {
        error!("sad is not an u8. Returning {}", ERR_INVALID);
        return ERR_INVALID;
    }

    debug!("lenc: {}", lenc);
    if lenc < u16::MIN as usize && lenc > u16::MAX as usize {
        error!("lenc is not an u16. Returning {}", ERR_INVALID);
        return ERR_INVALID;
    }

    let safe_command = unsafe { slice::from_raw_parts(command, lenc) };
    debug!("command: {:?}", safe_command);

    let safe_lenr: &mut usize = unsafe { &mut *lenr };
    debug!("lenr: {}", safe_lenr);
    sanitize_lenr(&mut *safe_lenr);

    let safe_response = unsafe { slice::from_raw_parts_mut(response, *safe_lenr) };
    debug!("response with {} slices formed", safe_response.len());

    if !MAP.lock().unwrap().contains_key(&ctn) {
        error!("Card terminal has not been opened. Returning {}",
               ERR_INVALID);
        return ERR_INVALID;
    }

    let request_data = RequestData {
        dad: *safe_dad,
        sad: *safe_sad,
        lenc,
        command: encode(safe_command),
        lenr: *safe_lenr,
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
            let data: ResponseData = match serde_json::from_str(&response_body) {
                Ok(response) => response,
                Err(error) => {
                    error!("Failed to parse response data. {}", error);
                    error!("Returning {}", ERR_HOST);
                    return ERR_HOST;
                }
            };

            if data.responseCode == OK {
                *safe_dad = data.dad;
                *safe_sad = data.sad;
                *safe_lenr = data.lenr;

                let decoded = decode(&data.response).unwrap();
                debug!("Decoded response field {:?}", decoded);

                for (place, element) in safe_response.iter_mut().zip(decoded.iter()) {
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
    let min_usize = u16::MIN as usize;
    if *lenr < min_usize || *lenr > max_usize {
        debug!("... sanitize lenr to {}", u16::MAX);
        *lenr = max_usize;
    }
}
