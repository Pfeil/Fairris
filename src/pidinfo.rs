use std::convert::TryFrom;

use super::{AppRoute, Model, Msg};
use crate::{
    data_type_registry::*,
    service_communication::pit_record::PidRecord
};
use serde_json as json;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug)]
pub struct PidInfo {
    // The published record (if published)
    record: PidRecord,
    state: State,
    model_link: ComponentLink<Model>,

    // The record will contain the published record (if published).
    // The variables below contain the local state.
    // This way, it is possible to determine if the current local state is clean or modified.
    pub profile: MaybeProfile,
    pub digital_object_type: MaybeDOType,
    pub locations: Locations,
    pub date_created: DateCreated,
    pub date_modified: DateModified,
    pub etag: Etag,
    pub policy: Policy,
    pub version: Version,
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
            profile: Ok(Profile::default()),
            digital_object_type: Ok(DigitalObjectType::default()),
            locations: Locations::default(),
            date_created: DateCreated::default(),
            date_modified: DateModified::default(),
            etag: Etag::default(),
            policy: Policy::default(),
            version: Version::default(),
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
        let profile: MaybeProfile = Profile::try_from(&record);
        let digital_object_type = DigitalObjectType::try_from(&record);
        let locations = Locations::from(&record);
        let date_created = DateCreated::from(&record);
        let date_modified = DateModified::from(&record);
        let etag = Etag::from(&record);
        let policy = Policy::from(&record);
        let version = Version::from(&record);
        Self {
            record,
            state,
            model_link,
            profile,
            digital_object_type,
            locations,
            date_created,
            date_modified,
            etag,
            policy,
            version,
        }
    }

    pub fn as_record(&self) -> PidRecord {
        let mut record_state = PidRecord::default();
        self.profile.write(&mut record_state);
        self.digital_object_type.write(&mut record_state);
        self.locations.write(&mut record_state);
        self.date_created.write(&mut record_state);
        self.date_modified.write(&mut record_state);
        self.etag.write(&mut record_state);
        self.policy.write(&mut record_state);
        self.version.write(&mut record_state);
        record_state
    }

    pub fn update_state(&mut self) {
        let old_record = &self.record;
        let new_record = &self.as_record();
        let changed = !old_record.same_content_like(new_record);
        if changed {
            match self.state {
                State::Unregistered => {}
                State::Modified => {}
                State::Clean => self.state = State::Modified,
            }
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
