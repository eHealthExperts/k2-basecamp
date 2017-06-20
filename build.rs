extern crate cheddar;
#[cfg(windows)]
extern crate winres;

use std::env;
use std::path::MAIN_SEPARATOR;

fn build_header_file() {
    let profile = env::var("PROFILE").unwrap();

    cheddar::Cheddar::new()
        .expect("could not read manifest")
        .run_build(
            "target".to_owned() + &MAIN_SEPARATOR.to_string() + &profile +
                &MAIN_SEPARATOR.to_string() + "ctehxk2.h",
        );
}

#[cfg(windows)]
fn main() {
    build_header_file();

    if cfg!(target_os = "windows") {
        let res = winres::WindowsResource::new();
        res.compile().unwrap();
    }
}

#[cfg(not(windows))]
fn main() {
    build_header_file();
}
