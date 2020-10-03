use super::{AppRoute, Model, Msg};
use crate::service_communication::pit_record::PidRecord;
use serde_json as json;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug)]
pub struct PidInfo {
    record: PidRecord,
    state: State,
    model_link: ComponentLink<Model>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Unregistered,
    Modified,
    Clean,
}

impl PidInfo {
    pub fn default(model_link: ComponentLink<Model>) -> Self {
        PidInfo {
            record: PidRecord::default(),
            state: State::Unregistered,
            model_link,
        }
    }

    pub fn from_registered(record: PidRecord, model_link: ComponentLink<Model>) -> Self {
        Self::from(record, State::Clean, model_link)
    }

    pub fn from_unregistered(record: PidRecord, model_link: ComponentLink<Model>) -> Self {
        Self::from(record, State::Unregistered, model_link)
    }

    pub fn from_modified(record: PidRecord, model_link: ComponentLink<Model>) -> Self {
        Self::from(record, State::Modified, model_link)
    }

    fn from(record: PidRecord, state: State, model_link: ComponentLink<Model>) -> Self {
        Self {
            record,
            state,
            model_link,
        }
    }

    pub fn state(&self) -> State {
        self.state.clone()
    }

    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }

    pub fn pid(&self) -> &String {
        &self.record.pid
    }

    pub fn pid_mut(&mut self) -> &mut String {
        &mut self.record.pid
    }

    pub fn describe(&self) -> String {
        self.record.describe()
    }

    pub fn view_as_list_item(&self) -> Html {
        let pid = self.record.pid.clone();
        let pid2 = pid.clone();
        html! {
                <div class="piditem">
                <RouterButton<AppRoute> route=AppRoute::Details{path: pid} classes="fdo-button">
                    <p>{ self.record.pid.as_str() }</p>
                    <p>{ self.record.describe() }</p>
                    <p>{ format!("{:?}", self.state) }</p>
                </RouterButton<AppRoute>>
                <button onclick=self.model_link.callback( move |_| Msg::Remove(pid2.clone()) ) class="fdo-remove-button">{"✗"}</button>
                </div>
        }
    }

    pub fn view_record(&self) -> Html {
        self.record
            .entries
            .keys()
            .map(|property_pid| self.record.entries.get(property_pid).unwrap())
            .flatten()
            .map(|entry| {
                html! {
                    <>
                        <p class="align-right">{ &entry.name } <br/> { &entry.key }</p>
                        <p>{ json::to_string_pretty(&entry.value).unwrap() }</p>
                    </>
                }
            })
            .collect()
    }
}

impl PartialEq for PidInfo {
    fn eq(&self, other: &Self) -> bool {
        self.record == other.record && self.state == other.state
    }
}

impl Eq for PidInfo {}
