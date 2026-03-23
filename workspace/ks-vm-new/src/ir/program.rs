use crate::types::Pointer;
use std::collections::HashMap;

use super::instructions::Instruction;

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
