use yew::prelude::*;

use crate::data_type_registry::{HasProfileKey, Locations};

use super::DetailsPage;

pub struct LocationsList {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub active: bool,
    pub form_link: ComponentLink<DetailsPage>,
    pub locations: Locations,
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
                    .send_message(super::Msg::LocationsChanged(Locations(urls)));
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
        let name = Locations::get_key_name();
        let content = self.props.locations.0.join("\n");
        html! {
            <>
                <label class="form-description" for=name>{ name }</label>
                <textarea class="form-input" id=name disabled=!self.props.active value=content
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
