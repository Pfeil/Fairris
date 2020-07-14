#![recursion_limit = "1024"]

mod create_component;
mod known_pids;
mod pidinfo;
mod search_component;

use std::rc::Rc;

use create_component::CreateComponent;
use known_pids::*;
use pidinfo::PidInfo;
use search_component::SearchComponent;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::{prelude::*, router::Router, Switch};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/create"]
    CreateFdo,
    #[to = "/fdo/{*:path}"]
    Details { path: String },
    #[to = "/search"]
    Search,
    #[to = "/"]
    Index,
}

pub struct Model {
    link: ComponentLink<Self>,
    known_pids: Rc<KnownPids>,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Msg {
    AddPidItem(PidInfo),
    Remove,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let known_pids: Rc<KnownPids> = Rc::new(KnownPids::default());

        Self { link, known_pids }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddPidItem(item) => {
                Rc::get_mut(&mut self.known_pids).unwrap_throw().insert(item.pid.clone(), item);
            },
            other => log::error!("Unimplemented message: {:?}", other),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        false
    }

    fn view(&self) -> Html {
        let known_pids = self.known_pids.clone();
        let model_link = self.link.clone();
        let router_function = move |switch: AppRoute| {
            match switch {
                AppRoute::CreateFdo => html! {<CreateComponent model_link=model_link.clone() />},
                AppRoute::Details { ref path } => {
                    //html!{}
                    log::info!("Got id: {}", path);
                    known_pids.find(path).view_as_details_page()
                }
                AppRoute::Search => html! {<SearchComponent/>},
                AppRoute::Index => html! {<CreateComponent model_link=model_link.clone() />},
            }
        };
        html! {
            <div id="everything">
                <div id="sidebar" class="maincolumns">
                    <div id="pidbuttons">
                        <RouterButton<AppRoute> route=AppRoute::CreateFdo>{ "+" }</RouterButton<AppRoute>>
                        <RouterButton<AppRoute> route=AppRoute::Search>{ "search" }</RouterButton<AppRoute>>
                        <button onclick=self.link.callback(|_| Msg::Remove)>{ "-" }</button>  // TODO this should create a callback to remove a pid.
                    </div>
                    <div id="workspace" class="scroll-vertical">
                        { for self.known_pids.iter().map(|(_pid, pidinfo)| pidinfo.view_as_list_item()) }
                    </div>
                </div>
                <Router<AppRoute, ()> render = Router::render(router_function)
                />
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    App::<Model>::new().mount_to_body();
}
