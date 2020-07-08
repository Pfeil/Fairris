#![recursion_limit="1024"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct PidInfo {
    pid: String,
    description: String,
    status: String,
}

impl Default for PidInfo {
    fn default() -> Self {
        PidInfo {
            pid: "kitdm/test/1234567890".into(),
            description: "Placeholder description or object name.".into(),
            status: "downloaded, inactive".into(),
        }
    }
}

impl PidInfo {
    fn to_html(&self) -> Html {
        html! {
            <div class="piditem">
                <p>{ self.pid.clone() }</p>
                <p>{ self.description.clone() }</p>
                <p>{ self.status.clone() }</p>
            </div>
        }
    }
}

struct Model {
    link: ComponentLink<Self>,
    known_pids: Vec<PidInfo>,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            known_pids: vec![PidInfo::default(), PidInfo::default(), PidInfo::default(), PidInfo::default()],
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        //match msg {
        //    Msg::AddOne => self.value += 1
        //}
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="everything">
                <div id="sidebar" class="maincolumns">
                    <div id="pidbuttons">
                        <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+" }</button>
                        <button onclick=self.link.callback(|_| Msg::AddOne)>{ "search" }</button>
                        <button onclick=self.link.callback(|_| Msg::AddOne)>{ "-" }</button>
                    </div>
                    <div id="workspace" class="scroll-vertical">
                        { for self.known_pids.iter().map(|pidinfo| pidinfo.to_html()) }
                    </div>
                </div>
                <div id="content" class="maincolumns scroll-vertical">
                    <div class="column-form">
                        <label class="form-description" for="profile-select">{ "Profile:" }</label>
                        <select class="form-input" id="profile-select">
                            <option>{ "HMC Kernel Information Profile" }</option>
                            <option>{ "Metadata Document" }</option>
                        </select>

                        <label class="form-description" for="fdo-type">{ "Digital Object Data Type:" }</label>
                        <select class="form-input" id="fdo-type">
                            <option>{ "image/TIFF (media-type-IANA-image)" }</option>
                            <option>{ "image/PNG (media-type-IANA-image)" }</option>
                        </select>

                        <label class="form-description" for="fdo-data-url">{ "Digital Object Data URL (or Handle?):" }</label>
                        <input class="form-input" type="url" id="fdo-data-url" required=true />

                        <label class="form-description" for="fdo-lifecycle">{ "Lifecycle:" }</label>
                        <select class="form-input" id="fdo-lifecycle">
                            <option>{ "static" }</option>
                            <option>{ "dynamic and regular updates" }</option>
                            <option>{ "dynamic and irregular updates" }</option>
                        </select>

                        <label class="form-description" for="fdo-license">{ "License:" }</label>
                        <select class="form-input" id="fdo-license">
                            <option>{ "MIT" }</option>
                            <option>{ "Apache" }</option>
                            <option>{ "CC-by" }</option>
                        </select>

                        <label class="form-description" for="fdo-etag">{ "etag (object hash):" }</label>
                        <select class="form-input" id="fdo-etag">
                            <option>{ "Calculate from Location Body" }</option>
                            <option>{ "Provide manually (TODO create-text-unput on selection)" }</option>
                            <option>{ "What if the location is a stream?" }</option>
                        </select>

                        <p>{ "Assumption: Date of creation/modification is done by the pit/pid-service" }</p>
                        <p>{ "No input here, therefore." }</p>

                        <label class="form-description" for="fdo-version">{ "Object Version String:" }</label>
                        <input class="form-input" type="text" id="fdo-version" required=true />

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
                    <button class="okbutton" onclick=self.link.callback(|_| Msg::AddOne)>{ "Create FDO Record" }</button>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}