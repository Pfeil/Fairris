#[macro_use]
mod macros;
mod date_time;
mod etag;
mod locations;
mod policy;
mod profile;
mod r#type;
mod version;

use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

pub use date_time::*;
pub use etag::*;
pub use locations::*;
pub use policy::*;
pub use profile::*;
pub use r#type::*;
pub use version::*;

use crate::service_communication::PidRecord;

pub trait HasProfileKey {
    /// Associates key-PID to a type.
    fn get_key() -> Pid;
    /// Associates key-name to a type.
    fn get_key_name() -> &'static str;
    /// Serializes the object into a record.
    fn write(&self, record: &mut PidRecord);
}

impl<T, P> HasProfileKey for Result<T, Option<P>>
where
    T: HasProfileKey + Clone,
    P: From<T> + Into<Pid> + Clone,
{
    fn get_key() -> Pid {
        T::get_key()
    }

    fn get_key_name() -> &'static str {
        T::get_key_name()
    }

    fn write(&self, record: &mut PidRecord) {
        let maybe_pid: Option<Pid> = self
            .as_ref()
            .map(|ok| {
                let pid: Option<Pid> = Some(P::from(ok.clone()).into());
                pid
            })
            .unwrap_or_else(|pid| pid.clone().map(|p| p.into()));
        if let Some(pid) = maybe_pid {
            record.add_attribute(
                Self::get_key().deref().clone(),
                Self::get_key_name().into(),
                serde_json::Value::String(pid.deref().clone()),
            );
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
