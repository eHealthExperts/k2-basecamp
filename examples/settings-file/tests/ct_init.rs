#[macro_use]
extern crate const_cstr;
extern crate dlopen;
extern crate rand;
extern crate test_server;

use dlopen::raw::Library;
use std::fs;
use std::path::Path;
use std::str;
use test_server::HttpResponse;

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

    let server = test_server::new(65432, |_| HttpResponse::Ok().body("0"));

    let ctn = rand::random::<u16>();
    let pn = rand::random::<u16>();

    assert_eq!(0, unsafe { init(ctn, pn) });

    let request = server.requests.next().unwrap();

    //assert_eq!(body, "");
    assert_eq!(request.method, "POST");
    assert_eq!(request.path, "/yaml/ct_init/17/321");

    assert!(Path::new(LOG_FILE_PATH).exists());

    let metadata = fs::metadata(LOG_FILE_PATH).unwrap();

    assert!(metadata.len() > 0);
}
