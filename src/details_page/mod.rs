mod type_selector;
mod edit_button;
mod profile_selector;
mod publish_button;
mod locations_list;
mod version_input;

use type_selector::*;
use edit_button::*;
use profile_selector::*;
use publish_button::*;
use locations_list::*;
use version_input::*;

use yew::prelude::*;

use crate::{
    data_type_registry::{DigitalObjectType, Pid, Profile},
    pidinfo::{PidInfo, State},
    Model,
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
}

#[derive(Debug)]
pub enum Msg {
    ToggleEditMode,
    Publish,

    ProfileChanged(Result<Profile, Pid>),
    DigitalObjectTypeChanged(Result<DigitalObjectType, Pid>),
    LocationsChanged(Vec<String>),
    VersionChanged(String),
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
            Msg::ProfileChanged(_) => {
                // TODO need to store it within the PidInfo!
            }
            Msg::DigitalObjectTypeChanged(_) => {
                // TODO need to store it within the PidInfo!
            }
            Msg::LocationsChanged(_) => {
                // TODO need to store it within the PidInfo!
            }
            Msg::VersionChanged(_) => {
                // TODO need to store it within the PidInfo!
            }
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

    fn view(&self) -> yew::Html {
        let data = &self.props.record;
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
                
                <ProfileSelector form_link=self.link.clone() active=self.edit_mode />
                <DigitalObjectTypeSelector form_link=self.link.clone() active=self.edit_mode />
                <LocationsList form_link=self.link.clone() active=self.edit_mode />
                // TODO policy
                // TODO etag
                // TODO dateCreated
                // TODO dateModified
                <VersionInput form_link=self.link.clone() active=self.edit_mode />

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