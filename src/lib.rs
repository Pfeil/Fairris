#![recursion_limit = "1024"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;
// use yew_router::{route::Route, service::RouteService, Switch};

mod model_impl;
use model_impl::*;

// #[derive(Switch, Debug)]
// pub enum AppRoute {
//     #[to = "/profile/{id}"]
//     Profile(u32),
//     #[to = "/forum{*:rest}"]
//     Forum(ForumRoute),
//     #[to = "/"]
//     Index,
// }

struct Model {
    link: ComponentLink<Self>,
    known_pids: Vec<PidInfo>,
    view: View,
}

#[derive(Eq, PartialEq)]
enum Msg {
    ChangeView(View),
    Remove,
    ButtonRegisterFDO,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            known_pids: vec![
                PidInfo::default(),
                PidInfo::default(),
                PidInfo::default(),
                PidInfo::default(),
            ],
            view: View::RegisterFdo,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeView(view) => self.view = view,
            _ => (),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="everything">
                <div id="sidebar" class="maincolumns">
                    <div id="pidbuttons">
                        <button onclick=self.link.callback(|_| Msg::ChangeView(View::RegisterFdo))>{ "+" }</button>
                        <button onclick=self.link.callback(|_| Msg::ChangeView(View::Search))>{ "search" }</button>
                        <button onclick=self.link.callback(|_| Msg::Remove)>{ "-" }</button>
                    </div>
                    <div id="workspace" class="scroll-vertical">
                        { for self.known_pids.iter().map(|pidinfo| pidinfo.to_html()) }  // TODO pitinfo should obviously be a component, probably with a link to Model.
                    </div>
                </div>
                { self.view_to_html() }
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
