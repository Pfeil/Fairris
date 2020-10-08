use std::{convert::TryFrom, fmt::Display};

use enum_iterator::IntoEnumIterator;
use serde_json as json;

use crate::service_communication::{pit_record::PidRecordEntry, PidRecord};

use super::{HasProfileKey, Pid};

/// A list of profiles that are known to this client.\
/// Note: If a profile is added, there is only a need to adjust two things:
/// - the Pid::from::<Profile>() implementation to associate a PID, and
/// - the Display::fmt() implementation to associate a human readable name for the UI.
///
/// Fortunately, the compiler will remember you to do so.
#[derive(Clone, Copy, Debug, IntoEnumIterator, PartialEq)]
pub enum Profile {
    Testbed,
}

/// Associates profiles with their PID.
impl From<Profile> for Pid {
    fn from(p: Profile) -> Self {
        match p {
            Profile::Testbed => Pid(r#"21.T11148/61fd3446879407065218"#.into()),
        }
    }
}

/// Associates profiles with their Display name (for the user interface).
impl Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Profile::Testbed => write!(f, "Testbed"),
        }
    }
}

impl HasProfileKey for Profile {
    fn get_key() -> Pid {
        Pid("21.T11148/076759916209e5d62bd5".into())
    }
    fn get_key_name() -> &'static str {
        "kernelInformationProfile"
    }
}

try_from_pid!(Profile, Pid);

/// Err(None) -> no PID found
/// Err(Some(pid)) -> unknown PID found
impl TryFrom<&PidRecordEntry> for Profile {
    type Error = Option<Pid>;

    fn try_from(entry: &PidRecordEntry) -> Result<Self, Self::Error> {
        if entry.key != *Profile::get_key() {
            return Err(None);
        }
        if let json::Value::String(s) = &entry.value {
            let pid: Pid = Pid(s.clone());
            Profile::try_from(&pid).map_err(|e| Some(e))
        } else {
            Err(None)
        }
    }
}

/// Err(None) -> no PID found
/// Err(Some(pid)) -> unknown PID found
impl TryFrom<&mut PidRecord> for Profile {
    type Error = Option<Pid>;

    fn try_from(record: &mut PidRecord) -> Result<Self, Self::Error> {
        record
            .entries
            .get(&*Profile::get_key())
            .map(|list| list.get(0))
            .flatten()
            .ok_or(Self::Error::None)
            .and_then(|entry| Profile::try_from(entry))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_pid_conversion() {
        let wrong_pid = Pid("wrong/pid".into());
        assert_eq!(Profile::try_from(&wrong_pid), Err(wrong_pid));
        let testbed_pid = Pid::from(Profile::Testbed);
        assert_eq!(Profile::try_from(&testbed_pid), Ok(Profile::Testbed));
    }
}
