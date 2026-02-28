use std::collections::HashMap;

use crate::compiler_new::types::CollectionId;

use super::types::VariableId;

pub enum Collection {
    Primitive,
    Module {
        children: Vec<Option<CollectionId>>,
        indeces: HashMap<String, VariableId>,
    },
    List {
        child: CollectionId,
    },
    Tuple {
        children: Vec<CollectionId>,
    },
}
