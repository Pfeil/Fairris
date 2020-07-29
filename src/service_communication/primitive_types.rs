//! This module contains datatypes and profiles as defined in the registry.
use chrono::prelude::*;
use std::ops::{Deref, DerefMut};

/// This macro will shorten boilerplate to implement Deref and DerefMut for simple Newtype patterns.
/// deref() will resolve to &self.0
/// deref_mut() will resolve to &mut self.0
/// I recommend using *self instead of calling .deref() or .deref_mut() implicitly.
macro_rules! newtype_deref {
    ($name:ty, $target:ty) => {
        impl Deref for $name {
            type Target = $target;
        
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        
        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }       
    };
}

pub type Pid = String;

/// A Pid embedded into a URL which will resolve it.
/// Represents http://dtr-test.pidconsortium.net/#objects/21.T11148/8bcd7b4a8a9c74402c71
// TODO Could ba an alias for a URL type instead, which does not exist yet.
pub type PidProxy = String;

#[derive(Debug)]
pub enum Profile {
    RecommendedKernelProfile,
    HmcKernelProfile,
}

impl Default for Profile {
    fn default() -> Self {
        Self::RecommendedKernelProfile
    }
}

impl From<i32> for Profile {
    fn from(index: i32) -> Self {
        match index {
            0 => Self::RecommendedKernelProfile,
            1 => Self::HmcKernelProfile,
            _ => Self::default(),
        }
    }
}

pub enum DataType {
    Tiff,
    Png,
}

impl Default for DataType {
    fn default() -> Self {
        Self::Tiff
    }
}

impl From<i32> for DataType {
    fn from(index: i32) -> Self {
        match index {
            0 => Self::Tiff,
            1 => Self::Png,
            _ => Self::default(),
        }
    }
}

#[derive(Default)]
pub struct Policy {
    pub lifecycle: Lifecycle,
    pub license: License,
    pub tombstone: Option<String>,
}

pub enum Lifecycle {
    Static,
    RegularUpdates,
    IrregularUpdates,
}

impl Default for Lifecycle {
    fn default() -> Self {
        Self::Static
    }
}

impl From<i32> for Lifecycle {
    fn from(index: i32) -> Self {
        match index {
            0 => Self::Static,
            1 => Self::RegularUpdates,
            2 => Self::IrregularUpdates,
            _ => Self::default(),
        }
    }
}

pub enum License {
    MIT,
    Apache,
    CcBy,
}

impl Default for License {
    fn default() -> Self {
        Self::MIT
    }
}

impl From<i32> for License {
    fn from(index: i32) -> Self {
        match index {
            0 => Self::MIT,
            1 => Self::Apache,
            2 => Self::CcBy,
            _ => Self::default(),
        }
    }
}

pub enum HashAlgorithm {
    Sha256sum,
}

impl Default for HashAlgorithm {
    fn default() -> Self {
        Self::Sha256sum
    }
}

pub struct Checksum {
    pub algorithm: HashAlgorithm,
    pub value: String,
}

impl Default for Checksum {
    fn default() -> Self {
        Checksum {
            algorithm: Default::default(),
            value: "c50624fd5ddd2b9652b72e2d2eabcb31a54b777718ab6fb7e44b582c20239a7c".into(),
        }
    }
}

/// Represents a URL.
/// TODO Change name to URL or something similar.
pub struct ObjectLocation(String);

impl Default for ObjectLocation {
    fn default() -> Self {
        // TODO this default is for testing only. It should be empty and the component should handle the case of an invalid url properly.
        ObjectLocation("https://example.com/image.tiff".into())
    }
}

newtype_deref!(ObjectLocation, String);

pub struct DateTimeHandle(DateTime<Utc>);

impl Default for DateTimeHandle {
    fn default() -> Self {
        // TODO this default is for testing only. It should be empty and the component should handle the case of an invalid url properly.
        DateTimeHandle(Utc::now())
    }
}

newtype_deref!(DateTimeHandle, DateTime<Utc>);
