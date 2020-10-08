use yew::prelude::*;

use crate::data_type_registry::*;

use super::DetailsPage;

pub struct VersionInput {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub active: bool,
    pub form_link: ComponentLink<DetailsPage>,
}

#[derive(Debug)]
pub enum Msg {
    Value(String),
    Error(String),
}

impl Component for VersionInput {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Value(versionstring) => {
                self.props
                    .form_link
                    .send_message(super::Msg::VersionChanged(versionstring));
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
        let name = Version::get_key_name();
        html! {
            <>
                <label class="form-description" for=name>{ name }</label>
                <input class="form-input" id=name disabled=!self.props.active
                    onchange=self.link.callback(|e: ChangeData| match e {
                        ChangeData::Value(versionstring) => {
                            Msg::Value(versionstring)
                        },
                        other => Msg::Error(format!("Got unexpected: {:?}", other))
                    })
                />
            </>
        }
    }
}