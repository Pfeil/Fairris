use std::{
    convert::TryFrom,
    ops::{Deref, DerefMut},
};

use crate::collection_service::collection::Collection;
use crate::strum::IntoEnumIterator;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct DataID(pub u16);

#[derive(Debug, PartialEq, Eq, Clone, EnumIter)]
pub enum Data {
    AnnotatedImage(AnnotatedImage),
    Collection(Collection),
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct AnnotatedImage {
    pub url: String,
    pub annotation_urls: Vec<String>,
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
