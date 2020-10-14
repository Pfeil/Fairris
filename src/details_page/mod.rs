mod helpers;
mod type_selector;
mod edit_button;
mod profile_selector;
mod publish_button;
mod locations_list;
mod version_input;
mod policy_input;

use type_selector::*;
use edit_button::*;
use profile_selector::*;
use publish_button::*;
use locations_list::*;
use version_input::*;
use policy_input::*;

use yew::prelude::*;

use crate::{Model, data_type_registry::{DigitalObjectType, Locations, Pid, Policy, Profile, Version}, pidinfo::{PidInfo, State}};

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
}

#[derive(Debug)]
pub enum Msg {
    ToggleEditMode,
    Publish,

    ProfileChanged(Result<Profile, Pid>),
    DigitalObjectTypeChanged(Result<DigitalObjectType, Pid>),
    LocationsChanged(Locations),
    VersionChanged(Version),
    PolicyChanged(Policy),
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
                if !self.edit_mode {
                    self.props
                        .model_link
                        .send_message(super::Msg::UpdatePidItem(self.props.record.clone()))
                }
            }
            Msg::Publish => {
                match self.props.record.state() {
                    State::Clean => log::error!("Status is clean. This should not happen."),
                    State::Modified => {
                        // TODO 0. send update request to PIT service
                        // TODO 1. pid will stay the same -> update item in Model
                        log::error!("UNIMPLEMENTED!")
                    }
                    State::Unregistered => {
                        // TODO 0. send create request to PIT service
                        // TODO 1. pid (dummy) will change -> add new, remove old item in Model
                        log::error!("UNIMPLEMENTED!")
                    }
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
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        let changed = self.props.record == props.record;
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
        let version = self.props.record.version.clone();
        let policy = self.props.record.policy.clone();
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
                <div class="two-column-lefty">{ data.view_record() }</div>
                <EditButton form_link=self.link.clone() edit_mode=self.edit_mode />
                <PublishButton form_link=self.link.clone() edit_mode=self.edit_mode state=self.props.record.state() />
                
                <ProfileSelector form_link=self.link.clone() active=self.edit_mode maybe_profile=profile />
                <DigitalObjectTypeSelector form_link=self.link.clone() active=self.edit_mode maybe_type=digital_object_type/>
                <LocationsList form_link=self.link.clone() active=self.edit_mode locations=locations />
                <PolicyInput form_link=self.link.clone() active=self.edit_mode policy=policy />
                // TODO etag
                // TODO dateCreated
                // TODO dateModified
                <VersionInput form_link=self.link.clone() active=self.edit_mode version=version />

                <EditButton form_link=self.link.clone() edit_mode=self.edit_mode />
                <PublishButton form_link=self.link.clone() edit_mode=self.edit_mode state=self.props.record.state() />
            </div>
        }
    }
}

impl DetailsPage {
    fn reset_page_state(&mut self) {
        self.edit_mode = false;
    }
}
