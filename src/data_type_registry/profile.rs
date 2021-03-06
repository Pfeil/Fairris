use std::{ops::Deref, convert::TryFrom, fmt::Display};

use strum::IntoEnumIterator;
use serde_json as json;

use crate::service_communication::{pit_record::PidRecordEntry, PidRecord};

use super::{HasProfileKey, Pid};

/// A list of profiles that are known to this client.\
/// Note: If a profile is added, there is only a need to adjust two things:
/// - the Pid::from::<Profile>() implementation to associate a PID, and
/// - the Display::fmt() implementation to associate a human readable name for the UI.
///
/// Fortunately, the compiler will remember you to do so.
#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq)]
pub enum Profile {
    Testbed,
}

pub type MaybeProfile = Result<Profile, Option<Pid>>;

/// Associates profiles with their PID.
impl From<&Profile> for Pid {
    fn from(p: &Profile) -> Self {
        match p {
            Profile::Testbed => Pid("21.T11148/301c6f04763a16f0f72a".into()),
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
    
    fn write(&self, record: &mut PidRecord) {
        record.add_attribute(
            Self::get_key().deref().clone(),
            Self::get_key_name().into(),
            Pid::from(self).deref().to_owned().into());
    }
}

impl Default for Profile {
    fn default() -> Self {
        Profile::Testbed
    }
}

try_from_all!(Profile, Pid);

impl From<Profile> for Pid {
    fn from(p: Profile) -> Self {
        Pid::from(&p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_pid_conversion() {
        let wrong_pid = Pid("wrong/pid".into());
        assert_eq!(Profile::try_from(&wrong_pid), Err(wrong_pid));
        let testbed_pid = Pid::from(&Profile::Testbed);
        assert_eq!(Profile::try_from(&testbed_pid), Ok(Profile::Testbed));
    }
}
