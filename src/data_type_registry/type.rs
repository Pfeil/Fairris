use std::{convert::TryFrom, fmt::Display};

use serde_json as json;

use super::{HasProfileKey, Pid};
use enum_iterator::IntoEnumIterator;
use crate::service_communication::{pit_record::PidRecordEntry, PidRecord};

#[derive(Clone, Copy, Debug, IntoEnumIterator, PartialEq)]
pub enum DigitalObjectType {
    // a collection (api) item with all related objects to replicate the results.
    Publication,
    // a written piece that explains a publication (human readable).
    Paper,
    // an object describing an algorithm. Not self runnable.
    // i.e. a script for bash/python, pseudocode, ...
    Algorithm,
    // an application may interpret algorithms or is compatible to certain data formats.
    Application,
    // a collection (api) item collecting ManuscriptPages.
    Manuscript,
    // a manuscript page of any format with any kind of additional metadata.
    ManuscriptPage,
    // one or multiple annotations belonging all to the same unit.
    // might be a collection (api) item.
    Annotations,
}

pub type MaybeDOType = Result<DigitalObjectType, Option<Pid>>;

/// Associates types with their PID.
/// FIXME PIDs are not yet correct as they do not yet exist.
impl From<DigitalObjectType> for Pid {
    fn from(p: DigitalObjectType) -> Self {
        match p {
            DigitalObjectType::Publication => Pid(r#"21.T11148/Publication"#.into()),
            DigitalObjectType::Paper => Pid(r#"21.T11148/Paper"#.into()),
            DigitalObjectType::Algorithm => Pid(r#"21.T11148/Algorithm"#.into()),
            DigitalObjectType::Application => Pid(r#"21.T11148/Application"#.into()),
            DigitalObjectType::Manuscript => Pid(r#"21.T11148/Manuscript"#.into()),
            DigitalObjectType::ManuscriptPage => Pid(r#"21.T11148/ManuscriptPage"#.into()),
            DigitalObjectType::Annotations => Pid(r#"21.T11148/Annotations"#.into()),
        }
    }
}

/// Associates profiles with their Display name (for the user interface).
impl Display for DigitalObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DigitalObjectType::Publication => {write!(f, "Publication")}
            DigitalObjectType::Paper => {write!(f, "Paper")}
            DigitalObjectType::Algorithm => {write!(f, "Algorithm")}
            DigitalObjectType::Application => {write!(f, "Application")}
            DigitalObjectType::Manuscript => {write!(f, "Manuscript")}
            DigitalObjectType::ManuscriptPage => {write!(f, "ManuscriptPage")}
            DigitalObjectType::Annotations => {write!(f, "Annotations")}
        }
    }
}

impl HasProfileKey for DigitalObjectType {
    fn get_key() -> Pid {
        Pid("21.T11148/c83481d4bf467110e7c9".into())
    }

    fn get_key_name() -> &'static str {
        "digitalObjectType"
    }
}

impl Default for DigitalObjectType {
    fn default() -> Self {
        DigitalObjectType::ManuscriptPage
    }
}

try_from_all!(DigitalObjectType, Pid);