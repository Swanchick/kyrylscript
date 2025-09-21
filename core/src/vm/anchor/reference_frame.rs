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


    pub fn add_reference(&mut self, reference: u64) {
        self.new_references.push(reference);
    }
}