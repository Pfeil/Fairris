use std::collections::{HashMap, HashSet};

use crate::PidInfo;

use rand::prelude::*;
use strum::IntoEnumIterator;
use yew::worker::{Agent, AgentLink, Context, HandlerId};

use super::data::{Data, DataID};

pub struct DataManager {
    link: AgentLink<DataManager>,
    subscribers: HashSet<HandlerId>,

    known_data: HashMap<DataID, Data>,
    selected: Option<DataID>,
}

pub enum Incoming {
    AddNewData(Data),
    UpdateData(DataID, Data),
    GetAllData,

    SelectDataId(Option<DataID>),
    AddAndSelectData(Option<Data>),
}

#[derive(Clone, Debug)]
pub enum Outgoing {
    AllData(HashMap<DataID, Data>),
    SelectedData(Option<(DataID, Data)>),
}

impl Agent for DataManager {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Incoming;
    type Output = Outgoing;

    fn create(link: yew::worker::AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: Default::default(),
            known_data: Default::default(),
            selected: None,
        }
    }

    fn update(&mut self, msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, id: yew::worker::HandlerId) {
        let (has_data_changed, has_selection_changed): (bool, bool) = match msg {
            Incoming::AddNewData(data) => {
                self.add_new(data);
                (true, false)
            }
            Incoming::UpdateData(id, data) => {
                self.update(id, data);
                (true, false)
            }
            Incoming::GetAllData => {
                self.link.respond(id, Outgoing::AllData(self.known_data.clone()));
                (false, false)
            }
            Incoming::AddAndSelectData(maybe_data) => {
                if let Some(data) = maybe_data {
                    let id= self.add_new(data);
                    self.select(id);
                    (true, true)
                } else {
                    let selection_changed: bool = self.selected == None;
                    self.unselect();
                    (false, selection_changed)
                }
            }
            Incoming::SelectDataId(maybe_id) => {
                if let Some(id) = maybe_id {
                    let changed = self.selected == Some(id);
                    self.select(id);
                    (false, changed)
                } else {
                    let changed = self.selected == None;
                    self.unselect();
                    (false, changed)
                }
            }
        };
        if has_data_changed {
            self.notify_all(Outgoing::AllData(self.known_data.clone()));
        }
        if has_selection_changed {
            let selected = self.selected.map(|id| {
                let data = self.get(&id).expect("Selected data must be in the hashmap.");
                (id, data.clone())
            });
            self.notify_all( Outgoing::SelectedData(selected));
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}

// private API
impl DataManager {
    fn notify_all(&self, msg: Outgoing) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, msg.clone())
        }
    }
}

// public API
impl DataManager {
    pub fn add_new(&mut self, data: Data) -> DataID {
        let id: DataID;
        loop {
            let maybe_pid = DataID(rand::thread_rng().gen::<u16>());
            if !self.known_data.contains_key(&maybe_pid) {
                id = maybe_pid;
                break;
            }
        }
        self.known_data.insert(id, data);
        id
    }

    pub fn update(&mut self, id: DataID, data: Data) {
        self.known_data.insert(id, data);
    }

    pub fn get(&self, id: &DataID) -> Option<&Data> {
        self.known_data.get(id)
    }

    pub fn data_iter(&self) -> std::collections::hash_map::Iter<'_, DataID, Data> {
        self.known_data.iter()
    }

    pub fn select(&mut self, id: DataID) {
        if self
            .known_data
            .keys()
            .filter(|&&some_id| some_id == id)
            .count()
            > 0
        {
            self.selected = Some(id);
        }
    }

    pub fn unselect(&mut self) {
        self.selected = None;
    }

    pub fn get_selected(&self) -> Option<DataID> {
        self.selected
    }

    pub fn get_from_record(&self, item: &PidInfo) -> Option<(DataID, Data)> {
        if let Some(id) = &item.data {
            log::debug!("Found some data in record: {:?}", id);
            self.known_data
                .get(&id)
                .map(|data| (id.clone(), data.clone()))
        } else {
            // TODO here you may handle retrieving data out of the raw record inside the pidinfo
            //      (map data info to data objects, send GET request to collection service, etc)
            log::warn!("Did not find existing data. Implement creating data here (maybe).");
            None
        }
    }
}

impl std::fmt::Debug for DataManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KnownData(...)")
    }
}
