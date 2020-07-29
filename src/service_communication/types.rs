use serde_json as json;

use super::pit_record::PidRecord;
use super::primitive_types as primitive;
use super::RecordEntry;

impl RecordEntry for primitive::Profile {
    fn write(&self, record: &mut PidRecord) {
        use primitive::Profile;
        let id = "21.T11148/076759916209e5d62bd5".into();
        let name = "KernelInformationProfile".into();
        let profile = match self {
            Profile::RecommendedKernelProfile => "21.T11148/0c5636e4d82b88f86132".into(),
            Profile::HmcKernelProfile => "21.T11148/b9b76f887845e32d29f7".into(),
        };
        record.add_attribute(id, name, profile);
    }
}

impl RecordEntry for primitive::DataType {
    fn write(&self, record: &mut PidRecord) {
        use primitive::DataType;
        let id = "21.T11148/1c699a5d1b4ad3ba4956".into();
        let name = "digitalObjectType".into();
        let image_type = match self {
            DataType::Tiff => "21.T11148/2834eac0159f584bcf05".into(),
            DataType::Png => "21.T11148/2834eac0159f584bcf05".into(),
        };
        // TODO how to set the image type png/tiff? Currently the PID just says "iana image"
        record.add_attribute(id, name, image_type);
    }
}

impl RecordEntry for primitive::Policy {
    fn write(&self, record: &mut PidRecord) {
        use primitive::{License, Lifecycle};
        let id: String = "21.T11148/8074aed799118ac263ad".into();
        let name: String = "digitalObjectPolicy".into();
        // TODO choose pid to policy by given lifecycle and license.
        let policy_pid = match (&self.lifecycle, &self.license) {
            (Lifecycle::Static, License::MIT) => "policy/default".into(),
            (Lifecycle::Static, License::Apache) => "policy/default".into(),
            (Lifecycle::Static, License::CcBy) => "policy/default".into(),
            (Lifecycle::RegularUpdates, License::MIT) => "policy/default".into(),
            (Lifecycle::RegularUpdates, License::Apache) => "policy/default".into(),
            (Lifecycle::RegularUpdates, License::CcBy) => "policy/default".into(),
            (Lifecycle::IrregularUpdates, License::MIT) => "policy/default".into(),
            (Lifecycle::IrregularUpdates, License::Apache) => "policy/default".into(),
            (Lifecycle::IrregularUpdates, License::CcBy) => "policy/default".into(),
        };
        record.add_attribute(id, name, policy_pid);
    }
}

impl RecordEntry for primitive::Checksum {
    fn write(&self, record: &mut PidRecord) {
        let id = "21.T11148/92e200311a56800b3e47".into();
        let name = "etag".into();
        let value =
            json::json!({ "sha256sum": json::Value::String(format!("sha256: {}", self.value)) });
        let value = json::Value::String(value.to_string()); // PIT service only parses json strings as values
        record.add_attribute(id, name, value);
    }
}

impl RecordEntry for primitive::ObjectLocation {
    fn write(&self, record: &mut PidRecord) {
        let id = "21.T11148/b8457812905b83046284".into();
        let name = "digitalObjectLocation".into();
        let value = json::Value::String((*self).clone());
        record.add_attribute(id, name, value);
    }
}

// Below this line are proper newtypes that wrap primitive types properly so they can be assigned different pids.
// TODO all of the above should be below there. Wrap everything in newtypes (or type in cases like version).

#[derive(Default)]
pub struct DateCreated(primitive::DateTimeHandle);

impl RecordEntry for DateCreated {
    fn write(&self, record: &mut PidRecord) {
        let id = "21.T11148/29f92bd203dd3eaa5a1f".into();
        let name = "dateCreated".into();
        let value = json::Value::String( format!("{}", self.0.format("%F %T")) );
        record.add_attribute(id, name, value);
    }
}

#[derive(Default)]
pub struct DateModified(primitive::DateTimeHandle);

impl RecordEntry for DateModified {
    fn write(&self, record: &mut PidRecord) {
        let id = "21.T11148/397d831aa3a9d18eb52c".into();
        let name = "dateModified".into();
        let value = json::Value::String( format!("{}", self.0.to_rfc3339()) );
        record.add_attribute(id, name, value);
    }
}

pub type Version = String;

impl RecordEntry for Version {
    fn write(&self, record: &mut PidRecord) {
        if !self.is_empty() {
            let id = "21.T11148/c692273deb2772da307f".into();
            let name = "version".into();
            let value = json::Value::String( self.clone() );
            record.add_attribute(id, name, value);
        }
    }
}