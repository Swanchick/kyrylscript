pub struct VarInfo {
    reference: u64,
    depth: usize,
}

impl VarInfo {
    pub fn reference(&self) -> &u64 {
        &self.reference
    }

    pub fn depth(&self) -> &usize {
        &self.depth
    }
}