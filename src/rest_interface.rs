/// This file contains structs that can be serialized into and deserialized from JSON.
/// They may contain methods to convert them into more usable structs.

use serde::{Serialize, Deserialize};
use ::std::collections::HashMap;
use crate::pidinfo::PidInfo;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PidRecordEntry {
    pub key: String,
    pub name: String,
    pub value: String,
}

impl PidRecordEntry {
    fn from(key: String, value: String) -> Self {
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
    fn get_attribute(&mut self, attribute: &str) -> String {
        self.entries.remove(attribute).and_then(|mut vec| {
            vec.pop().and_then(|entry| {
                Some(entry.value)
            })
        }).unwrap_or("Not found.".into())
    }
}

impl From<PidInfo> for PidRecord {
    fn from(info: PidInfo) -> Self {
        let mut map = HashMap::new();
        map.insert("description".into(), vec![PidRecordEntry::from("description".into(), info.description)]);
        map.insert("status".into(), vec![PidRecordEntry::from("status".into(), info.status)]);
        PidRecord {
            pid: info.pid,
            entries: map,
        }
    }
}

impl From<PidRecord> for PidInfo {
    fn from(mut record: PidRecord) -> Self {
        PidInfo {
            description: record.get_attribute("description"),
            status: record.get_attribute("status"),
            pid: record.pid,
        }
    }
}