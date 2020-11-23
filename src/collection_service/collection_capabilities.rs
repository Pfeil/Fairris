use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CollectionCapabilities {
    #[serde(skip_serializing)]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_ordered: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    appends_to_end: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supports_roles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    membership_is_mutable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties_are_mutable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restricted_to_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_length: Option<i64>,
}