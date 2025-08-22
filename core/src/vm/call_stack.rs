use crate::global::constants::Instructions;
use crate::compiler::instruction::Instruction;

pub struct CallStack {
    name: String,
    instructions: Instructions,
    step: usize,
    scopes: usize
}

impl CallStack {
    pub fn new(name: &str, instructions: Instructions) -> CallStack {
        CallStack { 
            name: name.to_string(), 
            instructions, 
            step: 0,
            scopes: 0 
        }
    }

    pub fn step(&mut self) {
        self.step += 1;
    }

    pub fn peek(&self) -> Option<&Instruction> {
        self.instructions.get(self.step)
    } 
}