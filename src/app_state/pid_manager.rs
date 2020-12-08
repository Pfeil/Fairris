use std::collections::{HashMap, HashSet};

use crate::{data_type_registry::Pid, pidinfo::PidInfo, service_communication::PidRecord};

use rand::prelude::*;
use yew::{worker::{Agent, AgentLink, Context, HandlerId}};

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
    AddPidInfo(PidInfo),  // TODO rename: This one updates record AND local changes into the state
    UpdateRecord(Pid, PidRecord),  // TODO rename: This one updates only the internal record. The equivalent for the local changed might be a good idea.
    RemovePidInfo(Pid),
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

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
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
                self.remove(&pid);
                // TODO it would be possible to select something else (or nothing) in case the deleted is selected.
                (true, false)
            }
            Incoming::UpdateRecord(pid, record) => {
                self.update_record(&pid, record);
                let selection_was_changed = self.selected == Some(pid);
                (true, selection_was_changed)
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

    fn remove(&mut self, pid: &Pid) {
        self.known_pids.remove(&pid);
    }

    fn update_record(&mut self, pid: &Pid, record: PidRecord) {
        let data_id = self.known_pids.get(&pid).map(|old_pidinfo| old_pidinfo.data).flatten();
        self.remove(pid);
        let mut new_object = PidInfo::from_registered(record);
        new_object.data = data_id;
        self.add(new_object);
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
