use crate::environment::Reference;

use std::collections::HashMap;

pub enum VariableIter<'a> {
    Collection(&'a [Reference], usize),
    Module(&'a HashMap<String, Reference>, usize),
    Leaf,
}
