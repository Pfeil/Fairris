use yew::prelude::*;

use crate::{collection_service::collection::Collection, details_page::DetailsPage, known_data::{Data, DataID}};

pub struct CollectionForm {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id: DataID,
    pub collection: Collection,
    pub detail_page: ComponentLink<DetailsPage>,
    pub model: ComponentLink<crate::Model>,
}

#[derive(Debug)]
pub enum Msg {
    DescriptionChanged(String),
    // PropertiesChanged as sub-component?
    // CapabilitiesChanged as sub-component?
    PublishClicked,
    Error(String),
}

impl Component for CollectionForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DescriptionChanged(description) => {
                if description.is_empty() {
                    self.props.collection.description = None;
                } else {
                    self.props.collection.description = Some(description);
                }
                self.update_data();
            }
            Msg::PublishClicked => {
                let id = self.props.id.clone();
                if self.props.collection.get_id().is_empty() {
                    self.props.model.send_message(crate::Msg::DataRegister(id))
                } else {
                    self.props.model.send_message(crate::Msg::DataUpdate(id))
                }
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
        let mut id = self.props.collection.get_id();
        if id.is_empty() {
            id = "No id yet (collection not registered)"
        }
        let id = id;
        let description = if let Some(d) = &self.props.collection.description {
            d.as_str()
        } else {
            ""
        };
        let on_description_changed = self.link.callback(|c: ChangeData| match c {
            ChangeData::Value(description) => Msg::DescriptionChanged(description),
            other => Msg::Error(format!("Unexpected change: {:?}", other)),
        });
        let properties: String = self.props.collection.properties.as_ref().map_or(
            "No properties yet".to_owned(),
            |properties| serde_json::to_string_pretty(&properties).unwrap()
        );
        let capabilities: String = self.props.collection.capabilities.as_ref().map_or(
            "No capabilities yet".to_owned(),
            |capabilities| serde_json::to_string_pretty(&capabilities).unwrap()
        );
        let (button_text, button_classes) = if self.props.collection.get_id().is_empty() {
            ("Register collection", "publish-button")
        } else {
            ("Update collection", "update-button")
        };
        let on_button_click = self.link.callback(|_| Msg::PublishClicked);
        html! {
            <>
            <div class="two-column-lefty">
                <label class="form-description">{ "ID within collection API" }</label>
                <p class="form-input">{ id }</p>
                <label class="form-description" for=DESCRIPTION_FIELD>{ "Description" }</label>
                <textarea class="form-input" id=DESCRIPTION_FIELD value=description onchange=on_description_changed />
                <label class="form-description">{ "Properties" }</label>
                <p class="form-input">{ properties }</p>
                <label class="form-description">{ "Capabilities" }</label>
                <p class="form-input">{ capabilities }</p>
            </div>
            <div class="column-form">
                <button class=button_classes onclick=on_button_click>{ button_text }</button>
            </div>
            </>
        }
    }
}

const DESCRIPTION_FIELD: &str = "description_field";
const ANNOS_FIELD_NAME: &str = "annos_field";

impl CollectionForm {
    fn update_data(&self) {
        let data = Data::Collection(self.props.collection.clone());
        let id = self.props.id;
        self.props.detail_page.send_message(crate::details_page::Msg::DataChanged(id, data))
    }
}