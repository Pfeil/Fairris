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
                $given_type::into_enum_iter() // iterate over every profile
                    .map(|p: $given_type| {
                        // assiociate them with their PID
                        (Pid::from(p), p)
                    })
                    .find(|(p_pid, _)| pid == p_pid) // find the pid
                    .map(|(_, p)| p) // get profile
                    .ok_or(pid.clone()) // return pid on error
            }
        }
    }
}