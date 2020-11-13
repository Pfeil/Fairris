use std::{rc::Rc, cell::RefCell, convert::TryFrom};

use yew::prelude::*;
use strum::IntoEnumIterator;

use crate::{known_data::{Data, DataID, KnownData}};

use super::DetailsPage;

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
    ChangedDataType(String),

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
            Msg::ChangedDataType(t) => {
                // TODO change data object accordingly!
            },
            Msg::DataEmpty => {
                self.props.data = None;
            },
            Msg::DataValue(value) => {
                if let Ok(id) = DataID::try_from(value) {
                    let data = self.props.known_data.borrow().get(&id).cloned();
                    data.and_then(|d| {
                        self.props.data = Some((id, d.clone()));
                        self.props.detail_page.send_message(super::Msg::DataChanged(id, d.clone()));
                        Some(())
                    });
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        log::debug!("New Props for Data Widget: {:?}", props);
        self.props = props;
        let dropdown = super::helpers::DOM::get_element::<web_sys::HtmlSelectElement, _>(DATA_CHOOSER_NAME);
        // TODO the unused result warning should remember you to also display a missing or unknown type.
        self.props.data.clone().map_or_else(
            || dropdown.set_value("new"),
            |(id, _data)| dropdown.set_value(id.to_string().as_str())
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
                    match self.props.data {
                        None => self.view_no_data_ui(),
                        _ => html! {<p>{"TODO"}</p>},
                    }
                }
            </details>
        }
    }
}



impl DataWidget {
    fn view_no_data_ui(&self) -> Html {
        // new or select existing.
        // TODO if record is Clean (or modified), make sure the data is extracted from the record. (probably not here!)
        let label_name = "type_selection";
        html! {
            <div class="two-column-lefty">
                <div class="stacking">
                    <label class="form-description" for=label_name>{ "Type of your data:" }</label>
                    <select class="form-input" id=label_name
                            onchange=self.link.callback(|e: ChangeData| match e {
                                ChangeData::Select(element) => Msg::ChangedDataType(element.value()),
                                other => Msg::Error(format!("Got unexpected: {:?}", other))
                            })>
                        {
                            for Data::iter()
                                .map(|data: Data| {
                                    let value = data.type_name();
                                    html! { <option value=value>{ value }</option> }
                                })
                        }
                    </select>
                </div>

                <button class="ok-button">
                    { "CREATE" }
                </button>
            </div>
        }
    }
    fn view_collection_ui(&self) -> Html {
        match &self.props.data {
            Some((id, data)) => {
                html! {

                }
            }
            None => {
                html! {
                    <button class="ok-button">
                        { "CREATE" }
                    </button>
                }
            }
        }
    }

    fn view_annotated_image_ui(&self) -> Html {
        let name = "Image URL";
        let (id, content, annotations) =
            match self.props.data.clone() {
                Some((id, Data::AnnotatedImage(data))) => (id, data.url, data.annotation_urls),
                None => (DataID(0), "".into(), vec!["".into()]),
                _ => (DataID(0), "error?".into(), vec!["error?".into()])
            };

        html! {
            <>
                <label class="form-description" for=name>{ name }</label>
                <input class="form-input" id=name value=content
                    onchange=self.link.callback(|e: ChangeData| match e {
                        other => Msg::Error(format!("Got unexpected: {:?}", other))
                    })
                />
            </>
        }
    }
}

const DATA_CHOOSER_NAME: &str = "data_chooser";