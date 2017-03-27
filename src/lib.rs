extern crate hyper;
extern crate url;

#[macro_use]
extern crate lazy_static;

use hyper::Client;
use std::env::var;
use std::io::Read;
use std::sync::Mutex;
use url::form_urlencoded;

lazy_static! {
    static ref REGISTER: Mutex<Vec<u16>> = Mutex::new(vec![]);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn CT_init(ctn: u16, pn: u16) -> i8 {
    if REGISTER.lock().unwrap().iter().any(| &x| x == ctn) {
        return -1
    } else {
        REGISTER.lock().unwrap().push(ctn);
    }

    let ctn_string = ctn.to_string();
    let pn_string = pn.to_string();
    let path = "ct_init/".to_string() + &ctn_string + "/" + &pn_string;

    let response = post_query(&path, vec![]);

    match response {
        Ok(v) => v.parse::<i8>().unwrap(),
        Err(_) => -1
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

    1
}

fn env_or_default(var_name: &str, default: &str) -> String {
    match var(var_name) {
        Ok(s) => s,
        Err(_) => default.into(),
    }
}

type Query<'a> = Vec<(&'a str, &'a str)>;

fn post_query(path: &str, query: Query) -> hyper::Result<String> {
    let base_url = env_or_default("BASE_URL", "http://localhost:8080/k2/ctapi/");
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
