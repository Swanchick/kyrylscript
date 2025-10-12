use crate::vm::variable::Variable;

pub struct Frame {
    pub variable: Variable,
    pub index: usize,
}

impl Frame {
    pub fn new(variable: Variable, index: usize) -> Frame {
        Frame {
            variable,
            index,
        }
    }

    pub fn step(&mut self) {
        self.index += 1;
    }
}
