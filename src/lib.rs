#![recursion_limit = "1024"]

mod create_component;
mod known_pids;
mod pidinfo;
mod search_component;
mod service_communication;

use std::{cell::RefCell, rc::Rc};

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
    #[to = "/createObject"]
    CreateFdo,
    #[to = "/createCollection"]
    CreateCollection,
    #[to = "/fdo/{*:path}"]
    Details { path: String },
    #[to = "/search"]
    Search,
    #[to = "/"]
    Index,
}

pub struct Model {
    link: ComponentLink<Self>,
    known_pids: Rc<RefCell<KnownPids>>,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Msg {
    AddDefaultItem,
    AddPidItem(PidInfo),
    Remove(String),
    Error(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let known_pids: Rc<RefCell<KnownPids>> = Rc::new(RefCell::new(KnownPids::default()));
        Self { link, known_pids }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::debug!("Model received update.");
        match msg {
            Msg::AddPidItem(item) => {
                log::debug!("Adding new item: {:?}", item);
                self.known_pids.borrow_mut().insert(item.pid().clone(), item);
            },
            Msg::Remove(pid) => {
                log::debug!("Removing item: {}", pid);
                self.known_pids.borrow_mut().remove(&pid);
            }
            Msg::AddDefaultItem => {
                self.known_pids.borrow_mut().add_unregistered(self.link.clone());
            }
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
                AppRoute::CreateCollection => html! {},
                AppRoute::Details { ref path } => {
                    known_pids.borrow().find(path)
                        .map_or_else(
                            || html!{<p>{"Object not locally available."}</p>},
                            |item| item.view_as_details_page()
                        )
                }
                AppRoute::Search => html! {<SearchComponent/>},
                AppRoute::Index => html! {<CreateComponent model_link=model_link.clone() />},
            }
        };
        html! {
            <div id="everything">
                <div id="sidebar" class="maincolumns">
                    <div id="pidbuttons">
                        <button onclick=self.link.callback(|_| Msg::AddDefaultItem)>{ "Add" }</button>
                        <RouterButton<AppRoute> route=AppRoute::Search>{ "Search" }</RouterButton<AppRoute>>
                    </div>
                    <div id="workspace" class="scroll-vertical">
                        { for self.known_pids.borrow().iter().map(|(_pid, pidinfo)| pidinfo.view_as_list_item()) }
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
