use std::{convert::TryFrom, ops::{Deref, DerefMut}};

use chrono::{DateTime, ParseError, Utc};
use serde_json as json;

use crate::service_communication::{pit_record::PidRecordEntry, PidRecord};

use super::{HasProfileKey, Pid};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DateCreated(pub DateTimeHandle);

impl HasProfileKey for DateCreated {
    fn get_key() -> Pid {
        Pid("21.T11148/aafd5fb4c7222e2d950a".into())
    }

    fn get_key_name() -> &'static str {
        "dateCreated"
    }

    fn write(&self, record: &mut PidRecord) {
        record.add_attribute(
            Self::get_key().deref().clone(),
            Self::get_key_name().into(),
            json::Value::String(self.0.to_rfc3339()))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DateModified(pub DateTimeHandle);

impl HasProfileKey for DateModified {
    fn get_key() -> Pid {
        Pid("21.T11148/397d831aa3a9d18eb52c".into())
    }

    fn get_key_name() -> &'static str {
        "dateModified"
    }

    fn write(&self, record: &mut PidRecord) {
        record.add_attribute(
            Self::get_key().deref().clone(),
            Self::get_key_name().into(),
            json::Value::String(self.0.to_rfc3339()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateTimeHandle(DateTime<Utc>);

impl Default for DateTimeHandle {
    fn default() -> Self {
        // TODO this default is for testing only. It should be empty and the component should handle the case of an invalid url properly.
        DateTimeHandle(Utc::now())
    }
}

impl TryFrom<&String> for DateTimeHandle {
    type Error = ParseError;

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        let parsed_date = DateTime::parse_from_rfc3339(s.as_str());
        parsed_date.map(|date| DateTimeHandle(date.with_timezone(&Utc)))
    }
}

impl DateTimeHandle {
    pub fn to_string(&self) -> String {
        self.to_rfc3339()
    }
}

macro_rules! impl_date_wrapper_from_entry {
    ( $date_wrapper:tt ) => {
        impl From<&PidRecordEntry> for $date_wrapper {
            fn from(entry: &PidRecordEntry) -> Self {
                match &entry.value {
                    json::Value::String(s) => Self(
                        // TODO this should return a Result (TryFrom) and give it up to the GUI
                        DateTimeHandle::try_from(s).unwrap_or_default()
                    ),
                    _ => {
                        log::error!("The given date was not a string.");
                        Self::default()
                    }
                }
            }
        }
    };
}

// Impls for From<&PidRecord> and From<&PidRecordEntry>
impl_date_wrapper_from_entry!(DateCreated);
impl_date_wrapper_from_entry!(DateModified);

impl_from_record_single_entry!(DateCreated);
impl_from_record_single_entry!(DateModified);

// Makes inner value accessible and mutable with * operator.
newtype_deref!(DateCreated, DateTimeHandle);
newtype_deref!(DateModified, DateTimeHandle);
newtype_deref!(DateTimeHandle, DateTime<Utc>);