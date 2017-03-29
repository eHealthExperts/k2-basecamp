extern crate hyper;
extern crate libc;
extern crate rustc_serialize;

#[macro_use]
extern crate lazy_static;

use hyper::Client;
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};
use libc::{uint8_t, size_t};
use rustc_serialize::{Encodable, json};
use std::collections::HashMap;
use std::env::var;
use std::io::Read;
use std::slice;
use std::sync::Mutex;

lazy_static! {
    static ref MAP: Mutex<HashMap<u16, u16>> = Mutex::new(HashMap::new());
}

static OK: i8 = 0;
static ERR_INVALID: i8 = -1;
static ERR_HOST: i8 = 127;

#[derive(RustcDecodable, RustcEncodable)]
struct Empty();

macro_rules! post_request {
    ($path:expr) => (post_request($path, &Empty{}));
    ($path:expr, $data:expr) => (post_request($path, $data));
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn CT_init(ctn: u16, pn: u16) -> i8 {
    // Do we know this CTN?
    if MAP.lock().unwrap().contains_key(&ctn) {
        return ERR_INVALID
    }

    // Build the request URL
    let endpoint = "ct_init".to_string();
    let path = endpoint + "/" + &ctn.to_string() + "/" + &pn.to_string();

    // Perform the request
    let response = post_request!(&path);

    match response {
        Ok(v) => {
            // Cast server response
            let response = v.parse::<i8>().unwrap();
            if response == OK {
                // Store CTN
                MAP.lock().unwrap().insert(ctn, pn);
            }

            response
        },
        Err(_) => ERR_HOST
    }
}

#[derive(RustcDecodable, RustcEncodable)]
struct Data {
    ctn: u16,
    dad: u8,
    sad: u8,
    lenc: usize,
    command: Vec<u8>,
    lenr: usize
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn CT_data(ctn: u16, dad: u8, sad: u8, lenc: size_t, command: *const uint8_t, lenr: size_t, response: *const uint8_t) -> i8 {
    if !MAP.lock().unwrap().contains_key(&ctn) {
        return ERR_INVALID
    }

    let commands = unsafe {
        assert!(!command.is_null());
        slice::from_raw_parts(command, lenc as usize)
    };

    let responses = unsafe {
        assert!(!response.is_null());
        slice::from_raw_parts(response, lenr as usize)
    };

    let data = Data {
        ctn: ctn,
        dad: dad,
        sad: sad,
        lenc: lenc,
        command: commands.to_vec(),
        lenr: lenr
    };

    let pn = MAP.lock().unwrap();
    let pn = pn.get(&ctn).unwrap();
    let endpoint = "ct_data".to_string();
    let path = endpoint + "/" + &ctn.to_string() + "/" + &pn.to_string();

    let post_response = post_request(&path, &data);

    // fill responses
    // adjust lenr

    match post_response {
        Ok(v) => v.parse::<i8>().unwrap(),
        Err(_) => ERR_HOST
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn CT_close(ctn: u16) -> i8 {
    // Do we know this CTN?
    if !MAP.lock().unwrap().contains_key(&ctn) {
        return ERR_INVALID
    }

    // Build the request URL
    let pn = MAP.lock().unwrap().get(&ctn).unwrap().clone();
    let endpoint = "ct_close".to_string();
    let path = endpoint + "/" + &ctn.to_string() + "/" + &pn.to_string();

    // Perform the request
    let response = post_request!(&path);

    match response {
        Ok(v) => {
            // Cast server response
            let response = v.parse::<i8>().unwrap();
            if response == OK {
                // Remove CTN
                MAP.lock().unwrap().remove(&ctn);
            }

            response
        },
        Err(_) => ERR_HOST
    }
}

fn env_or_default(var_name: &str, default: &str) -> String {
    match var(var_name) {
        Ok(s) => s,
        Err(_) => default.into(),
    }
}

fn post_request<T>(path: &str, payload: &T) -> hyper::Result<String>
    where T: Encodable
{
    // untested
    let base_url = env_or_default("K2_BASE_URL", "http://localhost:8080/k2/ctapi/");
    let url = base_url + path;

    let client = Client::new();

    let mut headers = Headers::new();
    headers.set(
        ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![]))
    );

    let body = json::encode(payload).unwrap();

    let mut response = client.post(&url)
        .headers(headers)
        .body(&body[..])
        .send()?;

    let mut buf = String::new();
    response.read_to_string(&mut buf)?;

    Ok(buf)
}
