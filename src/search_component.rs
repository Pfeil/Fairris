use yew::prelude::*;

pub struct SearchComponent {}

impl Component for SearchComponent {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html!{
            <div id="content" class="maincolumns scroll-vertical">
                <p>{ "Not implemented yet. This page would offer you to search for data with given properties." }</p>
            </div>
        }
    }
}