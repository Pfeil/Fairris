use std::{cell::RefCell, convert::TryFrom, rc::Rc};

use yew::prelude::*;

use super::create_data_form::*;
use super::annotated_image_form::*;
use crate::{
    known_data::{Data, DataID, KnownData},
    DetailsPage,
};

pub struct DataWidget {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone, Debug)]
pub struct Props {
    pub model: ComponentLink<crate::Model>,
    pub detail_page: ComponentLink<DetailsPage>,
    pub data: Option<(DataID, Data)>,
    pub known_data: Rc<RefCell<KnownData>>,
}

#[derive(Debug)]
pub enum Msg {
    DataEmpty,
    DataValue(String),
    Error(String),
}

impl Component for DataWidget {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::debug!("Data Widget got message: {:?}", msg);
        match msg {
            Msg::Error(e) => log::error!("Message not handled: {}", e),
            Msg::DataEmpty => {
                self.props.data = None;
            }
            Msg::DataValue(value) => {
                if let Ok(id) = DataID::try_from(value) {
                    // TODO use yew-state
                    let data = self.props.known_data.borrow().get(&id).cloned();
                    if let Some(data) = data {
                        self.props.data = Some((id, data.clone()));
                        self.props
                            .detail_page
                            .send_message(crate::details_page::Msg::DataChanged(id, data.clone()));
                    }
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        log::debug!("New Props for Data Widget: {:?}", props);
        self.props = props;
        use crate::details_page::helpers::DOM;
        let dropdown = DOM::get_element::<web_sys::HtmlSelectElement, _>(DATA_CHOOSER_NAME);
        // TODO the unused result warning should remember you to also display a missing or unknown type.
        self.props.data.clone().map_or_else(
            || dropdown.set_value("new"),
            |(id, _data)| dropdown.set_value(id.to_string().as_str()),
        );
        true
    }

    fn view(&self) -> Html {
        log::debug!("Redraw Data Widget");
        let nothing_is_selected = self.props.data == None;
        html! {
            <details open=true>
                <summary>{ "Data" }</summary>
                <div class="two-column-lefty header">
                    <div class="stacking">
                        <label class="form-description" for=DATA_CHOOSER_NAME>{ "Create or reuse a data entry:" }</label>
                        <select class="form-input" id=DATA_CHOOSER_NAME
                            onchange=self.link.callback(|value: ChangeData| match value {
                                // e will be a data id or "new" as you can see in the code below.
                                ChangeData::Select(element) if element.value() == "new" => Msg::DataEmpty,
                                ChangeData::Select(element) => Msg::DataValue(element.value()),
                                other => Msg::Error("Unexpected value in selector.".into()),
                            })>
                            <option value="new" selected=nothing_is_selected>{ "Create new data entry/reference." }</option>
                            {
                                for self.props.known_data.borrow().iter()
                                    .map(|(dataid, data)| {
                                        let selected = self.props.data.clone().and_then(|(id, _data)| Some(*dataid == id)).unwrap_or(false);
                                        // TODO type name is not a good description.
                                        html! { <option value=dataid selected=selected>{ format!("{} - {}", **dataid, data.type_name()) }</option> }
                                    })
                            }
                        </select>
                    </div>

                    <p>{
                        r#"Here you reference and manage data. This represents the data that a real client would store locally,
                        i.e. data that was inserted into the client or the client produced.
                        You may reuse existing data references or create a new one."#
                    }</p>
                </div>

                {
                    match &self.props.data {
                        None => html!{<CreateData detail_page=self.props.detail_page.clone() model=self.props.model.clone()/>},
                        Some((id, Data::AnnotatedImage(image))) => html! {<AnnotatedImageForm id=id image=image detail_page=self.props.detail_page.clone() />},
                        _ => html! {<p>{"UI unimplemented."}</p>},
                    }
                }
            </details>
        }
    }
}

const DATA_CHOOSER_NAME: &str = "data_chooser";
