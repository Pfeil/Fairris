use std::convert::TryFrom;

use yew::prelude::*;
use strum::IntoEnumIterator;

use crate::{known_data::Data, details_page::DetailsPage};

pub struct CreateData {
    link: ComponentLink<Self>,
    props: Props,

    datatype: String,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub detail_page: ComponentLink<DetailsPage>,
    pub model: ComponentLink<crate::Model>,
}

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
        Self { link, props, datatype: Data::default().type_name() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Datatype(stringname) => self.datatype = stringname,
            Msg::ButtonClick => {
                Data::try_from(&self.datatype).and_then(|d| {
                    self.props.detail_page.send_message(crate::details_page::Msg::DataNew(d.clone()));
                    Ok(())
                })
                .map_err(|_| {
                    log::error!("Could not parse a data entry from {}", &self.datatype);
                });
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
        html! {
            <div class="two-column-lefty">
                <div class="stacking">
                    <label class="form-description" for=label_name>{ "Type of your data:" }</label>
                    <select class="form-input" id=label_name
                            onchange=self.link.callback(|e: ChangeData| match e {
                                ChangeData::Select(element) => Msg::Datatype(element.value()),
                                other => Msg::Error(format!("Got unexpected: {:?}", other)),
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

                <button class="ok-button" onclick=self.link.callback(|_| Msg::ButtonClick)>
                    { format!("Create {}", self.datatype) }
                </button>
            </div>
        }
    }
}
