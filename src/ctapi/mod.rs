mod close;
mod data;
mod init;
mod status;

pub use self::close::close;
pub use self::data::data;
pub use self::init::init;
pub use self::status::Status;
use antidote::Mutex;
use std::collections::HashMap;

lazy_static! {
    pub static ref MAP: Mutex<HashMap<u16, u16>> = Mutex::new(HashMap::new());
}
