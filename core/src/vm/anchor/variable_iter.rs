use std::collections::HashMap;

pub enum VariableIter<'a> {
    Collection(&'a [u64], usize),
    Module(&'a HashMap<String, u64>, usize),
    Leaf,
}
