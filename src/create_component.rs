use super::Model;
use super::service_communication::{
    RecordEntry,
    pit_record::PidRecord,
    primitive_types::*,  // TODO make newtypes around primitives so they are reusable and this import is not needed anymore.
    types::*,
};

use yew::prelude::*;
use yew::services::{fetch::{FetchService, Request, Response, FetchTask}};
use yew::format::Json;
use anyhow::Error;
use crate::pidinfo::PidInfo;

pub struct CreateComponent {
    link: ComponentLink<Self>,
    props: Props,
    task: Option<FetchTask>,
    
    profile: Profile,
    data_type: DataType,
    data_url: ObjectLocation,
    policy: Policy,
    etag: Checksum,
    date_created: DateCreated,
    date_modified: DateModified,
    metadata_document: MetadataDocument,
    license_string: LicenseString,
    
    metadata_document_pid: MetadataDocumentPid,
    version: Version,
    contributors: Contributors,

    // Recommended profile (optional)
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
    // IS_DUMMY ChangeChecksum(ChangeData),
    // IS_DUMMY ChangeDateCreation(ChangeData),

    ChangeVersion(ChangeData),

    // IS_DUMMY ChangeDateModification(ChangeData),
    
    ChangeMetadataPidUrl(ChangeData),
    ChangeMetadataSchemaUrl(ChangeData),
    ChangeMetadataTypeUrl(ChangeData),
    ChangeLicenseString(ChangeData),

    ChangeMetadataPid(ChangeData),
    
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
            date_created: DateCreated::default(),
            date_modified: DateModified::default(),
            version: Version::default(),

            metadata_document: MetadataDocument::default(),
            metadata_document_pid: MetadataDocumentPid::default(),
            license_string: LicenseString::default(),
            contributors: Contributors::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::debug!("Form received an update: {:?}", msg);
        match msg {
            Msg::ChangeProfile(ChangeData::Select(input /*stdweb::web::html_element::SelectElement*/)) => {
                log::info!("Change form profile to: ({:?}){:?}", input.selected_index(), input.value());
                self.profile = Profile::from(input.selected_index());
            }
            Msg::ChangeDataType(ChangeData::Select(input)) => {
                log::info!("Change data type to: ({:?}){:?}", input.selected_index(), input.value());
                self.data_type = DataType::from(input.selected_index());
            }
            Msg::ChangeDataURL(ChangeData::Value(url)) => *self.data_url = url,
            Msg::ChangeLifecycle(ChangeData::Select(input)) => {
                log::info!("Change lifecycle to: ({:?}){:?}", input.selected_index(), input.value());
                self.policy.lifecycle = Lifecycle::from(input.selected_index());
            }
            Msg::ChangeLicense(ChangeData::Select(input)) => {
                log::info!("Change license to: ({:?}){:?}", input.selected_index(), input.value());
                self.policy.license = License::from(input.selected_index());
            }
            Msg::ChangeVersion(ChangeData::Value(version)) => *self.version = version,
            Msg::ChangeMetadataPidUrl(ChangeData::Value(url)) => self.metadata_document.pid = url,
            Msg::ChangeMetadataSchemaUrl(ChangeData::Value(url)) => self.metadata_document.scheme = url,
            Msg::ChangeMetadataTypeUrl(ChangeData::Value(url)) => self.metadata_document.r#type = url,
            Msg::ChangeLicenseString(ChangeData::Value(license)) => *self.license_string = license,
            Msg::ChangeMetadataPid(ChangeData::Value(pid)) => *self.metadata_document_pid = pid,
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
        log::debug!("View was called. Profile is {:?}", self.profile);
        html! {
            <div id="content" class="maincolumns scroll-vertical">
                <p>{ format!("Adjust the profile to get a form fitting your FDO. Current Profile is {:?}", self.profile) }</p>
                <div class="two-column-lefty">
                    { self.profile.display_form(&self.link) }
                    { self.data_type.display_form(&self.link) }
                    { self.data_url.display_form(&self.link) }
                    { self.metadata_document.display_form(&self.link) }
                    { self.policy.display_form(&self.link) }
                    { self.etag.display_form(&self.link) }
                    { self.date_created.display_form(&self.link) }
                    { self.date_modified.display_form(&self.link) }
                    { self.version.display_form(&self.link) }
                    { self.contributors.display_form(&self.link) }

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
        }
    }
}

impl CreateComponent {
    fn extract_record(&self) -> serde_json::Value {
        let mut record = PidRecord::default();
        self.profile.write(&mut record);
        self.data_type.write(&mut record);
        self.data_url.write(&mut record);
        self.metadata_document.write(&mut record);
        self.policy.write(&mut record);
        self.etag.write(&mut record);
        self.date_created.write(&mut record);
        self.date_modified.write(&mut record);
        self.version.write(&mut record);
        // TODO self.contributors.write(&mut record);

        serde_json::to_value(record).unwrap()
    }
}

// TODO move all newtypes and "Recordable" implementations into crate::service_communication::types

