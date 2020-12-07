#![recursion_limit = "1024"]

#[macro_use]
extern crate strum;

mod app_state;
mod pidinfo;
mod pidinfo_viewer;

mod details_page;
mod search_component;

mod service_communication;
mod data_type_registry;

mod pit_service;
mod collection_service;

use std::collections::HashMap;

use app_state::{data_manager::DataManager, pid_manager::PidManager};
use data_type_registry::Pid;
use details_page::DetailsPage;
use pidinfo::PidInfo;
use search_component::SearchComponent;
use pidinfo_viewer::PidInfoView;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::{prelude::*, router::Router, Switch};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/fdo/{*:path}"]
    Details { path: String },
    #[to = "/search"]
    Search,
    #[to = "/"]
    Index,
}

pub struct Model {
    link: ComponentLink<Self>,

    pid_manager: Box<dyn Bridge<PidManager>>,
    // keeping a bridge to the data state here keeps it alive.
    _data_manager: Box<dyn Bridge<DataManager>>,

    known_pids: HashMap<Pid, PidInfo>,
}

#[derive(Debug)]
pub enum Msg {
    AddDefaultItem,
    PidAdd(PidInfo),  // overwrites if object with this pid exists
    PidRemove(Pid),  // object will be removed

    UpdatePidInfoList(HashMap<Pid, PidInfo>),
    Error(String),
    Noop,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        use crate::app_state::pid_manager::Incoming;
        let mut pid_manager = PidManager::bridge(link.callback(|msg| {
            match msg {
                app_state::pid_manager::Outgoing::AllPidInformation(infos) => Msg::UpdatePidInfoList(infos),
            }
        }));
        pid_manager.send(Incoming::GetAllPidInformation);
        let _data_manager = DataManager::bridge(link.callback(|_msg| Msg::Noop));
        Self { link, pid_manager, _data_manager, known_pids: Default::default() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use crate::app_state::pid_manager::Incoming;

        log::debug!("Model received update {:?}", msg);
        match msg {
            Msg::Error(issue) => {
                log::error!("Something went wrong: {}", issue);
                false
            },
            Msg::UpdatePidInfoList(list) => {
                self.known_pids = list;
                true
            },

            Msg::AddDefaultItem => {
                self.pid_manager.send(Incoming::AddUnregisteredItem);
                true
            }
            Msg::PidAdd(item) => {
                self.pid_manager.send( Incoming::AddPidInfo(item) );
                true
            },
            Msg::PidRemove(pid) => {
                self.pid_manager.send( Incoming::RemovePidInfo(pid) );
                true
            }
            Msg::Noop => false,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        true
    }

    fn view(&self) -> Html {
        let known_pids = self.known_pids.clone();
        let router_function = move |switch: AppRoute| match switch {
            AppRoute::Details { ref path } => {
                let pid = Pid(path.to_string());
                known_pids.get(&pid).map_or_else(
                    || Self::view_record_not_found_page(path),
                    |item| {
                        html! {<DetailsPage record=item.clone() />}
                    }
                )
            },
            AppRoute::Search => html! {<SearchComponent/>},
            AppRoute::Index => Self::view_welcome_page(),
        };
        html! {
            <div id="everything">
                <div id="sidebar" class="maincolumns">
                    <div id="pidbuttons">
                        <button onclick=self.link.callback(|_| Msg::AddDefaultItem)>{ "Add" }</button>
                        <RouterButton<AppRoute> route=AppRoute::Search>{ "Search" }</RouterButton<AppRoute>>
                    </div>
                    <div id="workspace" class="scroll-vertical">
                        {
                            for self.known_pids.iter().map(|(_pid, pidinfo)| {
                                html!{ <PidInfoView model_link=self.link.clone() record=pidinfo /> }
                            })
                        }
                    </div>
                </div>
                <Router<AppRoute, ()> render = Router::render(router_function)
                />
            </div>
        }
    }
}

impl Model {
    fn view_record_not_found_page(pid: &String) -> Html {
        // TODO extend and style
        html! {<p>{format!("Object {} not locally available.", pid)}</p>}
    }

    fn view_welcome_page() -> Html {
        // TODO extend and style
        // TODO Add understandable introduction, tell the user that he may play around.
        html! {
            <div>
            <h1>{"Welcome to Fairris"}</h1>
            <p>{"
                Fairris show the FAIR digital object ecosystem (the testbed) from a user perspective. 
                Imagine this user interface to be a digital lab notebook, workflow system or IDE, depending on your needs. 
                It will help you to manage and register your research data and may automate a lot of your work.
            "}</p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    App::<Model>::new().mount_to_body();
}
