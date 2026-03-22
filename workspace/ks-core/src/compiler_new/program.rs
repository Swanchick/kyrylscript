use super::instructions::Instruction;
use super::types::Pointer;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<Instruction>,
    functions: HashMap<String, Pointer>,
}

impl Program {
    pub fn new(instructions: Vec<Instruction>, functions: HashMap<String, Pointer>) -> Self {
        Self {
            instructions,
            functions,
        }
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }
}
