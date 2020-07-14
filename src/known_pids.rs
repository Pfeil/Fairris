use std::ops::Deref;

use super::{PidInfo};

pub struct KnownPids {
    known_pids: Vec<PidInfo>,
}

impl KnownPids {
    pub fn find(&self, pid: &str) -> &PidInfo {
        // TODO handle the unwrap better.
        self.known_pids.iter().find(|info| info.pid == pid).unwrap_or_else(|| {
            log::error!("Finding the PID ({}) did not work out.", pid);
            panic!()
        })
    }
}

impl Default for KnownPids {
    fn default() -> Self {
        log::info!("Will insert some dummy Pid objects for testing.");
        let mut known_pids = vec![
            PidInfo::default(),
            PidInfo::default(),
            PidInfo::default(),
            PidInfo::default(),
        ];
        known_pids.iter_mut().enumerate().for_each(|(num, info)| {
            info.pid = format!("{}_{}", info.pid, num);
        });
        KnownPids {
            known_pids,
        }
    }
}

impl Deref for KnownPids {
    type Target = Vec<PidInfo>;

    fn deref(&self) -> &Self::Target {
        &self.known_pids
    }
}