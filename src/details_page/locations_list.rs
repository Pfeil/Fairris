use yew::prelude::*;

use crate::data_type_registry::*;

use super::DetailsPage;

pub struct LocationsList {
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
    Value(Vec<String>),
    Error(String),
}

impl Component for LocationsList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Value(urls) => {
                self.props
                    .form_link
                    .send_message(super::Msg::LocationsChanged(urls));
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
                <label class="form-description" for=Locations::get_key_name()>{ Locations }</label>
                <textarea class="form-input" id=Locations::get_key_name() disabled=!self.props.active
                    onchange=self.link.callback(|e: ChangeData| match e {
                        ChangeData::Value(element) => {
                            let urls: Vec<String> = element.split("\n").map(|str| str.to_owned()).collect();
                            Msg::Value(urls)
                        },
                        other => Msg::Error(format!("Got unexpected: {:?}", other))
                    })
                />
            </>
        }
    }
}
