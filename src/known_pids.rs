use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use super::{Model, PidInfo};
use crate::service_communication::primitive_types::Pid;

use rand::prelude::*;
use yew::prelude::*;

#[derive(Default)]
pub struct KnownPids {
    known_pids: HashMap<Pid, PidInfo>,
}

impl KnownPids {
    pub fn find(&self, pid: &str) -> Option<&PidInfo> {
        self.known_pids.get(pid.into())
    }

    pub fn add_unregistered(&mut self, model_link: ComponentLink<Model>) -> Pid {
        let mut object = PidInfo::default(model_link);
        let pid: String;
        loop {
            let random_number = rand::thread_rng().gen::<u16>();
            let maybe_pid = format!("unregistered-{}", random_number);
            if !self.known_pids.contains_key(&maybe_pid) {
                pid = maybe_pid;
                let obj_pid = object.pid_mut();
                *obj_pid = pid.clone();
                break;
            }
        }
        self.known_pids.insert(pid.clone(), object);
        pid
    }
}

impl KnownPids {
    pub fn with_dummy(model_link: ComponentLink<Model>) -> Self {
        log::info!("Will insert some dummy Pid objects for testing.");
        let dummy_prefix = "kitdm/dummy";
        let pids = vec![
            PidInfo::default(model_link.clone()),
            PidInfo::default(model_link.clone()),
            PidInfo::default(model_link.clone()),
            PidInfo::default(model_link.clone()),
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
