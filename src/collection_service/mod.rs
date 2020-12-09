pub mod collection;
pub mod collection_capabilities;
pub mod collection_properties;
pub mod member_item;

use std::collections::HashSet;

use anyhow::Error;
use serde_json::Value;
use yew::{prelude::*, Callback, agent::Dispatcher, format::Json, services::FetchService, services::fetch, services::fetch::FetchTask, worker::*};

use crate::app_state::{data::{Data, DataID}, data_manager::DataManager};
use collection::*;

pub type Etag = http::header::HeaderValue;
pub type MaybeEtag = Option<Etag>;

pub struct CollectionService {
    link: AgentLink<CollectionService>,
    subscribers: HashSet<HandlerId>,

    task: Option<FetchTask>,
    data_manager: Dispatcher<DataManager>,
}

#[derive(Debug)]
pub enum Request {
    Register(Vec<(DataID, Collection)>),
    Push(DataID, Collection, Etag),
    Pull(DataID, Collection),
}

#[derive(Debug, Clone)]
pub enum Response {
    Registered(Vec<(DataID, Collection)>),
    Pushed(DataID, Collection, MaybeEtag),
    Pulled(DataID, Collection, MaybeEtag),
    Error(String),
}

impl Agent for CollectionService {
    type Reach = Context<Self>;
    type Message = Response;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
            task: None,
            data_manager: DataManager::dispatcher(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        use crate::app_state::data_manager::Incoming as DataMsg;
        let msg = match msg.clone() {
            Response::Registered(collections) => {
                for (id, coll) in collections.iter() {
                    self.data_manager.send(DataMsg::UpdateData(*id, Data::Collection(coll.clone())));
                }
                Some(Response::Registered(collections))
            }
            Response::Pushed(id, coll, etag) => {
                self.data_manager.send(DataMsg::UpdateData(id, Data::Collection(coll.clone())));
                Some(Response::Pushed(id, coll, etag))
            }
            Response::Pulled(id, coll, etag) => {
                self.data_manager.send(DataMsg::UpdateData(id, Data::Collection(coll.clone())));
                Some(Response::Pulled(id, coll, etag))
            }
            Response::Error(msg) => {
                log::error!("Collection Service Error: {}", msg);
                None
            },
        };
        if let Some(msg) = msg {
            for sub in self.subscribers.iter() {
                self.link
                    .respond(*sub, msg.clone());
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, from: HandlerId) {
        log::debug!("{:?} -> CollectionService: {:?}", from, msg);
        match msg {
            Request::Register(collections) => {
                let (ids, collections) = collections.into_iter().fold(
                    (Vec::new(), Vec::new()),
                    |(mut ids, mut colls), (id, coll)| {
                        ids.push(id);
                        colls.push(coll);
                        (ids, colls)
                    },
                );
                self.register_collections(ids, collections.into());
            }
            Request::Push(id, coll, etag) => {
                self.push_collection(id, coll, etag);
            }
            Request::Pull(id, coll) => {
                self.pull_collection(id, coll);
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}

impl CollectionService {
    pub fn register_collections(&mut self, ids: Vec<DataID>, collections: Collections) {
        let callback = self.link.callback(
            move |http_response: fetch::Response<Result<String, Error>>| {
                if http_response.status().is_success() {
                    let thing: serde_json::Result<Vec<Collection>> = serde_json::from_str(
                        http_response
                            .body()
                            .as_ref()
                            .expect("Get reference from body")
                            .as_str(),
                    );
                    if let Ok(collections) = thing {
                        let packed = ids
                            .clone()
                            .into_iter()
                            .zip(collections.into_iter())
                            .collect();
                            Response::Registered(packed)
                    } else {
                        Response::Error(format!(
                            "Could not parse collections from collection service: {:?}",
                            thing
                        ))
                    }
                } else {
                    Response::Error(
                        "Response of collection service was not successful.".to_string(),
                    )
                }
            },
        );
        let collections = &collections.into();
        let request = Self::create_collections_request(&collections);
        self.send_json_request(request, callback);
    }

    pub fn push_collection(&mut self, id: DataID, collection: Collection, etag: Etag) {
        let callback = self.link.callback(
            move |http_response: fetch::Response<Result<String, Error>>| {
                if http_response.status().is_success() {
                    let thing: serde_json::Result<Collection> = serde_json::from_str(
                        http_response
                            .body()
                            .as_ref()
                            .expect("Get reference from body")
                            .as_str(),
                    );
                    let etag = http_response.headers().get("ETag").cloned();
                    if let Ok(collection) = thing {
                        Response::Pushed(id, collection, etag)
                    } else {
                        Response::Error(format!(
                            "Could not parse collections from collection service: {:?}",
                            thing
                        ))
                    }
                } else {
                    Response::Error(
                        "Response of collection service was not successful.".to_string(),
                    )
                }
            },
        );
        let collection_id = collection.get_id().map(|id| id.to_owned());
        if let Some(id) = collection_id {
            let collection = serde_json::to_value(collection).expect("Could not serialize collection.");
            let request = Self::push_collection_request(id.as_str(), &collection, etag);
            self.send_json_request(request, callback);
        } else {
            log::error!("Tried to push (update) a collection without id (not registered).")
        }
    }

    pub fn pull_collection(&mut self, id: DataID, collection: Collection) {
        let callback = self.link.callback(
            move |http_response: fetch::Response<Result<String, Error>>| {
                if http_response.status().is_success() {
                    let thing: serde_json::Result<Collection> = serde_json::from_str(
                        http_response
                            .body()
                            .as_ref()
                            .expect("Get reference from body")
                            .as_str(),
                    );
                    let etag = http_response.headers().get("ETag").cloned();
                    if let Ok(collection) = thing {
                        Response::Pulled(id, collection, etag)
                    } else {
                        Response::Error(format!(
                            "Could not parse collections from collection service: {:?}",
                            thing
                        ))
                    }
                } else {
                    Response::Error(
                        "Response of collection service was not successful.".to_string(),
                    )
                }
            },
        );
        let collection_id = collection.get_id().map(|id| id.to_owned());
        if let Some(id) = collection_id {
            let request = Self::pull_collection_request(id.as_str());
            self.send_empty_request(request, callback);
        } else {
            log::error!("Tried to pull (download) a collection without id (not registered).")
        }
    }

    fn send_json_request(
        &mut self,
        request: fetch::Request<Json<&Value>>,
        callback: Callback<fetch::Response<Result<String, Error>>>,
    ) {
        self.task = FetchService::fetch(request, callback)
            .map_err(|e| log::error!("Error creating task to register metadata: {}", e))
            .ok();
    }

    fn send_empty_request(
        &mut self,
        request: fetch::Request<yew::format::Nothing>,
        callback: Callback<fetch::Response<Result<String, Error>>>,
    ) {
        self.task = FetchService::fetch(request, callback)
            .map_err(|e| log::error!("Error creating task to register metadata: {}", e))
            .ok();
    }

    fn create_collections_request(collections: &serde_json::Value) -> fetch::Request<Json<&Value>> {
        fetch::Request::post(Self::get_create_collections_uri())
            .header("Content-Type", "application/json")
            .body(Json(collections))
            .expect("Failed to build this request.")
    }

    fn push_collection_request<'a>(id: &str, collection: &'a serde_json::Value, etag: Etag) -> fetch::Request<Json<&'a Value>> {
        fetch::Request::put(Self::get_collection_uri(id))
            .header("Content-Type", "application/json")
            .header("IF-Match", etag)
            .body(Json(collection))
            .expect("Failed to build this request.")
    }

    fn pull_collection_request(id: &str) -> fetch::Request<yew::format::Nothing> {
        fetch::Request::get(Self::get_collection_uri(id))
            //.header("Content-Type", "application/json")
            .body(yew::format::nothing::Nothing)
            .expect("Failed to build this request.")
    }

    // TODO make configurable
    fn get_base_uri() -> &'static str {
        "http://localhost:8091/api/v1"
    }

    /// POST: create one or more collections.
    /// GET: get collections within this API.
    fn get_create_collections_uri() -> String {
        format!("{}/{}/", Self::get_base_uri(), "collections")
    }

    /// GET: get the collection
    /// PUT: update the collections properties
    /// DELETE: delete the collection
    fn get_collection_uri(id: &str) -> String {
        format!("{}/{}/{}", Self::get_base_uri(), "collections", id)
    }
}

impl From<Collections> for serde_json::Value {
    fn from(cs: Collections) -> Self {
        serde_json::Value::Array(
            cs.iter()
                .map(|c| serde_json::to_value(c))
                .filter(|result| {
                    if let Ok(_) = result {
                        true
                    } else {
                        log::error!("Own collection could not be serialized: {:?}", result);
                        false
                    }
                })
                .map(|ok| ok.unwrap())
                .collect(),
        )
    }
}
