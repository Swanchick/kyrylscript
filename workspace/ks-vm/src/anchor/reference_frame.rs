use crate::environment::Reference;

pub struct ReferenceFrame {
    pub reference: Reference,
    pub index: usize,
    pub new_references: Vec<Reference>,
}

impl ReferenceFrame {
    pub fn new(reference: Reference, index: usize) -> ReferenceFrame {
        ReferenceFrame {
            reference,
            index,
            new_references: Vec::new(),
        }
    }

    pub fn step(&mut self) {
        self.index += 1;
    }
}
