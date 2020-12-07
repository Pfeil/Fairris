use serde_json as json;

use crate::service_communication::{PidRecord, pit_record::PidRecordEntry};

use super::{HasProfileKey, Pid};
use std::{ops::Deref, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy(pub String);

/// Associates profiles with their Display name (for the user interface).
impl Display for Policy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Policy")
    }
}

impl HasProfileKey for Policy {
    fn get_key() -> Pid {
        Pid("21.T11148/8074aed799118ac263ad".into())
    }
    
    fn get_key_name() -> &'static str {
        "digitalObjectPolicy"
    }

    fn write(&self, record: &mut PidRecord) {
        record.add_attribute(
            Self::get_key().deref().clone(),
            Policy::get_key_name().into(), 
            json::Value::String(self.0.clone()))
    }
}

impl Default for Policy {
    fn default() -> Self {
        Policy("your/favorite/license/etc".into())
    }
}

impl From<&PidRecordEntry> for Policy {
    fn from(entry: &PidRecordEntry) -> Self {
        match &entry.value {
            json::Value::String(s) => Policy(s.clone()),
            _ => {
                log::error!("The given policy was not a string.");
                Policy::default()
            },
        }
    }
}

impl From<&PidRecord> for Policy {
    fn from(record: &PidRecord) -> Self {
        record.entries.get(&*Self::get_key()).and_then(|list| {
            list.get(0)
                .and_then(|entry| Some(Self::from(entry)))
        })
        .unwrap_or_default()
    }
}
