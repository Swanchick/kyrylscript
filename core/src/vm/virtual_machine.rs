use std::collections::HashMap;

use crate::compiler::instruction::Instruction;
use crate::vm::call_stack::CallStack;

use super::environment::Environment;
use super::variable::Variable;

pub struct VirtualMachine {
    environment: Environment,
    instruction_position: usize,
    variable_stack: Vec<Variable>,
    call_stack: Vec<CallStack>,
    compilation: HashMap<String, Instruction>
}

impl VirtualMachine {
    pub fn from(compilation: HashMap<String, Instruction>) -> VirtualMachine {
        VirtualMachine { 
            environment: Environment::new(),
            instruction_position: 0,
            variable_stack: Vec::new(),
            call_stack: Vec::new(),
            compilation
        }
    }

    fn enter_function(&mut self, function: &str) {
        
    }

    fn leave_function(&mut self) {

    }

    pub fn interpret(&mut self, function: &str) {



    }


    // fn peek(&self) -> Option<&Instruction> {
    //     self.instructions.get(self.instruction_position)
    // }
}
