use serde_json as json;

use crate::service_communication::{pit_record::PidRecordEntry, PidRecord};

use super::{HasProfileKey, Pid};
use std::{ops::Deref, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq)]
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

    fn write(&self, record: &mut PidRecord) {
        record.add_attribute(
            Self::get_key().deref().clone(),
            Self::get_key_name().into(),
            json::Value::String(self.0.clone()),
        )
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

impl Default for Version {
    fn default() -> Self {
        Version("1.0.0".into())
    }
}