pub use self::close::close;
pub use self::data::data;
pub use self::init::init;

use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    pub static ref MAP: Mutex<HashMap<u16, u16>> = Mutex::new(HashMap::new());
}

pub static OK: i8 = 0;
pub static ERR_INVALID: i8 = -1;
pub static ERR_HOST: i8 = -127;
pub static ERR_HTSI: i8 = -128;

mod init;
mod data;
mod close;
