use std::convert::TryFrom;

use enum_iterator::IntoEnumIterator;
use yew::prelude::*;

use crate::data_type_registry::{HasProfileKey, Pid, Profile};

use super::DetailsPage;

pub struct ProfileSelector {
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

impl Component for ProfileSelector {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Value(value) => {
                let changed = Profile::try_from(&Pid(value));
                self.props
                    .form_link
                    .send_message(super::Msg::ProfileChanged(changed));
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
        let name = Profile::get_key_name();
        html! {
            <>
                <label class="form-description" for=name>{ name }</label>
                <select class="form-input" id=name required=true disabled=!self.props.active
                        onchange=self.link.callback(|e: ChangeData| match e {
                            ChangeData::Select(element) => Msg::Value(element.value()),
                            other => Msg::Error(format!("Got unexpected: {:?}", other))
                        })>
                    {
                        for Profile::into_enum_iter()
                            .map(|p: Profile| {
                                let pid = Pid::from(p);
                                html! { <option value=pid>{ p }</option> }
                            })
                    }
                </select>
            </>
        }
    }
}