use yew::prelude::*;

use crate::data_type_registry::{Etag, HasProfileKey};

use super::DetailsPage;

pub struct EtagInput {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub active: bool,
    pub form_link: ComponentLink<DetailsPage>,
    pub etag: Etag,
}

#[derive(Debug)]
pub enum Msg {
    Value(String),
    Error(String),
}

impl Component for EtagInput {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Value(etag) => {
                self.props
                    .form_link
                    .send_message(super::Msg::EtagChanged(Etag(etag)));
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
        let name = Etag::get_key_name();
        let content = self.props.etag.0.as_str();
        html! {
            <>
                <label class="form-description" for=name>{ name }</label>
                <input class="form-input" id=name disabled=!self.props.active value=content
                    onchange=self.link.callback(|e: ChangeData| match e {
                        ChangeData::Value(etag) => {
                            Msg::Value(etag)
                        },
                        other => Msg::Error(format!("Got unexpected: {:?}", other))
                    })
                />
            </>
        }
    }
}
