use super::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Eq, PartialEq, Clone, Debug)]
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

impl PidInfo {
    pub fn view_as_list_item(&self) -> Html {
        let pid = self.pid.clone();
        html! {
                // TODO make sure that in the details page, the actual data is shown.
                <RouterButton<AppRoute> route=AppRoute::Details{path: pid} classes="piditem">
                    <p>{ self.pid.clone() }</p>
                    <p>{ self.description.clone() }</p>
                    <p>{ self.status.clone() }</p>
                </RouterButton<AppRoute>>
        }
    }

    pub fn view_as_details_page(&self) -> Html {
        let pid = self.pid.clone();
        html! {
            <div id="content" class="maincolumns scroll-vertical">
                <div class="image-placeholder"><p>{ "IMAGE" }</p></div>
                <div class="textblock">
                    <p>{ format!("{}: {}", "PID:", pid) }</p>
                    <p>{ "Print PID Record here." }</p>
                    <p>{ "Print PID Record here." }</p>
                    <p>{ "Print PID Record here." }</p>
                    <p>{ "Print PID Record here." }</p>
                    <p>{ "Print PID Record here." }</p>
                    <p>{ "Print PID Record here." }</p>
                    <p>{ "Print PID Record here." }</p>
                </div>
                <div class="fdo-actions"><p>{ "Placeholder for Action Buttons here." }</p></div>
                <div class="action-placeholder"><p>{ "Placeholder for action visualization. This could be i.e. viewing raw metadata, visualizations, or the possibility to update your FDO." }</p></div>
            </div>
        }
    }
}
