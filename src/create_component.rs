use super::Model;
use super::service_communication::{
    pit_record::PidRecord,
    datatypes::*,
};

use yew::prelude::*;
use yew::services::{fetch::{FetchService, Request, Response, FetchTask}};
use yew::format::Json;
use anyhow::Error;
use serde_json as json;
use crate::pidinfo::PidInfo;

pub struct CreateComponent {
    link: ComponentLink<Self>,
    props: Props,
    task: Option<FetchTask>,

    profile: Profile,  // should work
    data_type: DataType,  // should work
    data_url: ObjectLocation, // should work
    policy: Policy,  // should work
    etag: Checksum,  // should work
    // TODO optional date_modified: String,
    date_created: DateTimeHandle,
    version: String,
    // TODO many optional handles:
    //derived_from: Pid,
    //specialization_of: Pid,
    //revision_of: Pid,
    //primary_source: Pid,
    //quoted_from: Pid,
    //alternate_of: Pid,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub model_link: ComponentLink<Model>,
}

#[derive(Debug)]
pub enum Msg {
    ChangeProfile(ChangeData),
    ChangeDataType(ChangeData),
    ChangeDataURL(ChangeData),
    ChangeLifecycle(ChangeData),
    ChangeLicense(ChangeData),
    ChangeVersion(ChangeData),
    SendForm,
    Error(String),
}

impl Component for CreateComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            task: None,

            profile: Profile::default(),
            data_type: DataType::default(),
            data_url: ObjectLocation::default(),
            policy: Policy::default(),
            etag: Checksum::default(),
            date_created: DateTimeHandle::default(),
            version: String::new(),
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::debug!("Form Received an update");
        match msg {
            Msg::ChangeProfile(ChangeData::Select(input /*stdweb::web::html_element::SelectElement*/)) => {
                log::info!("Change form profile to: ({:?}){:?}", input.selected_index(), input.value());
                self.profile = Profile::from(input.selected_index());
            }
            Msg::ChangeDataType(ChangeData::Select(input)) => {
                log::info!("Change data type to: ({:?}){:?}", input.selected_index(), input.value());
                self.data_type = DataType::from(input.selected_index());
            }
            Msg::ChangeDataURL(ChangeData::Value(url)) => {
                log::info!("Change data URL to: {}", url);
                *self.data_url = url;
            }
            Msg::ChangeLifecycle(ChangeData::Select(input)) => {
                log::info!("Change lifecycle to: ({:?}){:?}", input.selected_index(), input.value());
                self.policy.lifecycle = Lifecycle::from(input.selected_index());
            }
            Msg::ChangeLicense(ChangeData::Select(input)) => {
                log::info!("Change license to: ({:?}){:?}", input.selected_index(), input.value());
                self.policy.license = License::from(input.selected_index());
            }
            Msg::ChangeVersion(ChangeData::Value(version)) => {
                log::info!("Change version to: {}", version);
                self.version = version;
            }
            Msg::SendForm => {
                let new_object = self.extract_record();
                let request = Request::post("http://localhost:8090/api/v1/pit/pid/")
                    .header("Content-Type", "application/json")
                    .body(Json(&new_object))
                    .expect("Failed to build this request.");
                let task = FetchService::fetch(
                    request,
                    self.props.model_link.callback(|response: Response<Result<String, Error>>| {
                        if response.status().is_success() {
                            serde_json::from_str(
                                response.body().as_ref().expect("Get reference from body.").as_str()
                            ).and_then(|record| {
                                Ok(super::Msg::AddPidItem(PidInfo::from_registered(record)))
                            }).unwrap_or_else(|e| super::Msg::Error(format!("Error parsing record: {:?}", e)) )
                        } else {
                            // TODO should not the form actually show some error here?
                            super::Msg::Error(format!("HTTP error: {:?}", response))
                        }
                    }),
                ).map_err(|e| log::error!("Error requesting PID creation: {}", e));
                self.task = task.ok();
                log::info!("SendForm has finished.");
            }
            other => log::error!("Unimplemented message: {:?}", other),
        };
        true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        match self.profile {
            Profile::AnnotatedImageProfile => html! {
                <div id="content" class="maincolumns scroll-vertical">
                    <div class="column-form">
                        // profile selection
                        { self.profile.display_form(&self.link) }
                        // data type
                        { self.data_type.display_form(&self.link) }
                        // Data URL
                        { self.data_url.display_form(&self.link) }
                        // Policy
                        { self.policy.display_form(&self.link) }
                        // Checksum
                        { self.etag.display_form(&self.link) }

                        // creationDateTime
                        { self.date_created.display_form(&self.link) }
                        // version
                        <label class="form-description" for="fdo-version">{ "Object Version String:" }</label>
                        <input class="form-input" type="text" id="fdo-version" required=true
                            onchange=self.link.callback(|e: ChangeData| Msg::ChangeVersion(e))
                        />
    
                        <p>{ "The following input fields are there because of the PID Kernel Information Recommendations." }</p>
                        <p>{ "They are temporarily ignored because they are not included in the HMC profile this demo uses." }</p>
    
                        <label class="form-description" for="fdo-metadata">{ "Metadata handle:" }</label>
                        <input class="form-input" type="text" id="fdo-metadata" required=true />
    
                        <label class="form-description" for="derived-from">{ "Derived from (handles):" }</label>
                        <input class="form-input" type="text" id="derived-from" required=true />
    
                        <label class="form-description" for="specialization-of">{ "Specialization of (handles):" }</label>
                        <input class="form-input" type="text" id="specialization-of" required=true />
    
                        <label class="form-description" for="revision-of">{ "Revision of (handles):" }</label>
                        <input class="form-input" type="text" id="revision-of" required=true />
    
                        <label class="form-description" for="primary-source">{ "Primary sources (handles):" }</label>
                        <input class="form-input" type="text" id="primary-source" required=true />
    
                        <label class="form-description" for="quoted-from">{ "Quoted from (handles):" }</label>
                        <input class="form-input" type="text" id="quoted-from" required=true />
    
                        <label class="form-description" for="alternate-of">{ "Alternative of (handles):" }</label>
                        <input class="form-input" type="text" id="alternate-of" required=true />
                    </div>
                    <button class="okbutton" onclick=self.link.callback(|_| Msg::SendForm)>{ "Create FDO Record" }</button>
    
                </div>
            },
            _ => html!{<div>{ "Not implemented yet." }</div>}
        }
    }
}

