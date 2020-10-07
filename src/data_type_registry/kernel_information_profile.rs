use std::{convert::TryFrom, fmt::Display};

use enum_iterator::IntoEnumIterator;
use serde_json as json;

use crate::service_communication::{pit_record::PidRecordEntry, PidRecord};

use super::{HasProfileKey, Pid};

#[derive(Clone, Copy, Debug, IntoEnumIterator, PartialEq)]
pub enum Profile {
    Testbed,
}

impl HasProfileKey for Profile {
    fn get_key() -> Pid {
        Pid("21.T11148/076759916209e5d62bd5".into())
    }

    fn get_key_name() -> &'static str {
        "kernelInformationProfile"
    }
}

/// Associates profiles with their PID.
impl From<Profile> for Pid {
    fn from(p: Profile) -> Self {
        match p {
            Profile::Testbed => Pid(r#"21.T11148/61fd3446879407065218"#.into()),
            // IMPORTANT: DO NOT DO A CATCH-ALL CASE HERE!
        }
    }
}

impl Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

/// Error contains a copy of the given pid for later handling.
/// This makes sense as it is not unlikely that the PID is valid, but
/// can not be handled by the client. Err is in this case a marker for
/// i.e. the UI that this PID can be displayed, but can not be operated on.
impl TryFrom<&Pid> for Profile {
    type Error = Pid;

    fn try_from(pid: &Pid) -> Result<Self, Self::Error> {
        Profile::into_enum_iter() // iterate over every profile
            .map(|p: Profile| {
                // assiociate them with their PID
                (Pid::from(p), p)
            })
            .find(|(p_pid, _)| pid == p_pid) // find the pid
            .map(|(_, p)| p) // get profile
            .ok_or(pid.clone()) // return pid on error
    }
}

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
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn profile_pid_conversion() {
        let wrong_pid = Pid("wrong/pid".into());
        assert_eq!(Profile::try_from(&wrong_pid), Err(wrong_pid));
        let testbed_pid = Pid::from(Profile::Testbed);
        assert_eq!(Profile::try_from(&testbed_pid), Ok(Profile::Testbed));
    }

    #[test]
    fn enum_iter_test() {
        #[derive(IntoEnumIterator, Debug, PartialEq)]
        enum Test {
            A, B,
        }

        let _ = Test::into_enum_iter()
            .map(|_x: Test| {
                println!("Hello")
            })
            .collect::<Vec<_>>();
    }
}
