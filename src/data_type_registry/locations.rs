use serde_json as json;

use crate::service_communication::{pit_record::PidRecordEntry, PidRecord};

use super::{HasProfileKey, Pid};
use std::{ops::Deref, fmt::Display};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Locations(pub Vec<String>);

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

    fn write(&self, record: &mut PidRecord) {
        for location in self.0.iter() {
            record.add_attribute(
                Self::get_key().deref().clone(),
                Self::get_key_name().into(),
                json::Value::String(location.to_owned()),
            )
        }
    }
}

impl From<&PidRecordEntry> for Locations {
    fn from(entry: &PidRecordEntry) -> Self {
        if entry.key != *Self::get_key() {
            return Self::default();
        }
        if let json::Value::String(s) = &entry.value {
            Self(vec![s.clone()])
        } else {
            log::error!("Value was not a string, which was not expected.");
            Self::default()
        }
    }
}

impl From<&PidRecord> for Locations {
    fn from(record: &PidRecord) -> Self {
        record
            .entries
            .get(&*Self::get_key())
            .and_then(|list| {
                let locations: Vec<String> = list
                    .iter()
                    .filter_map(|entry| {
                        if let json::Value::String(s) = &entry.value {
                            Some(s.clone())
                        } else {
                            None
                        }
                    })
                    .collect();
                Some(Locations(locations))
            })
            .unwrap_or_default()
    }
}
