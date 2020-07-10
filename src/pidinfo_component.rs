use yew::prelude::*;
use super::AppRoute;
use yew_router::prelude::*;


#[derive(Eq, PartialEq)]
pub struct PidInfo {
    pub pid: String,
    pub description: String,
    pub status: String,
}

impl Default for PidInfo {
    fn default() -> Self {
        PidInfo {
            pid: "kitdm/test/1234567890".into(),
            description: "Placeholder description or object name.".into(),
            status: "downloaded, inactive".into(),
        }
    }
}

impl Component for PidInfo {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self::default()
    }
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
            //<div class="piditem">
                <RouterButton<AppRoute> route=AppRoute::Details(self.pid.clone()) classes="piditem">
                    <p>{ self.pid.clone() }</p>
                    <p>{ self.description.clone() }</p>
                    <p>{ self.status.clone() }</p>
                </RouterButton<AppRoute>>
            //</div>
        }
    }
}
