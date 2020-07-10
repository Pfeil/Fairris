use yew::prelude::*;

pub struct DetailsComponent {}

impl Component for DetailsComponent {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
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
                <div class="image-placeholder"><p>{ "IMAGE" }</p></div>
                <div class="textblock">
                    <p>{ "Print PID Record here." }</p>
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