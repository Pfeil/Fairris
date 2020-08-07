//! This module contains serializable and deserializable types
//! and conversions between types for communicating with other
//! services using JSON.

#[macro_use]
pub mod primitive_types;
pub mod pit_record;
pub mod types;

mod metadata;

pub use pit_record::PidRecord;
pub use metadata::*;

pub trait RecordEntry {
    /// Writes content of entry representation into the given record.
    fn write(&self, record: &mut PidRecord);
}