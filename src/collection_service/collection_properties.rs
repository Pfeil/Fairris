use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CollectionProperties {
    #[serde(skip_serializing)]
    id: String,
    #[serde(skip_serializing)]
    date_created: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    ownership: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    license: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    model_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_access_restrictions: Option<bool>,
    #[serde(skip_serializing)]
    member_of: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    description_ontology: String,
}