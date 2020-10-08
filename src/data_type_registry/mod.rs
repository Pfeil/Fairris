#[macro_use]
mod macros;
mod profile;
mod r#type;
mod locations;
mod version;

use std::{fmt::Display, ops::{Deref, DerefMut}};

pub use profile::*;
pub use r#type::*;
pub use locations::*;
pub use version::*;

pub trait HasProfileKey {
    /// Associates key-PID to a type.
    fn get_key() -> Pid;
    /// Associates key-name to a type.
    fn get_key_name() -> &'static str;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pid(pub String);

impl Deref for Pid {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Pid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Pid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}