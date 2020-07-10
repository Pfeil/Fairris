use yew::prelude::*;

pub struct CreateComponent {}

impl Component for CreateComponent {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html!{
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
                // TODO how to enable this button?
                //<button class="okbutton" onclick=self.link.callback(|_| Msg::ButtonRegisterFDO)>{ "Create FDO Record" }</button>
            </div>
        }
    }
}