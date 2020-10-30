use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use super::{collection_capabilities::CollectionCapabilities, collection_properties::CollectionProperties};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    #[serde(skip_serializing)]
    id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<CollectionProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capabilities: Option<CollectionCapabilities>,
}

pub struct Collections(Vec<Collection>);

impl Deref for Collections {
    type Target = Vec<Collection>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Collections {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}