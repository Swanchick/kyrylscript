use crate::types::StorageId;

#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[derive(Debug, PartialEq)]
pub enum Collection {
    String(String),
    Stack(Vec<StorageId>),
    Free,
}
