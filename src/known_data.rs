use std::ops::{Deref, DerefMut};
use std::{collections::HashMap, convert::TryFrom};

use super::PidInfo;
use crate::collection_service::collection::Collection;

use rand::prelude::*;
use strum::IntoEnumIterator;

#[derive(Default, Clone, PartialEq)]
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

    pub fn find_data_for_record(&self, item: &PidInfo) -> Option<(DataID, Data)> {
        if let Some(id) = &item.data {
            log::debug!("Found some data in record: {:?}", id);
            self.deref().get(&id).map(|data| (id.clone(), data.clone()))
        } else {
            // TODO here you may handle retrieving data out of the raw record inside the pidinfo
            //      (map data info to data objects, send GET request to collection service, etc)
            log::warn!("Did not find existing data. Implement creating data here (maybe).");
            None
        }
    }

    pub fn get_descriptions(&self) -> Vec<(DataID, String)> {
        self.deref()
            .iter()
            .map(|(id, data)| (id.clone(), data.type_name()))
            .collect()
    }
}

impl Default for Data {
    fn default() -> Self {
        Data::AnnotatedImage(AnnotatedImage::default())
    }
}

impl TryFrom<String> for DataID {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse().map(|ok| Self(ok)).map_err(|_error| ())
    }
}

impl TryFrom<&String> for Data {
    type Error = ();

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Data::iter()
            .find(|d: &Data| d.type_name() == *value)
            .ok_or(())
    }
}

impl std::fmt::Debug for KnownData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KnownData(...)")
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
