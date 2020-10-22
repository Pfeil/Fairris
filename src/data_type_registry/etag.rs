use super::{HasProfileKey, Pid};
use crate::service_communication::{PidRecord, pit_record::PidRecordEntry};

use serde_json as json;
use std::{ops::Deref, fmt::Display};

#[derive(Debug, Clone)]
pub struct Etag(pub String);

impl Default for Etag {
    fn default() -> Self {
        Etag(r#"{ "sha256sum": "sha256 c50624fd5ddd2b9652b72e2d2eabcb31a54b777718ab6fb7e44b582c20239a7c" }"#.into())
    }
}

impl Display for Etag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "etag")
    }
}

impl HasProfileKey for Etag {
    fn get_key() -> Pid {
        Pid("21.T11148/92e200311a56800b3e47".into())
    }
    
    fn get_key_name() -> &'static str {
        "etag"
    }

    fn write(&self, record: &mut PidRecord) {
        record.add_attribute(
            Self::get_key().deref().clone(),
            Self::get_key_name().into(),
            json::Value::String(self.0.clone()))
    }
}

impl From<&PidRecordEntry> for Etag {
    fn from(entry: &PidRecordEntry) -> Self {
        match &entry.value {
            json::Value::String(s) => Etag(s.clone()),
            _ => {
                log::error!("The given etag/checksum was not a string.");
                Etag::default()
            }
        }
    }
}

impl From<&PidRecord> for Etag {
    fn from(record: &PidRecord) -> Self {
        record
            .entries
            .get(&*Self::get_key())
            .and_then(|list| list.get(0).and_then(|entry| Some(Self::from(entry))))
            .unwrap_or_default()
    }
}
