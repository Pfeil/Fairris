#![recursion_limit = "1024"]

#[macro_use]
extern crate strum;

mod known_pids;
mod known_data;
mod pidinfo;
mod service_communication;

mod details_page;
mod search_component;

mod pit_service;
mod collection_service;
mod data_type_registry;

use std::{ops::Deref, cell::RefCell, rc::Rc};

use data_type_registry::Pid;
use details_page::DetailsPage;
use known_pids::*;
use known_data::*;
use pidinfo::PidInfo;
use search_component::SearchComponent;
use pit_service::PitService;

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
    known_pids: Rc<RefCell<KnownPids>>,
    known_data: Rc<RefCell<KnownData>>,
    pit_service: PitService,
}

#[derive(Debug)]
pub enum Msg {
    AddDefaultItem,
    PidAdd(PidInfo),  // overwrites if object with this pid exists
    PidReplace(Pid, PidInfo),  // object with pid will be removed, new one will be added
    PidRemove(String),  // object will be removed

    DataAdd(Data, Pid),  // overwrites if object with this pid exists
    DataRemove(DataID),  // object will be removed

    RegisterFDO(PidInfo),
    UpdateFDO(PidInfo),

    Error(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let known_pids: Rc<RefCell<KnownPids>> = Rc::new(RefCell::new(KnownPids::default()));
        let known_data: Rc<RefCell<KnownData>> = Rc::new(RefCell::new(KnownData::default()));
        let pit_service = PitService::new(link.clone());
        Self { link, known_pids, known_data, pit_service }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::debug!("Model received update {:?}", msg);
        match msg {
            Msg::Error(issue) => log::error!("Something went wrong: {}", issue),

            Msg::AddDefaultItem => {
                self.known_pids.borrow_mut().add_unregistered(self.link.clone());
            }
            Msg::PidAdd(item) => {
                self.known_pids.borrow_mut().insert(item.pid().clone(), item);
            },
            Msg::PidRemove(pid) => {
                self.known_pids.borrow_mut().remove(&pid);
            }
            Msg::PidReplace(pid, record) => {
                self.link.send_message(Msg::PidAdd(record));
                self.link.send_message(Msg::PidRemove(pid.deref().clone()))
            },

            Msg::RegisterFDO(mut record) => self.pit_service.register_pidinfo(&mut record),
            Msg::UpdateFDO(mut record) => self.pit_service.update_pidinfo(&mut record),
            
            Msg::DataAdd(data, pid) => {
                log::debug!("Adding data {:?} to record {:?}", data, pid);
                let id = self.known_data.borrow_mut().add(data.clone());
                if let Some(record) = self.known_pids.borrow_mut().find_mut(&pid) {
                    record.data = Some(id);
                }
            }
            Msg::DataRemove(id) => {
                self.known_data.borrow_mut().remove(&id);
            }
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
        let known_data = self.known_data.clone();
        let model_link = self.link.clone();
        let router_function = move |switch: AppRoute| match switch {
            AppRoute::Details { ref path } => known_pids.borrow().find(path).map_or_else(
                || Self::view_record_not_found_page(path),
                |item| {
                    let data = known_data.borrow().find_data_for_record(&item);
                    html! {<DetailsPage model_link=model_link.clone() record=item.clone() current_data=data known_data=known_data.clone() />}
                },
            ),
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
                        { for self.known_pids.borrow().iter().map(|(_pid, pidinfo)| pidinfo.view_as_list_item()) }
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
