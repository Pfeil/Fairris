use yew::prelude::*;

use super::DetailsPage;

pub struct EditButton {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub form_link: ComponentLink<DetailsPage>,
    pub edit_mode: bool,
}

pub enum Msg {
    Toggle,
}

impl Component for EditButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle => self.props.form_link.send_message(super::Msg::ToggleEditMode),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changes = self.props.edit_mode != props.edit_mode;
        self.props = props;
        changes
    }

    fn view(&self) -> Html {
        let (label, classes) = match self.props.edit_mode {
            false => ("Edit object", "edit-button"),
            true => ("Save changes in object", "ok-button"),
        };
        html! {
            <button class=classes onclick=self.link.callback(|_| Msg::Toggle)>{ label }</button>
        }
    }
}