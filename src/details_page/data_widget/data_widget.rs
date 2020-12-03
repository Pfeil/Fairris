use std::{collections::HashMap, convert::TryFrom};

use yew::prelude::*;

use super::annotated_image_form::*;
use super::collection_form::*;
use super::create_data_form::*;
use crate::{DetailsPage, app_state::{data::{Data, DataID}, data_manager::DataManager}};

pub struct DataWidget {
    link: ComponentLink<Self>,
    props: Props,

    data_manager: Box<dyn Bridge<DataManager>>,

    data: Option<(DataID, Data)>,
    data_list: HashMap<DataID, Data>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub model: ComponentLink<crate::Model>,
    pub detail_page: ComponentLink<DetailsPage>,
}

#[derive(Debug)]
pub enum Msg {
    SetDataList(HashMap<DataID, Data>),
    SetData(Option<(DataID, Data)>),
    DataEmpty,
    DataSelect(String),
    Error(String),
}

impl Component for DataWidget {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        use crate::app_state::data_manager::{Incoming, Outgoing};
        let mut data_manager = DataManager::bridge(link.callback(|msg| match msg {
            Outgoing::SelectedData(d) => Msg::SetData(d),
            Outgoing::AllData(v) => Msg::SetDataList(v),
        }));
        data_manager.send(Incoming::GetAllData);
        Self { link, props, data_manager, data: None, data_list: Default::default() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use crate::app_state::data_manager::Incoming;
        log::debug!("Data Widget got message: {:?}", msg);
        match msg {
            Msg::DataEmpty => {
                self.data_manager.send(Incoming::SelectDataId(None));
            }
            Msg::DataSelect(value) => {
                // called when a new data object was selected
                if let Ok(id) = DataID::try_from(value) {
                    self.data_manager.send(Incoming::SelectDataId(Some(id)));
                }
            }
            Msg::SetData(d) => {
                self.data = d;
                self.update_dropdown();

                self.data_manager.send(Incoming::GetAllData);
            },
            Msg::Error(e) => log::error!("Message not handled: {}", e),
            Msg::SetDataList(v) => self.data_list = v,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;

        use crate::app_state::data_manager::Incoming;
        self.data_manager.send(Incoming::GetAllData);
        
        self.update_dropdown();
        true
    }

    fn view(&self) -> Html {
        log::debug!("Redraw Data Widget");
        let nothing_is_selected = self.data == None;
        let on_selection = self.link.callback(|value: ChangeData| match value {
            // e will be a data id or "new" as you can see in the code below.
            ChangeData::Select(element) if element.value() == "new" => Msg::DataEmpty,
            ChangeData::Select(element) => Msg::DataSelect(element.value()),
            other => Msg::Error(format!("Unexpected value in selector: {:?}", other)),
        });
        let data_list = self.data_list.iter()
            .map(|(dataid, data)| {
                let selected = self.data.clone().and_then(|(id, _data)| Some(*dataid == id)).unwrap_or(false);
                // TODO type name is not a good description.
                html! { <option value=dataid selected=selected>{ format!("{} - {}", **dataid, data.type_name()) }</option> }
            });
        let content_form = match &self.data {
            None => html! {<CreateData/>},
            Some((id, Data::AnnotatedImage(image))) => html! {<AnnotatedImageForm id=id image=image />},
            Some((id, Data::Collection(collection))) => html! {<CollectionForm id=id collection=collection />},
        };
        html! {
            <details open=true>
                <summary>{ "Data" }</summary>
                <div class="two-column-lefty header">
                    <div class="stacking">
                        <label class="form-description" for=DATA_CHOOSER_NAME>{ "Create or reuse a data entry:" }</label>
                        <select class="form-input" id=DATA_CHOOSER_NAME
                            onchange=on_selection>
                            <option value="new" selected=nothing_is_selected>{ "Create new data entry/reference." }</option>
                            { for data_list }
                        </select>
                    </div>

                    <p>{
                        r#"Here you reference and manage data. This represents the data that a real client would store locally,
                        i.e. data that was inserted into the client or the client produced.
                        You may reuse existing data references or create a new one."#
                    }</p>
                </div>

                { content_form }

            </details>
        }
    }
}

const DATA_CHOOSER_NAME: &str = "data_chooser";

impl DataWidget {
    fn update_dropdown(&mut self) {
        use crate::details_page::helpers::DOM;
        let dropdown = DOM::get_element::<web_sys::HtmlSelectElement, _>(DATA_CHOOSER_NAME);
        // TODO the unused result warning should remember you to also display a missing or unknown type.
        self.data.clone().map_or_else(
            || dropdown.set_value("new"),
            |(id, _data)| dropdown.set_value(id.to_string().as_str()),
        );
    }
}