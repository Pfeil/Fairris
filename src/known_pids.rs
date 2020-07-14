use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use super::PidInfo;

// TODO make type Pid = String in here and for pidinfo etc

pub struct KnownPids {
    known_pids: HashMap<String, PidInfo>,
}

impl KnownPids {
    pub fn find(&self, pid: &str) -> &PidInfo {
        // TODO handle the unwrap better.
        self.known_pids.get(pid.into()).unwrap()
    }
}

impl Default for KnownPids {
    fn default() -> Self {
        log::info!("Will insert some dummy Pid objects for testing.");
        let mut pids = vec![
            PidInfo::default(),
            PidInfo::default(),
            PidInfo::default(),
            PidInfo::default(),
        ];
        let mut known_pids = HashMap::new();
        pids.into_iter().enumerate().for_each(|(num, mut info)| {
            info.pid = format!("{}_{}", info.pid, num);
            known_pids.insert(info.pid.clone(), info);
        });
        KnownPids { known_pids }
    }
}

impl Deref for KnownPids {
    type Target = HashMap<String, PidInfo>;

    fn deref(&self) -> &Self::Target {
        &self.known_pids
    }
}

impl DerefMut for KnownPids {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.known_pids
    }
}