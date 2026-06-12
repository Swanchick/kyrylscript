use super::types::{CollectionId, StorageId};

#[derive(Debug, PartialEq)]
pub enum Assign {
    Variable(StorageId),
    Collection(CollectionId, usize),
    None,
}
