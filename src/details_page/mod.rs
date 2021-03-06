mod data_widget;
mod date_created_input;
mod date_modified_input;
mod edit_button;
mod etag_input;
mod helpers;
mod locations_list;
mod policy_input;
mod profile_selector;
mod publish_button;
mod type_selector;
mod version_input;

use data_widget::*;
use date_created_input::*;
use date_modified_input::*;
use edit_button::*;
use etag_input::*;
use locations_list::*;
use policy_input::*;
use profile_selector::*;
use publish_button::*;
use type_selector::*;
use version_input::*;

use yew::{agent::Dispatcher, prelude::*};

use crate::{pit_service::PitService, app_state::{data::DataID, data_manager, data_manager::DataManager, pid_manager, pid_manager::PidManager}, data_type_registry::{
        DateCreated, DateModified, DigitalObjectType, Etag, Locations, Pid, Policy, Profile,
        Version,
    }, pidinfo::{PidInfo, State}};

pub struct DetailsPage {
    link: ComponentLink<Self>,
    props: Props,

    pit_service: Dispatcher<PitService>,
    pid_manager: Dispatcher<PidManager>,
    data_manager: Dispatcher<DataManager>,

    edit_mode: bool,
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
    // the record this details page represents
    pub record: PidInfo,
}

#[derive(Debug)]
pub enum Msg {
    ToggleEditMode,
    Publish,

    ProfileChanged(Result<Profile, Pid>),
    DigitalObjectTypeChanged(Result<DigitalObjectType, Pid>),
    LocationsChanged(Locations),
    DateCreatedChanged(DateCreated),
    DateModifiedChanged(DateModified),
    VersionChanged(Version),
    PolicyChanged(Policy),
    EtagChanged(Etag),

    DataChanged(Option<DataID>),
}

impl Component for DetailsPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        let mut new_self = Self {
            link,
            props,
            pit_service: PitService::dispatcher(),
            pid_manager: PidManager::dispatcher(),
            data_manager: DataManager::dispatcher(),
            edit_mode: false,
        };
        new_self.sync_page_to_record_state();
        new_self
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        log::debug!("Details page received message: {:?}", msg);
        match msg {
            Msg::ToggleEditMode => {
                use pid_manager::Incoming;
                self.edit_mode = !self.edit_mode;
                self.props.record.update_state();
                if !self.edit_mode {
                    self.pid_manager
                        .send(Incoming::AddPidInfo(self.props.record.clone()));
                }
            }
            Msg::Publish => {
                use crate::pit_service::Request as PitReq;
                match self.props.record.state() {
                    State::Clean => log::error!("Status is clean. This should not happen."),
                    State::Modified => self.pit_service.send(PitReq::Update(self.props.record.clone())),
                    State::Unregistered => self.pit_service.send(PitReq::Register(self.props.record.clone())),
                }
            }

            Msg::ProfileChanged(p) => {
                self.props.record.profile = p.map_err(|e| Some(e));
            }
            Msg::DigitalObjectTypeChanged(t) => {
                self.props.record.digital_object_type = t.map_err(|e| Some(e));
            }
            Msg::LocationsChanged(l) => self.props.record.locations = l,
            Msg::VersionChanged(v) => self.props.record.version = v,
            Msg::PolicyChanged(policy) => self.props.record.policy = policy,
            Msg::EtagChanged(etag) => self.props.record.etag = etag,
            Msg::DateCreatedChanged(date) => self.props.record.date_created = date,
            Msg::DateModifiedChanged(date) => self.props.record.date_modified = date,
            Msg::DataChanged(id) => {
                // update own state
                self.props.record.data = id;
                // update global state
                let record = self.props.record.clone();
                use pid_manager::Incoming;
                self.pid_manager.send(Incoming::AddPidInfo(record));
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let changed = self.props.record != props.record;
        if changed {
            log::debug!("Detail Page Change: {:?}", &props);
            self.props = props;
            self.sync_page_to_record_state();
        }
        changed
    }

    #[allow(unused_must_use)]
    fn view(&self) -> yew::Html {
        let data = &self.props.record;
        let digital_object_type = self.props.record.digital_object_type.clone();
        let profile = self.props.record.profile.clone();
        let locations = self.props.record.locations.clone();
        let date_created = self.props.record.date_created.clone();
        let date_modified = self.props.record.date_modified.clone();
        let policy = self.props.record.policy.clone();
        let etag = self.props.record.etag.clone();
        let version = self.props.record.version.clone();
        html! {
            <div id="content" class="maincolumns scroll-vertical">
                <div class="two-column-lefty">
                    <div class="image-placeholder"><p>{ "Preview" }</p></div>
                    <div class="two-column-lefty">
                        <p class="align-right">{ "PID:" }</p>
                        <p>{ data.pid().as_str() }</p>
                        <p class="align-right">{ "Description:" }</p>
                        <p>{ data.describe() }</p>
                        <p class="align-right">{ "Status:" }</p>
                        <p>{ format!("{:?}", data.state()) }</p>
                    </div>
                </div>

                <DataWidget detail_page=self.link.clone() />

                <details open=true>
                    <summary>{ "Record Metadata (raw)" }</summary>
                    <div class="two-column-lefty">{ data.view_record() }</div>
                </details>

                <details open=true>
                    <summary>{ "Record Metadata (editable)" }</summary>
                    <div class="column-form">
                        <EditButton form_link=self.link.clone() edit_mode=self.edit_mode />
                        <PublishButton form_link=self.link.clone() edit_mode=self.edit_mode state=self.props.record.state() />
                    </div>

                    <div class="two-column-lefty">
                        <ProfileSelector form_link=self.link.clone() active=self.edit_mode maybe_profile=profile />
                        <DigitalObjectTypeSelector form_link=self.link.clone() active=self.edit_mode maybe_type=digital_object_type/>
                        <LocationsList form_link=self.link.clone() active=self.edit_mode locations=locations />
                        <PolicyInput form_link=self.link.clone() active=self.edit_mode policy=policy />
                        <EtagInput form_link=self.link.clone() active=self.edit_mode etag=etag />
                        <DateCreatedInput form_link=self.link.clone() active=self.edit_mode date_created=date_created />
                        <DateModifiedInput form_link=self.link.clone() active=self.edit_mode date_modified=date_modified />
                        <VersionInput form_link=self.link.clone() active=self.edit_mode version=version />
                    </div>
                </details>

            </div>
        }
    }
}

impl DetailsPage {
    fn sync_page_to_record_state(&mut self) {
        use data_manager::Incoming;
        self.edit_mode = false;
        let id = self.props.record.data;
        self.data_manager.send(Incoming::SelectDataId(id));
    }
}
