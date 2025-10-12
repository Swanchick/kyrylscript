pub enum TreeReference<'a> {
    Branch(&'a [u64], usize),
    Leaf,
}
