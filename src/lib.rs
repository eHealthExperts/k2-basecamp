extern crate libc;

use libc::{uint8_t, size_t};

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn CT_init(ctn: u16, pn: u16) -> i8 {

    1
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern fn CT_data(ctn: u16, dad: *mut uint8_t, sad: *mut uint8_t, lenc: size_t, command: *const uint8_t, lenr: *mut size_t, response: *mut uint8_t) -> i8 {
    unsafe {

        1
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn CT_close(ctn: u16) -> i8 {

    1
}
