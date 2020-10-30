use super::service_communication::{
    pit_record::PidRecord,
    primitive_types::*, // TODO make newtypes around primitives so they are reusable and this import is not needed anymore.
    types::*,
    RecordEntry,
    *,
};
use super::Model;

use crate::pidinfo::PidInfo;
use anyhow::Error;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

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
    metadata_document: MetadataObjectReference,
    metadata_urls: Vec<String>,

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
    // IS_DUMMY ChangeDateModification(ChangeData),
    ChangeVersion(ChangeData),
    ChangeMetadataUrls(ChangeData),

    SendForm,
    RegisteredMetadata(PidInfo),
    RegisteredObject(PidInfo),
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

            metadata_document: MetadataObjectReference::default(),
            metadata_urls: Vec::default(),
            contributors: Contributors::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::debug!("Form received an update: {:?}", msg);
        match msg {
            Msg::ChangeProfile(ChangeData::Select(
                input, /*stdweb::web::html_element::SelectElement*/
            )) => {
                log::info!(
                    "Change form profile to: ({:?}){:?}",
                    input.selected_index(),
                    input.value()
                );
                self.profile = Profile::from(input.selected_index());
            }
            Msg::ChangeDataType(ChangeData::Select(input)) => {
                log::info!(
                    "Change data type to: ({:?}){:?}",
                    input.selected_index(),
                    input.value()
                );
                self.data_type = DataType::from(input.selected_index());
            }
            Msg::ChangeDataType(ChangeData::Value(pid)) => self.data_type = DataType::Pid(pid),
            Msg::ChangeDataURL(ChangeData::Value(url)) => *self.data_url = url,
            Msg::ChangeLifecycle(ChangeData::Select(input)) => {
                log::info!(
                    "Change lifecycle to: ({:?}){:?}",
                    input.selected_index(),
                    input.value()
                );
                self.policy.lifecycle = Lifecycle::from(input.selected_index());
            }
            Msg::ChangeLicense(ChangeData::Select(input)) => {
                log::info!(
                    "Change license to: ({:?}){:?}",
                    input.selected_index(),
                    input.value()
                );
                self.policy.license = License::from(input.selected_index());
            }
            Msg::ChangeVersion(ChangeData::Value(version)) => *self.version = version,
            Msg::ChangeMetadataUrls(ChangeData::Value(urls)) => {
                self.metadata_urls.clear();
                urls
                .split("\n")
                .for_each(|url| self.metadata_urls.push(url.into()))
            },
            Msg::SendForm => {
                let new_metadata = self.extract_metadata_json();
                self.register_metadata(new_metadata);
            }
            Msg::RegisteredMetadata(item) => {
                let meta_pid = item.pid().clone();
                self.props
                    .model_link
                    .send_message(super::Msg::PidAdd(item));
                let mut record = self.extract_record();
                self.metadata_document.context = MetadataContext::Annotating;
                self.metadata_document.resource = ResourceReference::Handle(meta_pid);
                self.metadata_document.write(&mut record);
                let record = serde_json::to_value(record).unwrap();
                self.register_image_data(record);
            }
            Msg::RegisteredObject(item) => {
                // TODO think about showing a success message
                self.props
                    .model_link
                    .send_message(super::Msg::PidAdd(item))
            }
            Msg::Error(message) => log::error!("Received error: {}", message), // TODO think about showing an error message
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
                    {
                        match self.profile {
                            Profile::Testbed4infSimplified => html! { <>
                                { self.profile.display_form(&self.link) }
                                { self.data_type.display_form(&self.link) }
                                { self.data_url.display_form(&self.link) }
                                { self.metadata_document.display_form(&self.link) }
                                { self.version.display_form(&self.link) }
                                </>
                            },
                            _ => html! { <>
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
                                </>
                            }
                        }
                    }

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
    fn extract_record(&self) -> PidRecord {
        let mut record = PidRecord::default();
        self.profile.write(&mut record);
        self.data_type.write(&mut record);
        self.data_url.write(&mut record);
        //self.metadata_document.write(&mut record);
        self.policy.write(&mut record);
        self.etag.write(&mut record);
        self.date_created.write(&mut record);
        self.date_modified.write(&mut record);
        self.version.write(&mut record);
        // TODO self.contributors.write(&mut record);
        record
    }

    fn extract_json(&self) -> serde_json::Value {
        serde_json::to_value(self.extract_record()).unwrap()
    }

    fn extract_metadata_record(&self) -> PidRecord {
        let mut record = PidRecord::default();
        self.profile.write(&mut record);
        DataType::write_str(&mut record, "application/json+ld");
        self.metadata_urls.iter().for_each(|url| {
            ObjectLocation::write_str(&mut record, url.as_str());
        });
        // TODO add ontology and context as metadata.
        // defaults
        self.policy.write(&mut record);
        self.etag.write(&mut record);
        self.date_created.write(&mut record);
        self.date_modified.write(&mut record);
        self.version.write(&mut record);
        MetadataObjectReference {
            context: MetadataContext::Ontology,
            resource: ResourceReference::Url("https://www.w3.org/ns/oa#Annotation".into()),
        }
        .write(&mut record);
        record
    }

    fn extract_metadata_json(&self) -> serde_json::Value {
        serde_json::to_value(self.extract_metadata_record()).unwrap()
    }

    fn register_metadata(&mut self, record: serde_json::Value) {
        let model_link = self.props.model_link.clone();
        let callback = self
            .link
            .callback(move |response: Response<Result<String, Error>>| {
                if response.status().is_success() {
                    serde_json::from_str(
                        response
                            .body()
                            .as_ref()
                            .expect("Get reference from body.")
                            .as_str(),
                    )
                    .and_then(|record| {
                        Ok(Msg::RegisteredMetadata(PidInfo::from_registered(
                            record,
                            model_link.clone(),
                        )))
                    })
                    .unwrap_or_else(|e| Msg::Error(format!("Error parsing record: {:?}", e)))
                } else {
                    Msg::Error(format!("HTTP error: {:?}", response))
                }
            });
        self.register(record, callback);
    }

    fn register_image_data(&mut self, record: serde_json::Value) {
        let model_link = self.props.model_link.clone();
        let callback = self
            .link
            .callback(move |response: Response<Result<String, Error>>| {
                if response.status().is_success() {
                    serde_json::from_str(
                        response
                            .body()
                            .as_ref()
                            .expect("Get reference from body.")
                            .as_str(),
                    )
                    .and_then(|record| {
                        Ok(Msg::RegisteredObject(PidInfo::from_registered(
                            record,
                            model_link.clone(),
                        )))
                    })
                    .unwrap_or_else(|e| Msg::Error(format!("Error parsing record: {:?}", e)))
                } else {
                    Msg::Error(format!("HTTP error: {:?}", response))
                }
            });
        self.register(record, callback)
    }

    fn register(
        &mut self,
        record: serde_json::Value,
        callback: Callback<Response<Result<String, Error>>>,
    ) {
        log::debug!("register() was called.");
        let request = Request::post("http://localhost:8090/api/v1/pit/pid/")
            .header("Content-Type", "application/json")
            .body(Json(&record))
            .expect("Failed to build this request.");
        let task = FetchService::fetch(request, callback)
            .map_err(|e| log::error!("Error creating task to register metadata: {}", e));
        self.task = task.ok();
        log::debug!("register() has finished.");
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
        let disabled_value: bool = if let Self::Pid(_) = self { false } else { true };
        html! {
            <>
                <label class="form-description" for="fdo-type">{ "Digital Object Data Type:" }</label>
                <div>
                    <select class="form-input" id="fdo-type" required=true
                            onchange=link.callback(|e: ChangeData| Msg::ChangeDataType(e))>
                        <option>{ "image/TIFF (media-type-IANA-image)" }</option>
                        <option>{ "image/PNG  (media-type-IANA-image)" }</option>
                        <option>{ "Provide PID of a data type" }</option>
                    </select>
                    <input class="form-input" type="text" id="fdo-type-pid" disabled=disabled_value
                        onchange=link.callback(|e: ChangeData| Msg::ChangeDataType(e))
                    />
                </div>
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

impl FormElement for MetadataObjectReference {
    fn display_form(&self, link: &ComponentLink<CreateComponent>) -> Html {
        html! {
            <>
            <label class="form-description" for="fdo-metadata-urls">{ "Annotation urls:" }</label>
            <textarea class="form-input" id="fdo-metadata-urls" required=true
                onchange=link.callback(|e: ChangeData| Msg::ChangeMetadataUrls(e))
            />
            </>
        }
    }
}
