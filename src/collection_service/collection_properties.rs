use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CollectionProperties {
    #[serde(skip_serializing)]
    id: Option<u64>,
    #[serde(skip_serializing)]
    date_created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ownership: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_access_restrictions: Option<bool>,
    #[serde(skip_serializing)]
    member_of: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description_ontology: Option<String>,
}