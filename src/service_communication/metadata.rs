use super::primitive_types as primitive;
use super::*;
use primitive::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use serde_json as json;

#[derive(Default)]
pub struct MetadataDocumentPid(primitive::Pid);
newtype_deref!(MetadataDocumentPid, primitive::Pid);

impl RecordEntry for MetadataDocumentPid {
    fn write(&self, record: &mut PidRecord) {
        let id = "21.T11148/e0efd6b4c8e71c6d077b".into(); // TODO asjust to actual type as soon as it exists
        let name = "metadataObject".into();
        let value = json::Value::String(self.0.clone());
        record.add_attribute(id, name, value);
    }
}

#[derive(Default)]
pub struct MetadataDocument {
    pub scheme: primitive::PidProxy,
    pub pid: primitive::PidProxy,
    pub r#type: primitive::PidProxy,
}

impl RecordEntry for MetadataDocument {
    fn write(&self, record: &mut PidRecord) {
        let id = "21.T11148/e0efd6b4c8e71c6d077b".into();
        let name = "metadataDocument".into();
        let value = json::json!({
            "metadataScheme": self.scheme,
            "@id": self.pid,
            "@type": self.r#type
        });
        let value = json::Value::String(value.to_string());
        record.add_attribute(id, name, value);
    }
}

#[derive(Default, Debug)]
pub struct MetadataObjectReference {
    pub context: MetadataContext,
    pub resource: ResourceReference,
}

impl RecordEntry for MetadataObjectReference {
    fn write(&self, record: &mut PidRecord) {
        Self::write_new(record, self.context.clone(), self.resource.clone());
    }
}

impl MetadataObjectReference {
    pub fn write_new(
        record: &mut PidRecord,
        context: MetadataContext,
        reference: ResourceReference,
    ) {
        let id = "21.T11148/134c84df7eca7bced374".into();
        let name = "metadataObject".into();
        let value = json::json!({
            "relation": context,
            "resource": reference.as_json()
        });
        let value = json::Value::String(value.to_string());
        record.add_attribute(id, name, value);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MetadataContext {
    #[serde(rename = "annotating")]
    Annotating,
    #[serde(rename = "ontology")]
    Ontology,
}

impl Default for MetadataContext {
    fn default() -> Self {
        Self::Annotating
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ResourceReference {
    Handle(primitive::Pid),
    Url(primitive::URL),
}

impl Default for ResourceReference {
    fn default() -> Self {
        Self::Url(String::new())
    }
}

impl ResourceReference {
    pub fn as_json(&self) -> json::Value {
        match self {
            ResourceReference::Url(url) => json::Value::String(url.clone()),
            ResourceReference::Handle(handle) => json::Value::String(handle.clone()),
        }
    }
}
