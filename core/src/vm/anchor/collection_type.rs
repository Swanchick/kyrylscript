use std::collections::HashMap;

pub enum CollectionType {
    List(Vec<u64>),
    Module(HashMap<String, u64>),
}