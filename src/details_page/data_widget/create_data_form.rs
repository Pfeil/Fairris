use std::convert::TryFrom;

use strum::IntoEnumIterator;
use yew::{agent::Dispatcher, prelude::*};

use crate::app_state::{data::Data, data_manager::{DataManager, Incoming}};

pub struct CreateData {
    link: ComponentLink<Self>,
    props: Props,

    data_manager: Dispatcher<DataManager>,

    datatype: String,
}

#[derive(Properties, Clone)]
pub struct Props {}

#[derive(Debug)]
pub enum Msg {
    Datatype(String),
    ButtonClick,
    Error(String),
}

impl Component for CreateData {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            data_manager: DataManager::dispatcher(),
            datatype: Data::default().type_name(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Datatype(stringname) => self.datatype = stringname,
            Msg::ButtonClick => {
                let maybe_data = Data::try_from(&self.datatype).ok();
                self.data_manager.send( Incoming::AddAndSelectData(maybe_data.clone()));
                if let None = maybe_data {
                    log::error!("Could not parse a data entry from {}", &self.datatype);
                };
            }
            Msg::Error(e) => log::error!("Error: {}", e),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let label_name = "type_selection";
        let on_entry_selection_change = self.link.callback(|e: ChangeData| match e {
            ChangeData::Select(element) => Msg::Datatype(element.value()),
            other => Msg::Error(format!("Got unexpected: {:?}", other)),
        });
        let datatype_list = Data::iter().map(|data: Data| {
            let value = data.type_name();
            html! { <option value=value>{ value }</option> }
        });

        html! {
            <div class="two-column-lefty">
                <div class="stacking">
                    <label class="form-description" for=label_name>{ "Type of your data:" }</label>
                    <select class="form-input" id=label_name onchange=on_entry_selection_change>
                        { for datatype_list }
                    </select>
                </div>

                <button class="ok-button" onclick=self.link.callback(|_| Msg::ButtonClick)>
                    { format!("Create {}", self.datatype) }
                </button>
            </div>
        }
    }
}
