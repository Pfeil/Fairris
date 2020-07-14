#![recursion_limit = "1024"]

mod create_component;
mod model_impl;
mod pidinfo;
mod search_component;

use pidinfo::PidInfo;
use create_component::CreateComponent;
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
    #[to = "/fdo/{id}"]
    Details(String),
    #[to = "/search"]
    Search,
    #[to = "/"]
    Index,
}

struct Model {
    link: ComponentLink<Self>,
    known_pids: Vec<PidInfo>,
}

#[derive(Eq, PartialEq)]
enum Msg {
    Remove,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut known_pids = vec![
            PidInfo::default(),
            PidInfo::default(),
            PidInfo::default(),
            PidInfo::default(),
        ];
        known_pids.iter_mut().enumerate().for_each(|(num, info)| {
            info.pid = format!("{}_{}", info.pid, num);
        });
        Self { link, known_pids }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        //match msg {
        //    Msg::ChangeView(view) => self.view = view,
        //    _ => (),
        //}
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        false
    }

    fn view(&self) -> Html {
        let router_function = |switch: AppRoute| {
            match switch {
                AppRoute::CreateFdo => html!{<CreateComponent/>},
                AppRoute::Details(ref pid) => {
                    //html!{}
                    self.pidinfo_as_details_page(pid)  // create html within the model
                    //self.find_pidinfo_by_string(pid.as_str()).view_as_details_page()  // create html within the pitinfo
                },
                AppRoute::Search => html!{<SearchComponent/>},
                AppRoute::Index => html!{<CreateComponent/>},
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
                        { for self.known_pids.iter().map(|pidinfo| pidinfo.view_as_list_item()) }
                        // { self.pidinfo_as_details_page(&"kitdm/test/1234567890_1".into()) }  // TODO Just for testing. It works using it in here.
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
