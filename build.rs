extern crate cheddar;
#[cfg(windows)]
extern crate winres;

use std::env;
use std::path::MAIN_SEPARATOR;

fn build_header_file() {
    let target = format!("{}{}{}{}{}",
        "target".to_owned(),
        MAIN_SEPARATOR.to_string(), 
        env::var("PROFILE").unwrap(),
        MAIN_SEPARATOR.to_string(),
        "ctehxk2.h"
    );

    cheddar::Cheddar::new()
        .expect("could not read manifest")
        .run_build(target);
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
