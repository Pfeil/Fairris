use yew::prelude::*;
use super::Model;

#[derive(Eq, PartialEq)]
pub struct PidInfo {
    pid: String,
    description: String,
    status: String,
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

impl PidInfo {
    pub fn to_html(&self) -> Html {
        html! {
            <div class="piditem">
                <p>{ self.pid.clone() }</p>
                <p>{ self.description.clone() }</p>
                <p>{ self.status.clone() }</p>
            </div>
        }
    }
}

impl Model {
    //pub fn change_route(&self, app_route: AppRoute) -> Callback<MouseEvent> {
    //    self.link.callback(move |_| {
    //        let route = app_route.clone();
    //        Msg::ChangeRoute(route)
    //    })
    //}
}