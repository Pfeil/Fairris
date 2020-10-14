use serde_json as json;

use crate::service_communication::{pit_record::PidRecordEntry, PidRecord};

use super::{HasProfileKey, Pid};
use std::fmt::Display;

#[derive(Debug, Clone, Default)]
pub struct Version(pub String);

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

impl From<&PidRecordEntry> for Version {
    fn from(entry: &PidRecordEntry) -> Self {
        match &entry.value {
            json::Value::String(s) => Version(s.clone()),
            json::Value::Number(n) => Version(n.to_string()),
            _ => {
                log::error!("The given version was neither a number nor a string.");
                Version::default()
            }
        }
    }
}

impl From<&PidRecord> for Version {
    fn from(record: &PidRecord) -> Self {
        record
            .entries
            .get(&*Self::get_key())
            .and_then(|list| list.get(0).and_then(|entry| Some(Self::from(entry))))
            .unwrap_or_default()
    }
}
