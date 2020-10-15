use std::convert::TryFrom;

use yew::prelude::*;

use crate::data_type_registry::{DateModified, DateTimeHandle, HasProfileKey};

use super::DetailsPage;

pub struct DateModifiedInput {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub active: bool,
    pub form_link: ComponentLink<DetailsPage>,
    pub date_modified: DateModified,
}

#[derive(Debug)]
pub enum Msg {
    Value(String),
    Error(String),
}

impl Component for DateModifiedInput {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Value(string) => {
                let handle = DateTimeHandle::try_from(&string);
                if let Ok(date) = handle {
                    self.props
                        .form_link
                        .send_message(super::Msg::DateModifiedChanged(DateModified(date)));
                } else {
                    // TODO handle wrong input
                    log::error!("Could not parse date.");
                }
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
        let name = DateModified::get_key_name();
        let content = self.props.date_modified.0.to_string();
        html! {
            <>
                <label class="form-description" for=name>{ name }</label>
                <input class="form-input" id=name disabled=!self.props.active value=content
                    onchange=self.link.callback(|e: ChangeData| match e {
                        ChangeData::Value(datestring) => {
                            Msg::Value(datestring)
                        },
                        other => Msg::Error(format!("Got unexpected: {:?}", other))
                    })
                />
            </>
        }
    }
}
