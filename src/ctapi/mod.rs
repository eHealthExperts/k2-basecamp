use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    pub static ref MAP: Mutex<HashMap<u16, u16>> = Mutex::new(HashMap::new());
}

pub static OK: i8 = 0;
pub static ERR_INVALID: i8 = -1;
pub static ERR_HOST: i8 = -127;
pub static ERR_HTSI: i8 = -128;

pub mod init;
pub mod data;
pub mod close;
