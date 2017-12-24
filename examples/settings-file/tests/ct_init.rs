extern crate libloading;
extern crate rand;
extern crate test_server;

use libloading::{Library, Symbol};
use std::fs;
use std::path::Path;
use std::str;
use test_server::futures::{Future, Stream};
use test_server::hyper;

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
    let server = test_server::serve(Some(String::from("127.0.0.1:65432")));
    server.reply().status(hyper::Ok).body("0");

    match Library::new(LIB_PATH) {
        Ok(lib) => {
            let init: Symbol<unsafe extern "system" fn(u16, u16) -> i8> =
                unsafe { lib.get(b"CT_init").unwrap() };

            let ctn = rand::random::<u16>();
            let pn = rand::random::<u16>();

            unsafe {
                assert_eq!(0, init(ctn, pn));
            }
        }
        _ => assert!(false, format!("loading library from {}", LIB_PATH)),
    }

    let (method, uri, _version, _headers, body) = server.request().unwrap().deconstruct();

    assert_eq!(hyper::Method::Post, method);
    assert_eq!("/yaml/ct_init/17/321", uri.path());
    assert!(body.concat2().wait().unwrap().is_empty());

    assert!(Path::new(LOG_FILE_PATH).exists());

    let metadata = fs::metadata(LOG_FILE_PATH).unwrap();
    assert!(metadata.len() > 0);
}
