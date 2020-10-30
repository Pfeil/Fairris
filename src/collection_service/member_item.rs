use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    #[serde(skip_serializing)]
    id: String,
    location: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    description: String,
    datatype: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ontology: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    mappings: Option<Mapping>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Mapping {
    #[serde(skip_serializing_if = "String::is_empty")]
    role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<u64>,
    #[serde(skip_serializing)]
    date_added: String,
    #[serde(skip_serializing)]
    date_updated: String,
}
