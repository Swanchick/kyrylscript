use std::collections::HashMap;

pub enum TreeReference<'a> {
    Branch(&'a [u64], usize),
    ModuleBranch(&'a HashMap<String, u64>, usize),
    Leaf,
}
