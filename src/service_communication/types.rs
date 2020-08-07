use serde_json as json;
use serde::{Serialize, Deserialize};

use super::pit_record::PidRecord;
use super::primitive_types as primitive;
use super::RecordEntry;
use std::ops::{Deref, DerefMut};

impl RecordEntry for primitive::Profile {
    fn write(&self, record: &mut PidRecord) {
        use primitive::Profile;
        let id = "21.T11148/076759916209e5d62bd5".into();
        let name = "profilePid".into();
        let profile = match self {
            Profile::Testbed4infSimplified | Profile::Testbed4infRaw => {
                "21.T11148/61fd3446879407065218".into()
            }
        };
        record.add_attribute(id, name, profile);
    }
}

pub enum DataType {
    Tiff,
    Png,
    Pid(primitive::Pid),
}

impl Default for DataType {
    fn default() -> Self {
        Self::Tiff
    }
}

impl From<i32> for DataType {
    fn from(index: i32) -> Self {
        match index {
            0 => Self::Tiff,
            1 => Self::Png,
            2 => Self::Pid(String::new()),
            unknown => {
                log::error!("Profile index '{}' not implemented.", unknown);
                Self::default()
            }
        }
    }
}

impl DataType {
    pub fn as_json(&self) -> json::Value {
        match self {
            DataType::Tiff => "tiff".into(),
            DataType::Png => "png".into(),
            DataType::Pid(pid) => json::Value::String(pid.into()),
        }
    }
}

impl RecordEntry for DataType {
    fn write(&self, record: &mut PidRecord) {
        let id = "21.T11148/c83481d4bf467110e7c9".into();
        let name = "digitalObjectType".into();
        let image_type = self.as_json();
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
        let id = "21.T11148/aafd5fb4c7222e2d950a".into();
        let name = "dateCreatedRfc3339".into();
        let value = json::Value::String(format!("{}", self.0.to_rfc3339()));
        record.add_attribute(id, name, value);
    }
}

#[derive(Default)]
pub struct DateModified(primitive::DateTimeHandle);

impl RecordEntry for DateModified {
    fn write(&self, record: &mut PidRecord) {
        let id = "21.T11148/397d831aa3a9d18eb52c".into();
        let name = "dateModifiedRfc3339".into();
        let value = json::Value::String(format!("{}", self.0.to_rfc3339()));
        record.add_attribute(id, name, value);
    }
}

#[derive(Default)]
pub struct Version(String);

newtype_deref!(Version, String);

impl RecordEntry for Version {
    fn write(&self, record: &mut PidRecord) {
        if !self.is_empty() {
            let id = "21.T11148/c692273deb2772da307f".into();
            let name = "version".into();
            let value = json::Value::String(self.0.clone());
            record.add_attribute(id, name, value);
        }
    }
}

#[derive(Default)]
pub struct LicenseString(String);

impl RecordEntry for LicenseString {
    fn write(&self, record: &mut PidRecord) {
        let id = "21.T11148/dc54ae4b6807f5887fda".into();
        let name = "license".into();
        let value = json::Value::String(self.0.clone());
        record.add_attribute(id, name, value);
    }
}

newtype_deref!(LicenseString, String);

#[derive(Serialize, Deserialize)]
pub struct Contributor {
    #[serde(rename = "21.T11148/3626040cadcac1571685")]
    pub identifier: primitive::Pid,
    #[serde(rename = "21.T11148/31cf58fed6ddd1b96102")]
    pub role: String,
}

#[derive(Serialize, Deserialize)]
pub struct Contributors(Vec<Contributor>);

newtype_deref!(Contributors, Vec<Contributor>);

impl RecordEntry for Contributors {
    fn write(&self, record: &mut PidRecord) {
        let id = "21.T11148/6534eca5d640dc878d87".into();
        let name = "contributors".into();
        let value = json::Value::String(json::to_string(self).unwrap());
        record.add_attribute(id, name, value);
    }
}

impl Default for Contributors {
    fn default() -> Self {
        let author = Contributor {
            identifier: "kitdm/author123".into(),
            role: "author".into(),
        };
        let institute = Contributor {
            identifier: "kitdm/institute123".into(),
            role: "institute".into(),
        };
        Contributors(vec![author, institute])
    }
}
