use super::types::{CollectionId, StorageId};

#[derive(Debug)]
pub enum Assign {
    Variable(StorageId),
    Collection(CollectionId, usize),
    None,
}
