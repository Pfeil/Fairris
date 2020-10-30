mod helpers;
mod type_selector;
mod edit_button;
mod profile_selector;
mod publish_button;
mod locations_list;
mod version_input;
mod policy_input;
mod etag_input;
mod date_created_input;
mod date_modified_input;
mod data_widget;

use type_selector::*;
use edit_button::*;
use profile_selector::*;
use publish_button::*;
use locations_list::*;
use version_input::*;
use policy_input::*;
use etag_input::*;
use date_created_input::*;
use date_modified_input::*;
use data_widget::*;

use yew::prelude::*;

use crate::{
    Model,
    data_type_registry::{
        DateCreated, DateModified, DigitalObjectType, Etag, Locations, Pid, Policy, Profile, Version
    },
    pidinfo::{PidInfo, State},
    known_data::*,
};

pub struct DetailsPage {
    link: ComponentLink<Self>,
    props: Props,

    edit_mode: bool,
}

#[derive(Properties, Clone)]
pub struct Props {
    // A link to send messages to the application
    pub model_link: ComponentLink<Model>,
    // the record this details page represents
    pub record: PidInfo,
    pub data: Option<(DataID, Data)>,
    pub data_descriptions: Vec<(DataID, String)>,
}

#[derive(Debug)]
pub enum Msg {
    ToggleEditMode,
    Publish,
    AnnounceData(DataID, Data),

    ProfileChanged(Result<Profile, Pid>),
    DigitalObjectTypeChanged(Result<DigitalObjectType, Pid>),
    LocationsChanged(Locations),
    DateCreatedChanged(DateCreated),
    DateModifiedChanged(DateModified),
    VersionChanged(Version),
    PolicyChanged(Policy),
    EtagChanged(Etag),
}

impl Component for DetailsPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        let mut new_self = Self {
            link,
            props,
            edit_mode: false,
        };
        new_self.reset_page_state();
        new_self
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        log::debug!("Form received message: {:?}", msg);
        match msg {
            Msg::ToggleEditMode => {
                self.edit_mode = !self.edit_mode;
                self.props.record.update_state();
                if !self.edit_mode {
                    self.props
                        .model_link
                        .send_message(super::Msg::PidAdd(self.props.record.clone()))
                }
            }
            Msg::Publish => {
                match self.props.record.state() {
                    State::Clean => log::error!("Status is clean. This should not happen."),
                    State::Modified => {
                        self.props.model_link.send_message(super::Msg::UpdateFDO(self.props.record.clone()))
                    }
                    State::Unregistered => {
                        // TODO consider a waiting animation or something in here.
                        self.props.model_link.send_message(super::Msg::RegisterFDO(self.props.record.clone()));
                    }
                }
            }
            Msg::AnnounceData(id, data) => {
                self.props.data = Some((id, data));
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
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let changed = self.props.record != props.record;
        if changed {
            self.reset_page_state();
            self.props = props;
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
                    <div class="image-placeholder"><p>{ "IMAGE" }</p></div>
                    <div class="two-column-lefty">
                        <p class="align-right">{ "PID:" }</p>
                        <p>{ data.pid().as_str() }</p>
                        <p class="align-right">{ "Description:" }</p>
                        <p>{ data.describe() }</p>
                        <p class="align-right">{ "Status:" }</p>
                        <p>{ format!("{:?}", data.state()) }</p>
                    </div>
                </div>

                <DataWidget data=self.props.data.clone() data_descriptions=self.props.data_descriptions.clone()/>
                
                <details class="two-column-lefty" open=true>
                    <summary>{ "Record Metadata (raw)" }</summary>
                    <div class="two-column-lefty">{ data.view_record() }</div>
                </details>

                <details open=true>
                    <summary>{ "Record Metadata (editable)" }</summary>
                    <div class="column-form">
                        <EditButton form_link=self.link.clone() edit_mode=self.edit_mode />
                        <PublishButton form_link=self.link.clone() edit_mode=self.edit_mode state=self.props.record.state() />
                    </div>
                    
                    <ProfileSelector form_link=self.link.clone() active=self.edit_mode maybe_profile=profile />
                    <DigitalObjectTypeSelector form_link=self.link.clone() active=self.edit_mode maybe_type=digital_object_type/>
                    <LocationsList form_link=self.link.clone() active=self.edit_mode locations=locations />
                    <PolicyInput form_link=self.link.clone() active=self.edit_mode policy=policy />
                    <EtagInput form_link=self.link.clone() active=self.edit_mode etag=etag />
                    <DateCreatedInput form_link=self.link.clone() active=self.edit_mode date_created=date_created />
                    <DateModifiedInput form_link=self.link.clone() active=self.edit_mode date_modified=date_modified />
                    <VersionInput form_link=self.link.clone() active=self.edit_mode version=version />
                </details>

            </div>
        }
    }
}

impl DetailsPage {
    fn reset_page_state(&mut self) {
        self.edit_mode = false;
    }
}
