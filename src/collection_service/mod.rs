pub mod collection;
pub mod collection_capabilities;
pub mod collection_properties;
pub mod member_item;

use std::collections::HashSet;

use anyhow::Error;
use serde_json::Value;
use yew::{
    format::Json, services::fetch, services::fetch::FetchTask, services::FetchService, worker::*,
    Callback,
};

use crate::app_state::data::DataID;
use collection::*;

pub struct CollectionService {
    link: AgentLink<CollectionService>,
    subscribers: HashSet<HandlerId>,
    task: Option<FetchTask>,
}

pub enum Request {
    Register(Vec<(DataID, Collection)>),
    Update(Vec<(DataID, Collection)>),
}

pub enum Response {
    Registered(Vec<(DataID, Collection)>),
    Updated(Vec<(DataID, Collection)>),
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
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Response::Registered(collections) => {
                for sub in self.subscribers.iter() {
                    self.link
                        .respond(*sub, Response::Registered(collections.clone()));
                }
            }
            Response::Updated(collections) => {
                for sub in self.subscribers.iter() {
                    self.link
                        .respond(*sub, Response::Updated(collections.clone()));
                }
            }
            Response::Error(msg) => log::error!("Collection Service Error: {}", msg),
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _from: HandlerId) {
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
            Request::Update(collections) => {
                // TODO self.update_collection(id, coll);
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
                    let thing: Result<Vec<Collection>, _> = serde_json::from_str(
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
        self.send_request(collections, callback);
    }

    fn send_request(
        &mut self,
        collections: Collections,
        callback: Callback<fetch::Response<Result<String, Error>>>,
    ) {
        let collections = collections.into();
        let request = Self::create_collections_request(&collections);
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

    // TODO make configurable
    fn get_base_uri() -> &'static str {
        "http://localhost:8091/api/v1"
    }

    /// Post here to create one or more collections.
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
