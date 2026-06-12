use super::types::{CollectionId, Slot};

#[derive(Debug, PartialEq)]
pub enum Assign {
    Variable(Slot),
    Collection(CollectionId, usize),
    None,
}
