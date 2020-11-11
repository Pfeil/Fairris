use yew::prelude::*;

use super::DetailsPage;
use crate::pidinfo::State;

pub struct PublishButton {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub form_link: ComponentLink<DetailsPage>,
    pub edit_mode: bool,
    pub state: State,
}

pub enum Msg {
    Clicked,
}

impl Component for PublishButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => self.props.form_link.send_message(super::Msg::Publish),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let has_changes = self.props != props;
        if has_changes { self.props = props; }
        has_changes
    }

    fn view(&self) -> Html {
        if self.props.edit_mode {
            html! {}
        } else {
            let (label, classes) = match self.props.state {
                State::Unregistered => ("Publish object", "publish-button"),
                State::Modified => ("Publish object changes", "update-button"),
                State::Clean => ("Object state is published", "inactive-button"),
            };
            let is_disabled = self.props.state == State::Clean;
            html! {
                <button class=classes disabled=is_disabled
                    onclick=self.link.callback(|_| Msg::Clicked)>{ label }</button>
            }
        }
    }
}

impl PartialEq for Props {
    fn eq(&self, other: &Self) -> bool {
        self.edit_mode == other.edit_mode && self.state == other.state
    }
}
