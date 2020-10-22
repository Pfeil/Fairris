use std::convert::TryFrom;

use enum_iterator::IntoEnumIterator;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

use crate::data_type_registry::{HasProfileKey, Pid, Profile};

use super::{DetailsPage, helpers::DOM};

pub struct ProfileSelector {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub active: bool,
    pub form_link: ComponentLink<DetailsPage>,
    pub maybe_profile: Result<Profile, Option<Pid>>,
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
        self.props = props.clone();
        let dropdown = DOM::get_element::<HtmlSelectElement, _>(Profile::get_key_name());
        // TODO the unused result warning should remember you to also display a missing or unknown type.
        props.maybe_profile.map(|profile| dropdown.set_value(&*Pid::from(&profile)));
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
                                let selected: bool = self.props.maybe_profile
                                    .as_ref()
                                    .map(|this| *this == p)
                                    .unwrap_or(false);
                                let pid = Pid::from(&p);
                                html! { <option value=pid selected=selected>{ p }</option> }
                            })
                    }
                </select>
            </>
        }
    }
}
