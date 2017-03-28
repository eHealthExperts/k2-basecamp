extern crate hyper;
extern crate url;

#[macro_use]
extern crate lazy_static;

use hyper::Client;
use std::collections::HashMap;
use std::env::var;
use std::io::Read;
use std::sync::Mutex;
use url::form_urlencoded;

lazy_static! {
    static ref MAP: Mutex<HashMap<u16, u16>> = Mutex::new(HashMap::new());
}

static OK: i8 = 0;
static ERR_INVALID: i8 = -1;

macro_rules! post_query {
    ($path:expr) => (post_query($path, Vec::new()));
    ($path:expr, $query:expr) => (post_query($path, $query));
}

type Query<'a> = Vec<(&'a str, &'a str)>;

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
    let response = post_query!(&path);

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
        Err(_) => ERR_INVALID
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn CT_data(ctn: u16, dad: u8, sad: u8, lenc: u16, command: u8, lenr: u16, response: u8) -> i8 {

    1
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
    let response = post_query!(&path);

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
        Err(_) => ERR_INVALID
    }
}

fn env_or_default(var_name: &str, default: &str) -> String {
    match var(var_name) {
        Ok(s) => s,
        Err(_) => default.into(),
    }
}

fn post_query(path: &str, query: Query) -> hyper::Result<String> {
    // untested
    let base_url = env_or_default("K2_BASE_URL", "http://localhost:8080/k2/ctapi/");
    let url = base_url + path;

    let client = Client::new();
    let body = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(query.iter())
        .finish();
    let mut response = client.post(&url)
        .body(&body[..])
        .send()?;
    let mut buf = String::new();
    response.read_to_string(&mut buf)?;

    Ok(buf)
}
