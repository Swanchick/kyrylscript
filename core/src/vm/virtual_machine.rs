use std::collections::HashMap;

use crate::compiler::constant::Constant;
use crate::compiler::instruction::Instruction;
use crate::global::constants::MAIN_FUNCTION;
use crate::global::utils::ks_result::KsResult;
use crate::vm::call_stack::CallStack;

use super::environment::Environment;
use super::variable::Variable;
use super::value::Value;

pub struct VirtualMachine {
    environment: Environment,
    variable_stack: Vec<Variable>,
    call_stack: Vec<CallStack>,
    compilation: HashMap<String, Vec<Instruction>>
}

impl VirtualMachine {
    pub fn from(compilation: HashMap<String, Vec<Instruction>>) -> VirtualMachine {
        VirtualMachine { 
            environment: Environment::new(),
            variable_stack: Vec::new(),
            call_stack: Vec::new(),
            compilation
        }
    }

    fn enter_function(&mut self, function: &str) {
        let instructions = self.compilation.get(function);
        if let Some(instructions) = instructions {
            let call_stack = CallStack::new(function, instructions.to_vec());

            self.call_stack.push(call_stack);
        }
    }

    fn exit_function(&mut self) {
        self.call_stack.pop();
    }

    fn call_stack_last(&self) -> Option<&CallStack> {
        self.call_stack.last()
    }

    fn call_stack_last_mut(&mut self) -> Option<&mut CallStack> {
        self.call_stack.last_mut()
    }

    fn step(&mut self) {
        let call_stack = self.call_stack_last_mut();
        if let Some(call_stack) = call_stack {
            call_stack.step();
        }
    }

    fn constant_to_variable(&self, constant: &Constant) -> Variable {
        let value = match constant {
            Constant::String(string) => Value::String(string.clone()),
            Constant::Boolean(boolean) => Value::Boolean(*boolean),
            Constant::Integer(int) => Value::Integer(*int),
            Constant::Float(float) => Value::Float(*float),
            Constant::Function(name) => Value::Function(name.clone()),
            Constant::Null => Value::Null
        };

        Variable::empty(value)
    }

    fn interpret(&mut self) {
        let instruction = {
            let call_stack = self.call_stack_last();
            if let Some(call_stack) = call_stack {
                call_stack.peek()
            } else {
                None
            }
        };

        match instruction {
            Some(Instruction::LoadConst(constant)) => {
                let variable = self.constant_to_variable(constant);
                self.variable_stack.push(variable);
                self.step();
            },

            _ => {
                self.exit_function();
            }
        }
    }

    pub fn start(&mut self) -> KsResult<()> {
        self.enter_function(MAIN_FUNCTION);

        while self.call_stack.len() != 0 {
            self.interpret();
        }


        Ok(())
    }
}
