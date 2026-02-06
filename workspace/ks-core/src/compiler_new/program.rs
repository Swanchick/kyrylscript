use super::instructions::Instruction;
use super::types::FunctionPointer;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<Instruction>,
    functions: HashMap<String, FunctionPointer>,
}

impl Program {
    pub fn new(
        instructions: Vec<Instruction>,
        functions: HashMap<String, FunctionPointer>,
    ) -> Self {
        Self {
            instructions,
            functions,
        }
    }
}