impl CreateComponent {
    fn extract_record(&self) -> serde_json::Value {
        let mut record = PidRecord::default();
        self.profile.set_into(&mut record);
        self.data_type.set_into(&mut record);
        self.data_url.set_into(&mut record);
        self.policy.set_into(&mut record);
        self.etag.set_into(&mut record);
        self.date_created.set_into(&mut record);
        serde_json::to_value(record).unwrap()
    }
}

/// Types which implement this trait can be used to represent types of the data type repository,
/// as they can set themselves into a given PidRecord.
/// They can also display proper html Controls to control them.
trait RecordProperty {
    // TODO better name, like write_into or so.
    fn set_into(&self, record: &mut PidRecord);
    fn display_form(&self, link: &ComponentLink<CreateComponent>) -> Html;
}

impl RecordProperty for Profile {
    fn set_into(&self, record: &mut PidRecord) {
        let id = "21.T11148/076759916209e5d62bd5".into();
        let name = "KernelInformationProfile".into();
        let recommended_profile = "21.T11148/0c5636e4d82b88f86132".into();
        match self {
            Profile::AnnotatedImageProfile => record.add_attribute(id, name, recommended_profile),
            Profile::OtherProfile => {}
        };
    }

    fn display_form(&self, link: &ComponentLink<CreateComponent>) -> Html {
        html! {
            <>
                <label class="form-description" for="profile-select">{ "Profile:" }</label>
                <select class="form-input" id="profile-select"
                        onchange=link.callback(|e: ChangeData| Msg::ChangeProfile(e))>
                    <option>{ "Annotated Images (Pair(s) of Image file + PageAnnotation)" }</option>
                    <option>{ "Other profile (manual)" }</option>
                    //<option>{ "Metadata Document" }</option>
                </select>
            </>
        }
    }
}

impl RecordProperty for DataType {
    fn set_into(&self, record: &mut PidRecord) {
        let id = "21.T11148/1c699a5d1b4ad3ba4956".into();
        let name = "digitalObjectType".into();
        let image = "21.T11148/2834eac0159f584bcf05".into();
        // TODO how to set the image type png/tiff?
        match self {
            DataType::Tiff => record.add_attribute(id, name, image),
            DataType::Png  => record.add_attribute(id, name, image),
        };
    }

