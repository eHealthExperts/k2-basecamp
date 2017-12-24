pub mod close;
pub mod data;
pub mod init;
pub mod status;

use antidote::Mutex;
use std::collections::HashMap;

lazy_static! {
    pub static ref MAP: Mutex<HashMap<u16, u16>> = Mutex::new(HashMap::new());
}
