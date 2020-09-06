pub mod close;
pub mod data;
pub mod init;
pub mod status;

use antidote::RwLock;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub(crate) static MAP: Lazy<RwLock<HashMap<u16, u16>>> = Lazy::new(|| RwLock::new(HashMap::new()));
