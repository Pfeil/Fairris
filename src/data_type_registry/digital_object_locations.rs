use super::{HasProfileKey, Pid};
use std::fmt::Display;


pub struct Locations;

/// Associates profiles with their Display name (for the user interface).
impl Display for Locations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Locations")
    }
}

impl HasProfileKey for Locations {
    fn get_key() -> Pid {
        Pid("21.T11148/b8457812905b83046284".into())
    }
    fn get_key_name() -> &'static str {
        "digitalObjectLocation"
    }
}
