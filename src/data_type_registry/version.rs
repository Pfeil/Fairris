use super::{HasProfileKey, Pid};
use std::fmt::Display;

pub struct Version;

/// Associates profiles with their Display name (for the user interface).
impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Version")
    }
}

impl HasProfileKey for Version {
    fn get_key() -> Pid {
        Pid("21.T11148/c692273deb2772da307f".into())
    }
    fn get_key_name() -> &'static str {
        "version"
    }
}
