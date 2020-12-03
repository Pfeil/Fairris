use std::collections::{HashMap, HashSet};

use crate::{Model, data_type_registry::Pid, pidinfo::PidInfo};

use rand::prelude::*;
use yew::{prelude::*, worker::{Agent, AgentLink, Context, HandlerId}};

pub struct PidManager {
    link: AgentLink<PidManager>,
    subscribers: HashSet<HandlerId>,

    known_pids: HashMap<Pid, PidInfo>,
    selected: Option<Pid>,
}

#[derive(Debug)]
pub enum Incoming {
    GetAllPidInformation,
    AddUnregisteredItem,
    AddPidInfo(PidInfo),
    RemovePidInfo(Pid),
    Replace(Pid, PidInfo),  // remove object with Pid and register PidInfo (contains it's own pid).
}

#[derive(Debug, Clone)]
pub enum Outgoing {
    AllPidInformation(HashMap<Pid, PidInfo>),
}

impl Agent for PidManager {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Incoming;
    type Output = Outgoing;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: Default::default(),
            known_pids: Default::default(),
            selected: None,
        }
    }

    fn update(&mut self, msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        log::debug!("PidManager message: {:?}", msg);
        let (pids_changed, selection_changed): (bool, bool) = match msg {
            Incoming::GetAllPidInformation => (true, false),
            Incoming::AddUnregisteredItem => {
                self.add_unregistered();
                (true, false)
            }
            Incoming::AddPidInfo(pidinfo) => {
                self.add(pidinfo);
                (true, false)
            }
            Incoming::RemovePidInfo(pid) => {
                self.remove(pid);
                // TODO it would be possible to select something else (or nothing) in case the deleted is selected.
                (true, false)
            }
            Incoming::Replace(pid, info) => {
                self.replace(pid, info);
                (true, false)
            }
        };
        if pids_changed {
            self.notify_all(Outgoing::AllPidInformation(self.known_pids.clone()));
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}

impl PidManager {
    fn add(&mut self, pidinfo: PidInfo) {
        let pid = Pid(pidinfo.pid().clone());
        self.known_pids.insert(pid, pidinfo);
    }

    fn remove(&mut self, pid: Pid) {
        self.known_pids.remove(&pid);
    }

    fn replace(&mut self, pid: Pid, info: PidInfo) {
        self.remove(pid);
        self.add(info);
    }

    pub fn find(&self, pid: &Pid) -> Option<&PidInfo> {
        self.known_pids.get(pid)
    }

    pub fn find_mut(&mut self, pid: &Pid) -> Option<&mut PidInfo> {
        self.known_pids.get_mut(pid)
    }

    pub fn add_unregistered(&mut self) -> Pid {
        let mut object = PidInfo::default();
        let pid: Pid;
        loop {
            let random_number = rand::thread_rng().gen::<u16>();
            let maybe_pid = Pid(format!("unregistered-{}", random_number));
            if !self.known_pids.contains_key(&maybe_pid) {
                pid = maybe_pid;
                let obj_pid = object.pid_mut();
                *obj_pid = (*pid).clone();
                break;
            }
        }
        self.known_pids.insert(pid.clone(), object);
        pid
    }

    fn notify_all(&self, msg: Outgoing) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, msg.clone())
        }
    }
}
