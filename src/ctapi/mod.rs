pub mod close;
pub mod data;
pub mod init;
pub mod status;

use antidote::RwLock;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref MAP: RwLock<HashMap<u16, u16>> = RwLock::new(HashMap::new());
}
