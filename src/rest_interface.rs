//! This file contains structs that can be serialized into and deserialized from JSON.
//! They may contain methods to convert them into more usable structs.
use crate::pidinfo::PidInfo;
use ::std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json as json;

#[derive(Serialize, Deserialize, Debug, Default)]
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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PidRecord {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub pid: String,
    pub entries: HashMap<String, Vec<PidRecordEntry>>,
}

impl PidRecord {
    fn extract_attribute(&mut self, attribute: &str) -> json::Value {
        self.entries
            .remove(attribute)
            .and_then(|mut vec| vec.pop().and_then(|entry| Some(entry.value)))
            .unwrap_or(json::Value::String(format!("Value to attribute \"{}\" not found.", attribute)))
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
}

impl From<PidInfo> for PidRecord {
    fn from(info: PidInfo) -> Self {
        let mut map = HashMap::new();
        map.insert(
            "description".into(),
            vec![PidRecordEntry::from("description".into(), json::Value::String(info.description))],
        );
        map.insert(
            "status".into(),
            vec![PidRecordEntry::from("status".into(), json::Value::String(info.status))],
        );
        PidRecord {
            pid: info.pid,
            entries: map,
        }
    }
}

impl From<PidRecord> for PidInfo {
    fn from(mut record: PidRecord) -> Self {
        PidInfo {
            description: record.extract_attribute("description").as_str().unwrap().into(),
            status: record.extract_attribute("status").as_str().unwrap().into(),
            pid: record.pid,
        }
    }
}
