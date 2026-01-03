use crate::environment::Reference;

pub struct VariableFrame {
    pub reference: Reference,
    pub index: usize,
}

impl VariableFrame {
    pub fn new(reference: Reference, index: usize) -> VariableFrame {
        VariableFrame { reference, index }
    }

    pub fn step(&mut self) {
        self.index += 1;
    }
}
