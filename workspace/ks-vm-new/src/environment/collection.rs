use crate::types::StorageId;

#[derive(Debug, PartialEq)]
pub enum Collection {
    String(String),
    Stack(Vec<StorageId>),
    Free,
}
