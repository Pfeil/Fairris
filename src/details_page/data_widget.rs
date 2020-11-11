use std::convert::TryFrom;

use yew::prelude::*;
use strum::IntoEnumIterator;

use crate::{data_type_registry::{DigitalObjectType, Pid}, known_data::{AnnotatedImage, Data, DataID}};

use super::DetailsPage;

pub struct DataWidget {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub data: Option<(DataID, Data)>,
    pub data_descriptions: Vec<(DataID, String)>,
}

#[derive(Debug)]
pub enum Msg {
    ChangedDataType(String),
    Error(String),
}

impl Component for DataWidget {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Error(e) => log::error!("Message not handled: {}", e),
            Msg::ChangedDataType(t) => {
                // TODO change data object accordingly!
            },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <details open=true>
                <summary>{ "Data" }</summary>
                <div class="two-column-lefty header">
                    <div class="stacking">
                        <label class="form-description" for="data_chooser">{ "Create or reuse a data entry:" }</label>
                        <select class="form-input" id="data_chooser"
                            onchange=self.link.callback(|e: ChangeData| {Msg::Error("Unimplemented".into())})>
                        <option value="new">{ "Create new data entry/reference." }</option>
                            {
                                for self.props.data_descriptions.iter()
                                    .map(|(id, description): &(DataID, String)| {
                                        html! { <option value=id>{ description }</option> }
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
