use crate::constants::Instructions;
use crate::instruction::Instruction;

#[derive(Debug)]
pub struct CallStack {
    instructions: Instructions,
    step: usize,
    scopes: usize,
}

impl CallStack {
    pub fn new(instructions: Instructions) -> CallStack {
        CallStack {
            instructions,
            step: 0,
            scopes: 0,
        }
    }

    pub fn step(&mut self) {
        self.step += 1;
    }

    pub fn add_steps(&mut self, steps: i32) {
        let mut current_steps = self.step as i32;
        current_steps += steps;
        if current_steps < 0 {
            current_steps = 0;
        }

        self.step = current_steps as usize;
    }

    pub fn peek(&self) -> Option<&Instruction> {
        self.instructions.get(self.step)
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
