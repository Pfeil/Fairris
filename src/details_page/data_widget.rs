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
        let name = "type_selection";
        let (current_id, current_data) = self.props.data.clone().unwrap_or((DataID(0), Data::default()));
        html! {
            <details open=true>
                <summary>{ "Data" }</summary>
                <label class="form-description" for=name>{ "Type of your data:" }</label>
                <select class="form-input" id=name
                        onchange=self.link.callback(|e: ChangeData| match e {
                            ChangeData::Select(element) => Msg::ChangedDataType(element.value()),
                            other => Msg::Error(format!("Got unexpected: {:?}", other))
                        })>
                    {
                        for Data::iter()
                            .map(|data: Data| {
                                let value = data.type_name();
                                let selected = data.has_same_datatype_like(&current_data);
                                html! { <option value=value selected=selected>{ value }</option> }
                            })
                    }
                </select>
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
        html! {
            <>
            <button class="ok-button">
                { "CREATE" }
            </button>
            <select class="form-input" id="tmp"
                    onchange=self.link.callback(|e: ChangeData| {Msg::Error("Unimplemented".into())})>
                {
                    for self.props.data_descriptions.iter()
                        .map(|(value, description): &(DataID, String)| {
                            html! { <option value=value>{ description }</option> }
                        })
                }
            </select>
            </>
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
