#![recursion_limit = "1024"]

mod create_component;
mod details_page;
mod known_pids;
mod pidinfo;
mod search_component;
mod service_communication;
mod data_type_registry;

use std::{cell::RefCell, rc::Rc};

use create_component::CreateComponent;
use details_page::DetailsPage;
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
    UpdatePidItem(PidInfo),
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
            Msg::UpdatePidItem(item) => {
                self.known_pids.borrow_mut().find_mut(item.pid()).and_then(|found| Some(*found = item) );
            }
            Msg::Error(issue) => log::error!("Something went wrong: {}", issue),
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
        let router_function = move |switch: AppRoute| match switch {
            AppRoute::Details { ref path } => known_pids.borrow().find(path).map_or_else(
                || Self::view_record_not_found_page(path),
                |item| {
                    html! {<DetailsPage model_link=model_link.clone() record=item.clone() />}
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
    wasm_logger::init(wasm_logger::Config::default());
    App::<Model>::new().mount_to_body();
}
