use yew::prelude::*;

use super::DetailsPage;

pub struct FormSwitch {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub active: bool,
    pub form_link: ComponentLink<DetailsPage>,
}

#[derive(Debug)]
pub enum FormType {
    Collection = 0,
    Data = 1,
    Raw = 2,
}

#[derive(Debug)]
pub enum Msg {
    Value(String),
    Error(String),
}

impl Component for FormSwitch {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Value(value) => {
                let change: FormType = value.into();
                self.props.form_link.send_message(super::Msg::FormTypeChanged(change));
            }
            other => log::error!("Message not handled: {:?}", other),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <label class="form-description" for="profile-select">{ "Profile:" }</label>
                <select class="form-input" id="profile-select" required=true disabled=!self.props.active
                        onchange=self.link.callback(|e: ChangeData| match e {
                            ChangeData::Select(element) => Msg::Value(element.value()),
                            other => Msg::Error(format!("Got unexpected: {:?}", other))
                        })>
                    <option value="collection">{ "Collection" }</option>
                    <option value="data">{ "Data" }</option>
                    <option value="raw">{ "Raw Profile Form" }</option>
                </select>
            </>
        }
    }
}

impl From<String> for FormType {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "collection" => FormType::Collection,
            "data" => FormType::Data,
            "raw" => FormType::Raw,
            _ => {
                log::error!("Got unexpected form: {}", s);
                FormType::Raw
            },
        }
    }
}