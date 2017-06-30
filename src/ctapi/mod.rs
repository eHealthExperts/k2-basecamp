pub use self::close::close;
pub use self::data::data;
pub use self::init::init;
pub use self::status::StatusCode;

use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    pub static ref MAP: Mutex<HashMap<u16, u16>> = Mutex::new(HashMap::new());
}

mod close;
mod data;
mod init;
mod status;