trait FormElement: RecordEntry {
    /// Returns Html that can be used within components.
    fn display_form(&self, link: &ComponentLink<CreateComponent>) -> Html;
}

impl FormElement for Profile {
    fn display_form(&self, link: &ComponentLink<CreateComponent>) -> Html {
        html! {
            <>
                <label class="form-description" for="profile-select">{ "Profile:" }</label>
                <select class="form-input" id="profile-select" required=true
                        onchange=link.callback(|e| Msg::ChangeProfile(e))>
                    <option>{ "Simplified Form" }</option>
                    <option>{ "Raw Profile Form" }</option>
                </select>
            </>
        }
    }
}

impl FormElement for DataType {
    fn display_form(&self, link: &ComponentLink<CreateComponent>) -> Html {
        html! {
            <>
                <label class="form-description" for="fdo-type">{ "Digital Object Data Type:" }</label>
                <select class="form-input" id="fdo-type" required=true
                        onchange=link.callback(|e: ChangeData| Msg::ChangeDataType(e))>
                    <option>{ "image/TIFF (media-type-IANA-image)" }</option>
                    <option>{ "image/PNG  (media-type-IANA-image)" }</option>
                </select>
            </>
        }
    }
}

impl FormElement for Policy {
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

impl FormElement for Checksum {
    fn display_form(&self, _link: &ComponentLink<CreateComponent>) -> Html {
        html! {
        <> // TODO calculate hash yourself? (download image and calculate)
            <label class="form-description" for="fdo-etag">{ "etag (object hash):" }</label>
            <p>{ "In real world examples, the application would calculate the checksum automatically. Currently, a default is used." }</p>
        </>
        }
    }
}

impl FormElement for ObjectLocation {
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

impl FormElement for DateCreated {
    fn display_form(&self, _link: &ComponentLink<CreateComponent>) -> Html {
        html!(
            <>
                <label class="form-description">{ "creation date and time:" }</label>
                <p>{ "For testing purposes and to ease demonstration, the point of time that this page was loaded is chosen to be the creation time." }</p>
            </>
        )
    }
}

impl FormElement for DateModified {
    fn display_form(&self, _link: &ComponentLink<CreateComponent>) -> Html {
        html!(
            <>
                <label class="form-description">{ "Modification date and time:" }</label>
                <p>{ "For testing purposes and to ease demonstration, the point of time that this page was loaded is chosen to be the modification time." }</p>
            </>
        )
    }
}

impl FormElement for Version {
    fn display_form(&self, link: &ComponentLink<CreateComponent>) -> Html {
        html!(
            <>
            <label class="form-description" for="fdo-version">{ "Object Version String:" }</label>
            <input class="form-input" type="text" id="fdo-version"
                onchange=link.callback(|e: ChangeData| Msg::ChangeVersion(e))
            />
            </>
        )
    }
}

impl FormElement for MetadataDocumentPid {
    fn display_form(&self, link: &ComponentLink<CreateComponent>) -> Html {
        html! {
            <>
            <label class="form-description" for="fdo-metadata">{ "Metadata handle:" }</label>
            <input class="form-input" type="text" id="fdo-metadata" required=true
                onchange=link.callback(|e: ChangeData| Msg::ChangeMetadataPid(e))
            />
            </>
        }
    }
}

impl FormElement for MetadataDocument {
    fn display_form(&self, link: &ComponentLink<CreateComponent>) -> Html {
        html! {
            <>
            <label class="form-description" for="fdo-metadata-id">{ "Metadata PID as URL:" }</label>
            <input class="form-input" type="text" id="fdo-metadata-id" required=true
                onchange=link.callback(|e: ChangeData| Msg::ChangeMetadataPidUrl(e))
            />
            <label class="form-description" for="fdo-metadata-schema">{ "Metadatas schema PID as URL:" }</label>
            <input class="form-input" type="text" id="fdo-metadata-schema" required=true
                onchange=link.callback(|e: ChangeData| Msg::ChangeMetadataSchemaUrl(e))
            />
            <label class="form-description" for="fdo-metadata-type">{ "Metadatas type PID as URL:" }</label>
            <input class="form-input" type="text" id="fdo-metadata-type" required=true
                onchange=link.callback(|e: ChangeData| Msg::ChangeMetadataTypeUrl(e))
            />
            </>
        }
    }
}

impl FormElement for LicenseString {
    fn display_form(&self, link: &ComponentLink<CreateComponent>) -> Html {
        html! {
            <>
            <label class="form-description" for="fdo-license">{ "License:" }</label>
            <input class="form-input" type="text" id="fdo-license" required=true
                onchange=link.callback(|e: ChangeData| Msg::ChangeLicenseString(e))
            />
            </>
        }
    }
}

impl FormElement for Contributors {
    fn display_form(&self, _link: &ComponentLink<CreateComponent>) -> Html {
        html! {
            <>
            <p>{ "Contributors:" }</p>
            <p>{ "Placeholder contributers will be inserted for demonstration." }</p>
            </>
        }
    }
}