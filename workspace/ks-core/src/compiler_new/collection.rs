use std::collections::HashMap;

use super::types::{CollectionId, VariableId};

pub enum Collection {
    Module {
        children: Vec<Option<CollectionId>>,
        indeces: HashMap<String, VariableId>,
    },
    List {
        child: Option<CollectionId>,
    },
    Tuple {
        children: Vec<Option<CollectionId>>,
    },
}
