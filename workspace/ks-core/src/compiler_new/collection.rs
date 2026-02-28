use std::collections::HashMap;

use crate::compiler_new::types::CollectionId;

use super::types::VariableId;

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
