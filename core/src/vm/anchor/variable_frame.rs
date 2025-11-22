pub struct VariableFrame {
    pub reference: u64,
    pub index: usize,
}

impl VariableFrame {
    pub fn new(reference: u64, index: usize) -> VariableFrame {
        VariableFrame { reference, index }
    }

    pub fn step(&mut self) {
        self.index += 1;
    }
}