    fn display_form(&self, link: &ComponentLink<CreateComponent>) -> Html {
        html! {
            <>
                <label class="form-description" for="fdo-type">{ "Digital Object Data Type:" }</label>
                <select class="form-input" id="fdo-type" onchange=link.callback(|e: ChangeData| Msg::ChangeDataType(e))>
                    <option>{ "image/TIFF (media-type-IANA-image)" }</option>
                    <option>{ "image/PNG  (media-type-IANA-image)" }</option>
                </select>
            </>
        }
    }
}

impl RecordProperty for Policy {
    fn set_into(&self, record: &mut PidRecord) {
        // 1. create value
        let id: String = "21.T11148/8074aed799118ac263ad".into();
        let name: String = "digitalObjectPolicy".into();
        // TODO implement real pid to fitting object
        let value = json::Value::String(format!("policy/default"));
        // 2. set into record
        record.add_attribute(id, name, value);
        // TODO record.add_attribute(...);
    }

    fn display_form(&self, link: &ComponentLink<CreateComponent>) -> Html {
        html! {
            <>  // Lifecycle
                <label class="form-description" for="fdo-lifecycle">{ "Lifecycle:" }</label>
                <select class="form-input" id="fdo-lifecycle"
                        onchange=link.callback(|e: ChangeData| Msg::ChangeLifecycle(e))>
                    <option>{ "static" }</option>
                    <option>{ "dynamic and regular updates" }</option>
                    <option>{ "dynamic and irregular updates" }</option>
                </select>
                // License
                <label class="form-description" for="fdo-license">{ "License:" }</label>
                <select class="form-input" id="fdo-license"
                        onchange=link.callback(|e: ChangeData| Msg::ChangeLicense(e))>
                    <option>{ "MIT" }</option>
                    <option>{ "Apache" }</option>
                    <option>{ "CC-by" }</option>
                    <option>{ "other (choose from field)" }</option>
                </select>
            </>
        }
    }
}

impl RecordProperty for Checksum {
    fn set_into(&self, record: &mut PidRecord) {
        let id = "21.T11148/92e200311a56800b3e47".into();
        let name = "etag".into();
        //let value = json::Value::String(format!("{}", self.value));
        let value = json::json!({
            "sha256sum": json::Value::String(format!("sha256: {}", self.value))
        });
        let value = json::Value::String(value.to_string());  // PIT service only parses json strings as values
        record.add_attribute(id, name, value);
    }

    fn display_form(&self, link: &ComponentLink<CreateComponent>) -> Html {
        // TODO just a dummy.
        html! {
        <> // TODO calculate hash yourself? (download image and calculate)
            <label class="form-description" for="fdo-etag">{ "etag (object hash):" }</label>
            <select class="form-input" id="fdo-etag"
                    onchange=link.callback(|e: ChangeData| Msg::ChangeLicense(e))>
                <option>{ "Calculate from Location Body" }</option>
                <option>{ "Provide manually (TODO create-text-unput on selection)" }</option>
                <option>{ "What if the location is a stream?" }</option>
            </select>
        </>
        }
    }
    
}

impl RecordProperty for ObjectLocation {
    fn set_into(&self, record: &mut PidRecord) {
        let id = "21.T11148/b8457812905b83046284".into();
        let name = "digitalObjectLocation".into();
        let value = json::Value::String((*self).clone());
        record.add_attribute(id, name, value);
    }

    fn display_form(&self, link: &ComponentLink<CreateComponent>) -> Html {
        html!(
            <>
                <label class="form-description" for="fdo-data-url">{ "Digital Object Data URL (or Handle?):" }</label>
                <input class="form-input" type="url" id="fdo-data-url"  required=true
                    onchange=link.callback(|e: ChangeData| Msg::ChangeDataURL(e))
                />
            </>
        )
    }
    
}

impl RecordProperty for DateTimeHandle {
    fn set_into(&self, record: &mut PidRecord) {
        let id = "21.T11148/29f92bd203dd3eaa5a1f".into();
        let name = "dateCreated".into();
        let value = json::Value::String( format!("{}", self.format("%F %T")) );
        record.add_attribute(id, name, value);
    }

    fn display_form(&self, link: &ComponentLink<CreateComponent>) -> Html {
        html!(
            <>
                <label class="form-description" for="fdo-data-url">{ "creation date and time:" }</label>
                <p>{ "TODO (date time chooser or free text field)" }</p>
            </>
        )
    }
    
}