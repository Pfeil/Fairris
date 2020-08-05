use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use super::PidInfo;
use crate::service_communication::primitive_types::Pid;

#[derive(Default)]
pub struct KnownPids {
    known_pids: HashMap<Pid, PidInfo>,
}

impl KnownPids {
    pub fn find(&self, pid: &str) -> &PidInfo {
        // TODO handle the unwrap better.
        self.known_pids.get(pid.into()).unwrap()
    }
}

impl KnownPids {
    pub fn with_dummy() -> Self {
        log::info!("Will insert some dummy Pid objects for testing.");
        let dummy_prefix = "kitdm/dummy";
        let pids = vec![
            PidInfo::default(),
            PidInfo::default(),
            PidInfo::default(),
            PidInfo::default(),
        ];
        let mut known_pids = HashMap::new();
        pids.into_iter().enumerate().for_each(|(num, mut info)| {
            let pid = info.pid_mut();
            pid.push_str(format!("{}_{}", dummy_prefix, num).as_str());
            known_pids.insert(pid.clone(), info);
        });
        KnownPids { known_pids }
    }
}

impl Deref for KnownPids {
    type Target = HashMap<Pid, PidInfo>;

    fn deref(&self) -> &Self::Target {
        &self.known_pids
    }
}

impl DerefMut for KnownPids {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.known_pids
    }
}