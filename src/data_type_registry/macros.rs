#[macro_use]



/// Error contains a copy of the given pid for later handling.
/// This makes sense as it is not unlikely that the PID is valid, but
/// can not be handled by the client. Err is in this case a marker for
/// i.e. the UI that this PID can be displayed, but can not be operated on.
macro_rules! try_from_pid {
    ( $given_type:tt, $error_type:tt ) => {
        impl TryFrom<&Pid> for $given_type
        {
            type Error = $error_type;
        
            fn try_from(pid: &Pid) -> Result<Self, Self::Error> {
                $given_type::iter() // iterate over every profile
                    .map(|p: $given_type| {
                        // assiociate them with their PID
                        (Pid::from(&p), p)
                    })
                    .find(|(p_pid, _)| pid == p_pid) // find the pid
                    .map(|(_, p)| p) // get profile
                    .ok_or(pid.clone()) // return pid on error
            }
        }
    }
}

macro_rules! try_from_entry {
    ( $given_type:tt, $error_type:tt ) => {
        /// Err(None) -> no PID found
        /// Err(Some(pid)) -> unknown PID found
        impl TryFrom<&PidRecordEntry> for $given_type {
            type Error = Option<$error_type>;

            fn try_from(entry: &PidRecordEntry) -> Result<Self, Self::Error> {
                if entry.key != *$given_type::get_key() {
                    return Err(None);
                }
                if let json::Value::String(s) = &entry.value {
                    let pid: Pid = Pid(s.clone());
                    $given_type::try_from(&pid).map_err(|e| Some(e))
                } else {
                    Err(None)
                }
            }
        }
    }
}

macro_rules! try_from_record {
    ( $given_type:tt, $error_type:tt ) => {
        /// Err(None) -> no PID found
        /// Err(Some(pid)) -> unknown PID found
        impl TryFrom<&PidRecord> for $given_type {
            type Error = Option<$error_type>;

            fn try_from(record: &PidRecord) -> Result<Self, Self::Error> {
                record
                    .entries
                    .get(&*$given_type::get_key())
                    .map(|list| list.get(0))
                    .flatten()
                    .ok_or(Self::Error::None)
                    .and_then(|entry| $given_type::try_from(entry))
            }
        }
    }
}

macro_rules! try_from_all {
    ( $given_type:tt, $error_type:tt ) => {
        
        try_from_pid!($given_type, $error_type);
        try_from_entry!($given_type, $error_type);
        try_from_record!($given_type, $error_type);
        
    }
}

macro_rules! impl_from_record_single_entry {
    ( $given_type:tt ) => {
        
        impl From<&PidRecord> for $given_type {
            fn from(record: &PidRecord) -> Self {
                record
                    .entries
                    .get(&*Self::get_key())
                    .and_then(|list| list.get(0).and_then(|entry| Some(Self::from(entry))))
                    .unwrap_or_default()
            }
        }
        
    }
}

macro_rules! newtype_deref {
    ($name:ty, $target:ty) => {
        impl Deref for $name {
            type Target = $target;
        
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        
        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }       
    };
}