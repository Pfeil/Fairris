//! This module contains a serializable record definition
//! that is used by the PIT service.
use super::datatypes::Pid;
use ::std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json as json;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Clone)]
pub struct PidRecordEntry {
    pub key: String,
    pub name: String,
    pub value: json::Value,
}

impl PidRecordEntry {
    fn from(key: String, value: json::Value) -> Self {
        PidRecordEntry {
            key: key.clone(),
            name: key,
            value,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PidRecord {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub pid: Pid,
    pub entries: HashMap<String, Vec<PidRecordEntry>>,
}

impl PidRecord {
    fn extract_attribute(&mut self, attribute: &str) -> json::Value {
        self.entries
            .remove(attribute)
            .and_then(|mut vec| vec.pop().and_then(|entry| Some(entry.value)))
            .unwrap_or(json::Value::String(format!(
                "Value to attribute \"{}\" not found.",
                attribute
            )))
    }

    pub fn add_attribute(&mut self, id: String, name: String, value: json::Value) {
        let entry = PidRecordEntry {
            key: id.clone(),
            name,
            value,
        };
        let values = self.entries.entry(id.clone()).or_insert(Vec::new());
        values.push(entry);
    }

    pub fn describe(&self) -> String {
        String::from("TODO implement descripton of records")
    }
}

impl PartialEq for PidRecord {
    fn eq(&self, other: &Self) -> bool {
        match (self.pid.is_empty(), other.pid.is_empty()) {
            (true, true) => self.pid == other.pid,
            (false, false) => {
                self.entries
                    .keys()
                    .chain(other.entries.keys())
                    .filter(|&key| self.entries.get(key) != other.entries.get(key))
                    .count()
                    == 0
            }
            _ => false,
        }
    }
}

impl Eq for PidRecord {}
