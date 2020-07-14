use super::{Model, PidInfo};

use yew::prelude::*;
use yew::services::fetch::{FetchService, Request, Response};
use yew::format::{Json, Nothing};
use serde_json::json;
use anyhow::Error;

pub struct CreateComponent {
    link: ComponentLink<Self>,
    props: Props,

    profile: Profile,

    data_type: DataType,
    data_url: String,
    lifecycle: Lifecycle,
    license: License,
    version: String,
    // TODO many data stuff
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
            profile: Profile::default(),
            data_type: DataType::default(),
            data_url: String::new(),
            lifecycle: Lifecycle::default(),
            license: License::default(),
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
                self.data_url = url;
            }
            Msg::ChangeLifecycle(ChangeData::Select(input)) => {
                log::info!("Change lifecycle to: ({:?}){:?}", input.selected_index(), input.value());
                self.lifecycle = Lifecycle::from(input.selected_index());
            }
            Msg::ChangeLicense(ChangeData::Select(input)) => {
                log::info!("Change license to: ({:?}){:?}", input.selected_index(), input.value());
                self.license = License::from(input.selected_index());
            }
            Msg::ChangeVersion(ChangeData::Value(version)) => {
                log::info!("Change version to: {}", version);
                self.version = version;
            }
            Msg::SendForm => {
                // https://docs.rs/yew/0.17.2/yew/services/fetch/struct.FetchService.html#method.fetch
                // TODO will need a complicated function to generate this
                let content = &json!({"foo": "bar"});
                // TODO fix the request, it's just a placeholder. Do not forget to expose the ports in docker.
                let request = Request::get("https://www.rust-lang.org/")
                    //.header("User-Agent", "awesome/1.0")
                    .body(Nothing)
                    //.header("Content-Type", "application/json")
                    //.body(Json(content))
                    .expect("Failed to build this request.");
                let _task = FetchService::fetch(
                    request,
                    self.props.model_link.callback(|response: Response<Result<String, Error>>| {
                        if response.status().is_success() {
                            super::Msg::AddPidItem(PidInfo::default())  // TODO fix this
                        } else {
                            // TODO should not the form actually show some error here?
                            super::Msg::Error(format!("HTTP error: {:?}", response))
                        }
                    }),
                ).err().map(|e| log::error!("error: {}", e));
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
        // TODO match self.profile ...
        html! {
            <div id="content" class="maincolumns scroll-vertical">
                <div class="column-form">
                    // profile selection
                    <label class="form-description" for="profile-select">{ "Profile:" }</label>
                    <select class="form-input" id="profile-select"
                            onchange=self.link.callback(|e: ChangeData| Msg::ChangeProfile(e))>
                        <option>{ "Annotated Images (Pair(s) of Image file + PageAnnotation)" }</option>
                        //<option>{ "Metadata Document" }</option>
                    </select>
                    // data type
                    <label class="form-description" for="fdo-type">{ "Digital Object Data Type:" }</label>
                    <select class="form-input" id="fdo-type" onchange=self.link.callback(|e: ChangeData| Msg::ChangeDataType(e))>
                        <option>{ "image/TIFF (media-type-IANA-image)" }</option>
                        <option>{ "image/PNG  (media-type-IANA-image)" }</option>
                    </select>
                    // Data URL
                    <label class="form-description" for="fdo-data-url">{ "Digital Object Data URL (or Handle?):" }</label>
                    <input class="form-input" type="url" id="fdo-data-url"  required=true
                        onchange=self.link.callback(|e: ChangeData| Msg::ChangeDataURL(e))
                    />
                    // Lifecycle
                    <label class="form-description" for="fdo-lifecycle">{ "Lifecycle:" }</label>
                    <select class="form-input" id="fdo-lifecycle"
                            onchange=self.link.callback(|e: ChangeData| Msg::ChangeLifecycle(e))>
                        <option>{ "static" }</option>
                        <option>{ "dynamic and regular updates" }</option>
                        <option>{ "dynamic and irregular updates" }</option>
                    </select>
                    // License
                    <label class="form-description" for="fdo-license">{ "License:" }</label>
                    <select class="form-input" id="fdo-license"
                            onchange=self.link.callback(|e: ChangeData| Msg::ChangeLicense(e))>
                        <option>{ "MIT" }</option>
                        <option>{ "Apache" }</option>
                        <option>{ "CC-by" }</option>
                        <option>{ "other (choose from field)" }</option>
                    </select>
                    // Hash
                    // TODO calculate hash yourself? (download image and calculate)
                    <label class="form-description" for="fdo-etag">{ "etag (object hash):" }</label>
                    <select class="form-input" id="fdo-etag"
                            onchange=self.link.callback(|e: ChangeData| Msg::ChangeLicense(e))>
                        <option>{ "Calculate from Location Body" }</option>
                        <option>{ "Provide manually (TODO create-text-unput on selection)" }</option>
                        <option>{ "What if the location is a stream?" }</option>
                    </select>

                    <p>{ "Assumption: Date of creation/modification is done by the pit/pid-service" }</p>
                    <p>{ "No input here, therefore." }</p>
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
                // TODO This Button shall do the following:
                // 1. Initiate a HTTP request to the PIT-Service to fulfill
                // 2. Create a PidInfo Object to store in the main Model, somehow (Callback, Bridge, Agent, ...).
                // 3. Redirect to the details page of the new object. This may be done by the model handling the new object or by this component.
                //<RouterButton<AppRoute> route=AppRoute::CreateFdo classes="okbutton">{ "Create FDO Record" }</RouterButton<AppRoute>>
                <button class="okbutton" onclick=self.link.callback(|_| Msg::SendForm)>{ "Create FDO Record" }</button>

            </div>
        }
    }
}

// TODO do this with a macro

enum Profile {
    AnnotatedImageProfile,
}

impl Default for Profile {
    fn default() -> Self {
        Self::AnnotatedImageProfile
    }
}

impl From<i32> for Profile {
    fn from(index: i32) -> Self {
        match index {
            0 => Self::AnnotatedImageProfile,
            _ => Self::default(),
        }
    }
}

enum DataType {
    Tiff,
    Png,
}

impl Default for DataType {
    fn default() -> Self {
        Self::Tiff
    }
}

impl From<i32> for DataType {
    fn from(index: i32) -> Self {
        match index {
            0 => Self::Tiff,
            1 => Self::Png,
            _ => Self::default(),
        }
    }
}

enum Lifecycle {
    Static,
    RegularUpdates,
    IrregularUpdates,
}

impl Default for Lifecycle {
    fn default() -> Self {
        Self::Static
    }
}

impl From<i32> for Lifecycle {
    fn from(index: i32) -> Self {
        match index {
            0 => Self::Static,
            1 => Self::RegularUpdates,
            2 => Self::IrregularUpdates,
            _ => Self::default(),
        }
    }
}

enum License {
    MIT,
    Apache,
    CcBy,
}

impl Default for License {
    fn default() -> Self {
        Self::MIT
    }
}

impl From<i32> for License {
    fn from(index: i32) -> Self {
        match index {
            0 => Self::MIT,
            1 => Self::Apache,
            2 => Self::CcBy,
            _ => Self::default(),
        }
    }
}
