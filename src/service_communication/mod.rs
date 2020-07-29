//! This module contains serializable and deserializable types
//! and conversions between types for communicating with other
//! services using JSON.

#[macro_use]
pub mod primitive_types;
pub mod pit_record;
pub mod types;

use pit_record::PidRecord;

pub trait RecordEntry {
    /// Writes content of entry representation into the given record.
    fn write(&self, record: &mut PidRecord);
}