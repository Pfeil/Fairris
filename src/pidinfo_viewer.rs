use yew::{Component, ComponentLink, prelude::*};
use yew_router::prelude::*;

use crate::{Model, AppRoute, Msg, data_type_registry::Pid, pidinfo::PidInfo};


pub struct PidInfoView {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub model_link: ComponentLink<Model>,
    pub record: PidInfo,
}

impl Component for PidInfoView {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> yew::ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> yew::Html {
        let pid = self.props.record.pid().clone();
        let pid2 = Pid(pid.clone());
        html! {
                <div class="piditem">
                <RouterButton<AppRoute> route=AppRoute::Details{path: pid.clone()} classes="fdo-button">
                    <p>{ pid.as_str() }</p>
                    <p>{ self.props.record.describe() }</p>
                    <p>{ format!("{:?}", self.props.record.state()) }</p>
                </RouterButton<AppRoute>>
                <button onclick=self.props.model_link.callback( move |_| Msg::PidRemove(pid2.clone()) ) class="fdo-remove-button">{"✗"}</button>
                </div>
        }
    }
}