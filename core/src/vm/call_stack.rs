use crate::global::constants::Instructions;
use crate::compiler::instruction::Instruction;

#[derive(Debug)]
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

    pub fn current_step(&self) -> usize {
        self.step
    }

    pub fn step(&mut self) {
        self.step += 1;
    }

    pub fn peek(&self) -> Option<&Instruction> {
        self.instructions.get(self.step)
    } 

    pub fn peek_mut(&mut self) -> Option<&mut Instruction> {
        self.instructions.get_mut(self.step)
    } 

    pub fn scopes(&self) -> usize {
        self.scopes
    }

    pub fn enter_scope(&mut self) {
        self.scopes += 1;
    }

    pub fn exit_scope(&mut self) {
        self.scopes -= 1;
    }
}