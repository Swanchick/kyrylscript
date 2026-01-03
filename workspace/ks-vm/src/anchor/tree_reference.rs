use std::collections::HashMap;

use crate::environment::Reference;

pub enum TreeReference<'a> {
    Branch(&'a [Reference], usize),
    ModuleBranch(&'a HashMap<String, Reference>, usize),
    Leaf,
}
