use yew::{agent::Dispatcher, prelude::*};
use yewtil::NeqAssign;

use crate::{
    app_state::{data::DataID, data_manager, data_manager::DataManager},
    collection_service::{self, Etag, MaybeEtag},
    collection_service::{collection::Collection, CollectionService},
};

pub struct CollectionForm {
    link: ComponentLink<Self>,
    props: Props,

    collection_service: Box<dyn Bridge<CollectionService>>,
    data_manager: Dispatcher<DataManager>,

    etag: Option<(DataID, Etag)>,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub id: DataID,
    pub collection: Collection,
}

#[derive(Debug)]
pub enum Msg {
    DescriptionChanged(String),
    // PropertiesChanged as sub-component?
    // CapabilitiesChanged as sub-component?
    PublishClicked,
    Update(DataID, Collection, MaybeEtag),
    Error(String),
}

impl Component for CollectionForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let collection_service =
            CollectionService::bridge(link.callback(|response| match response {
                collection_service::Response::Registered(collections) => {
                    Msg::Error("Unimplemented".into())
                }
                collection_service::Response::Pushed(id, coll, etag) => Msg::Update(id, coll, etag),
                collection_service::Response::Pulled(id, coll, etag) => Msg::Update(id, coll, etag),
                collection_service::Response::Error(e) => Msg::Error(e),
            }));

        let mut myself = Self {
            link,
            props,
            collection_service,
            data_manager: DataManager::dispatcher(),
            etag: None,
        };
        myself.pull_collection();
        myself
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
                if self.props.collection.get_id().is_none() {
                    // register collection
                    self.collection_service
                        .send(crate::collection_service::Request::Register(vec![(
                            id,
                            self.props.collection.clone(),
                        )]));
                } else {
                    // update collection
                    let collection = self.props.collection.clone();
                    let etag = self.etag.clone();
                    if let Some((etag_id, etag)) = etag {
                        if etag_id != id {
                            log::error!("Tried to update collection without a correct etag!")
                        } else {
                            self.collection_service
                                .send(crate::collection_service::Request::Push(
                                    id, collection, etag,
                                ));
                            self.etag = None;
                        }
                    } else {
                        log::error!("Tried to update without etag!");
                    }
                }
                self.update_data();
            }
            Msg::Error(e) => log::error!("Error: {}", e),
            Msg::Update(id, collection, etag) => {
                if self.props.id == id {
                    if let Some(etag) = etag {
                        self.etag = Some((id, etag));
                        self.props.collection = collection
                    } else {
                        log::error!("Did not receive an etag! Current is: {:?}", self.etag);
                    }
                } else {
                    log::error!(
                        "Tried to assign an etag for another id! Current: {:?}, replacement: {:?}",
                        self.etag,
                        etag
                    );
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed_id = self.props.id != props.id;
        //let just_registered =
        //    self.props.collection.get_id().is_none() && props.collection.get_id().is_some();
        let etag_exists = self
            .etag
            .clone()
            .map_or(false, |(id, _)| id == self.props.id);
        let changed = self.props.neq_assign(props);
        if changed_id || !etag_exists {
            self.pull_collection();
        }
        changed
    }

    fn view(&self) -> Html {
        let id = self
            .props
            .collection
            .get_id()
            .unwrap_or("No id yet (collection not registered)");
        let description = self
            .props
            .collection
            .description
            .as_ref()
            .map(|desc| desc.as_str())
            .unwrap_or("");
        let on_description_changed = self.link.callback(|c: ChangeData| match c {
            ChangeData::Value(description) => Msg::DescriptionChanged(description),
            other => Msg::Error(format!("Unexpected change: {:?}", other)),
        });
        let properties: String = self
            .props
            .collection
            .properties
            .as_ref()
            .map_or("No properties yet".to_owned(), |properties| {
                serde_json::to_string_pretty(&properties).unwrap()
            });
        let capabilities: String = self
            .props
            .collection
            .capabilities
            .as_ref()
            .map_or("No capabilities yet".to_owned(), |capabilities| {
                serde_json::to_string_pretty(&capabilities).unwrap()
            });
        let (button_text, button_classes) = if self.props.collection.get_id().is_none() {
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

impl CollectionForm {
    fn update_data(&mut self) {
        use crate::app_state::data::Data;
        let data = Data::Collection(self.props.collection.clone());
        let id = self.props.id;
        //self.props
        //    .detail_page
        //    .send_message(crate::details_page::Msg::DataChanged(id, data))
        self.data_manager
            .send(data_manager::Incoming::UpdateData(id, data));
    }

    fn pull_collection(&mut self) {
        let is_registered = self.props.collection.get_id().is_some();
        if is_registered {
            let id = self.props.id;
            let coll = self.props.collection.clone();
            use collection_service::Request as CollMsg;
            self.collection_service.send(CollMsg::Pull(id, coll));
        }
    }
}
