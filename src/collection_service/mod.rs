pub mod collection;
pub mod collection_capabilities;
pub mod collection_properties;
pub mod member_item;

use anyhow::Error;
use serde_json::Value;
use yew::{
    format::Json,
    services::fetch::{FetchTask, Request, Response},
    services::FetchService,
    Callback, ComponentLink,
};

use crate::Model;
use collection::*;
use collection_capabilities::*;
use collection_properties::*;

pub struct CollectionService {
    task: Option<FetchTask>,
}

impl CollectionService {
    pub fn new() -> Self {
        Self {
            task: None,
        }
    }

    fn send_request(
        &mut self,
        collections: Collections,
        callback: Callback<Response<Result<String, Error>>>,
    ) {
        let collections = collections.into();
        let request = Self::create_collections_request(&collections);
        self.task = FetchService::fetch(request, callback)
            .map_err(|e| log::error!("Error creating task to register metadata: {}", e))
            .ok();
    }

    fn create_collections_request(collections: &serde_json::Value) -> Request<Json<&Value>> {
        Request::post(Self::get_create_collections_uri())
            .header("Content-Type", "application/json")
            .body(Json(collections))
            .expect("Failed to build this request.")
    }

    // TODO make configurable
    fn get_base_uri() -> &'static str {
        "http://localhost:8091"
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
