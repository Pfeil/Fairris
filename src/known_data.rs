use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use super::{Model, PidInfo};
use crate::{
    collection_service::collection::Collection, service_communication::primitive_types::Pid,
};

use rand::prelude::*;
use yew::prelude::*;
use strum::IntoEnumIterator;

#[derive(Default)]
pub struct KnownData {
    known_data: HashMap<DataID, Data>,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct DataID(pub u16);

#[derive(Debug, PartialEq, Eq, Clone, EnumIter)]
pub enum Data {
    AnnotatedImage(AnnotatedImage),
    Collection(Collection),
}

impl Data {
    pub fn has_same_datatype_like(&self, other: &Self) -> bool {
        match (self, other) {
            (Data::AnnotatedImage(_), Data::AnnotatedImage(_)) => true,
            (Data::Collection(_), Data::Collection(_)) => true,

            (Data::AnnotatedImage(_), Data::Collection(_)) => false,
            (Data::Collection(_), Data::AnnotatedImage(_)) => false,
        }
    }

    pub fn type_name(&self) -> String {
        match self {
            Data::AnnotatedImage(_) => "Annotated Image".into(),
            Data::Collection(_) => "Collection".into(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct AnnotatedImage {
    pub url: String,
    pub annotation_urls: Vec<String>,
}

impl KnownData {
    pub fn add(&mut self, data: Data) -> DataID {
        let id: DataID;
        loop {
            let maybe_pid = DataID(rand::thread_rng().gen::<u16>());
            if !self.known_data.contains_key(&maybe_pid) {
                id = maybe_pid;
                break;
            }
        }
        self.known_data.insert(id, data);
        id
    }
}

impl Default for Data {
    fn default() -> Self {
        Data::AnnotatedImage(AnnotatedImage::default())
    }
}

// === Deref and DerefMut for types in this module ===

impl Deref for DataID {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DataID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for KnownData {
    type Target = HashMap<DataID, Data>;

    fn deref(&self) -> &Self::Target {
        &self.known_data
    }
}

impl DerefMut for KnownData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.known_data
    }
}
