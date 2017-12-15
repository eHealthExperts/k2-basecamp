extern crate libloading as lib;
extern crate rand;
extern crate rouille;

#[macro_use]
mod macros;

use std::fs;
use std::path::Path;

#[cfg(target_os = "windows")]
const LOG_FILE_PATH: &str = "ctehxk2.log";
#[cfg(not(target_os = "windows"))]
const LOG_FILE_PATH: &str = "libctehxk2.log";

#[cfg(target_os = "windows")]
const LIB_PATH: &str = "../../target/debug/ctehxk2.dll";
#[cfg(target_os = "linux")]
const LIB_PATH: &str = "../../target/debug/ctehxk2.so";
#[cfg(target_os = "macos")]
const LIB_PATH: &str = "../../target/debug/libctehxk2.dylib";

#[test]
fn base_url_from_config_file() {
    let shutdown = test_server!(("127.0.0.1:5432", request: &Request) {
        if request.url() == "/yaml/ct_init/17/321" {
            ::rouille::Response::text("0")
        } else {
            ::rouille::Response::empty_404()
        }
    });

    match lib::Library::new(LIB_PATH) {
        Ok(lib) => unsafe {
            let init: lib::Symbol<unsafe extern "C" fn(u16, u16) -> i8> =
                lib.get(b"CT_init").unwrap();

            let ctn = rand::random::<u16>();
            let pn = rand::random::<u16>();

            assert_eq!(0, init(ctn, pn));
        },
        _ => assert!(false, format!("loading library from {}", LIB_PATH)),
    }

    // kill server thread
    let _ = shutdown.send(());

    assert!(Path::new(LOG_FILE_PATH).exists());

    let metadata = fs::metadata(LOG_FILE_PATH).unwrap();
    assert!(metadata.len() > 0);
}
