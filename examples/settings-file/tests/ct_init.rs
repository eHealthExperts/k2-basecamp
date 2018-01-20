#[macro_use]
extern crate const_cstr;
extern crate dlopen;
extern crate rand;
extern crate test_server;

use dlopen::raw::Library;
use std::fs;
use std::path::Path;
use std::str;
use test_server::http::{Method, StatusCode};

#[cfg(target_os = "windows")]
const LOG_FILE_PATH: &str = "ctehxk2.log";
#[cfg(not(target_os = "windows"))]
const LOG_FILE_PATH: &str = "libctehxk2.log";

#[cfg(target_os = "windows")]
const LIB_PATH: &str = "../../target/debug/ctehxk2.dll";
#[cfg(target_os = "linux")]
const LIB_PATH: &str = "../../target/debug/libctehxk2.so";
#[cfg(target_os = "macos")]
const LIB_PATH: &str = "../../target/debug/libctehxk2.dylib";

#[test]
fn with_config_file() {
    let lib = Library::open(LIB_PATH).expect("Could not open library");

    let init: unsafe extern "system" fn(u16, u16) -> i8 =
        unsafe { lib.symbol_cstr(const_cstr!("CT_init").as_cstr()) }.unwrap();

    let server = test_server::serve(Some("127.0.0.1:65432"));
    server.reply().status(StatusCode::OK).body("0");

    let ctn = rand::random::<u16>();
    let pn = rand::random::<u16>();

    assert_eq!(0, unsafe { init(ctn, pn) });

    let (parts, body) = server.request().unwrap().into_parts();
    assert_eq!(body, String::from(""));
    assert_eq!(parts.method, Method::POST);
    assert_eq!(parts.uri, "/yaml/ct_init/17/321");

    assert!(Path::new(LOG_FILE_PATH).exists());

    let metadata = fs::metadata(LOG_FILE_PATH).unwrap();

    assert!(metadata.len() > 0);
}
