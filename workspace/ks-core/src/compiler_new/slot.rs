use super::types::{CollectionId, VariableId};

#[derive(Debug)]
pub enum Slot {
    Variable(VariableId),
    Collection {
        variable_id: VariableId,
        collection_id: CollectionId,
    },
}

impl Slot {
    pub fn variable_id(&self) -> &VariableId {
        match self {
            Self::Variable(variable_id)
            | Self::Collection {
                variable_id,
                collection_id: _,
            } => variable_id,
        }
    }
}
