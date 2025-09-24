pub struct ReferenceFrame {
    pub reference: u64,
    pub index: usize,
    pub new_references: Vec<u64>
}

impl ReferenceFrame {
    pub fn new(reference: u64, index: usize) -> ReferenceFrame {
        ReferenceFrame {
            reference,
            index,
            new_references: Vec::new()
        }
    }

    pub fn step(&mut self) {
        self.index += 1;
    }
}