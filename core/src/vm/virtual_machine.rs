use std::collections::HashMap;
use std::env::var;

use crate::compiler::constant::Constant;
use crate::compiler::instruction::Instruction;
use crate::global::constants::{
    MAIN_FUNCTION, 
    MAX_DEPTH_RECURSION
};
use crate::global::utils::ks_error::KsError;
use crate::global::utils::ks_result::KsResult;
use crate::interpreter::enviroment;
use crate::native_registry::native_registry::NativeRegistry;
use crate::native_registry::native_types::NativeTypes;
use crate::vm::variable;
use crate::vm::variable_stack::{self, VariableStack};

use super::call_stack::CallStack;
use super::environment::Environment;
use super::variable::Variable;
use super::value::Value;

pub struct VirtualMachine {
    environment: Environment,
    variable_stack: Vec<VariableStack>,
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

    fn enter_function(&mut self, function: &str) -> KsResult<()> {
        if self.call_stack.len() >= MAX_DEPTH_RECURSION {
            return Err(KsError::runtime("Reached the maximum recursion depth!"));
        }
        
        let instructions = self.compilation.get(function);
        if let Some(instructions) = instructions {
            let call_stack = CallStack::new(function, instructions.to_vec());

            self.call_stack.push(call_stack);
        }

        Ok(())
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
            Constant::Function(name) => {
                let native = NativeRegistry::get();
                let native = native.borrow();
                
                if let Some(NativeTypes::NativeFunction(_)) = native.get_native(name) {
                    Value::NativeFunction(name.clone())
                } else {
                    Value::Function(name.clone())
                }
            },
            Constant::Null => Value::Null
        };

        Variable::empty(value)
    }

    fn define_variable(&mut self, name: &str) -> KsResult<()> {
        let variable_stack = self.variable_stack.pop();

        match variable_stack {
            Some(VariableStack::Variable(variable)) => {
                self.environment.define_variable(name, variable);
            },

            Some(VariableStack::Reference(reference)) => {
                self.environment.define_reference(name, &reference);
            },

            _ => self.environment.define_variable(
                name, 
                Variable::null()
            )
        }

        Ok(())
    }

    fn call_native_function(&mut self, args: Vec<&mut Variable>) {

    }

    fn call_function(&mut self, name: &str, args: Vec<&VariableStack>) -> KsResult<Variable> {
        todo!()
    }


    fn extract_function(&mut self, args: &usize) -> KsResult<Variable> {
        let function_variable = self.variable_stack.pop();
        
        if let Some(VariableStack::Reference(reference)) = function_variable {
            let value = {
                let variable = self.environment.variable(&reference)?;
                variable.value()
            };

            let mut name = String::new(); 

            Ok(Variable::null())
        } else {
            Ok(Variable::null())
        }
    }

    fn extract_variable(&mut self) -> KsResult<Variable> {
        let last_stack = self.variable_stack.pop();
        if let Some(last_stack) = last_stack {
            match last_stack {
                VariableStack::Variable(variable) => {
                    Ok(variable)
                },

                VariableStack::Reference(reference) => {
                    let variable = self.environment.variable(&reference)?;

                    Ok(variable.clone())
                }
            }
        } else {
            Err(KsError::runtime("No variable!"))
        }
    }

    fn add(&self, left: &Value, right: &Value) -> KsResult<Value> {
        match (left, right) {
            (Value::Integer(left), Value::Integer(right)) => 
                Ok(Value::Integer(*left + *right)),
            
            (Value::Integer(left), Value::Float(right)) => 
                Ok(Value::Float(*left as f64 + *right)),
            
            (Value::Float(left), Value::Integer(right)) => 
                Ok(Value::Float(*left + *right as f64)),
            
            (Value::String(left), Value::String(right)) => {
                let mut left = left.clone();
                left.push_str(&right);

                Ok(Value::String(left))
            },
            
            _ => Err(KsError::runtime("Arithmetic error!"))
        }
    }

    fn minus(&self, left: &Value, right: &Value) -> KsResult<Value> {
        match (left, right) {
            (Value::Integer(left), Value::Integer(right)) => 
                Ok(Value::Integer(*left - *right)),
            
            (Value::Integer(left), Value::Float(right)) => 
                Ok(Value::Float(*left as f64 - *right)),
            
            (Value::Float(left), Value::Integer(right)) => 
                Ok(Value::Float(*left - *right as f64)),
            
            _ => Err(KsError::runtime("Arithmetic error!"))
        }
    }

    fn multiply(&self, left: &Value, right: &Value) -> KsResult<Value> {
        match (left, right) {
            (Value::Integer(left), Value::Integer(right)) => 
                Ok(Value::Integer(*left * *right)),
            
            (Value::Integer(left), Value::Float(right)) => 
                Ok(Value::Float(*left as f64 * *right)),
            
            (Value::Float(left), Value::Integer(right)) => 
                Ok(Value::Float(*left * *right as f64)),
            
            _ => Err(KsError::runtime("Arithmetic error!"))
        }
    }

    fn divide(&self, left: &Value, right: &Value) -> KsResult<Value> {
        match (left, right) {
            (Value::Integer(left), Value::Integer(right)) => {
                if *right == 0 {
                    return Err(KsError::runtime("Division zero Error"));
                }

                Ok(Value::Integer(*left / *right))
            },

            (Value::Integer(left), Value::Float(right)) => {
                if *right == 0.0 {
                    return Err(KsError::runtime("Division zero Error"));
                }

                Ok(Value::Float(*left as f64 / *right))
            }

            (Value::Float(left), Value::Integer(right)) => {
                if *right == 0 {
                    return Err(KsError::runtime("Division zero Error"));
                }

                Ok(Value::Float(*left / *right as f64))
            }

            _ => Err(KsError::runtime("Arithmetic error!"))
        }
    }

    fn equal(&self, left: &Value, right: &Value) -> KsResult<Value> {
        Ok(Value::Boolean(left == right))
    }

    fn not_equal(&self, left: &Value, right: &Value) -> KsResult<Value> {
        Ok(Value::Boolean(left != right))
    }

    fn greater_equal(&self, left: &Value, right: &Value) -> KsResult<Value> {
        match (left, right) {
            (Value::Integer(left), Value::Integer(right)) => 
                Ok(Value::Boolean(*left >= *right)),
            
            (Value::Integer(left), Value::Float(right)) => 
                Ok(Value::Boolean(*left as f64 >= *right)),
            
            (Value::Float(left), Value::Integer(right)) => 
                Ok(Value::Boolean(*left >= *right as f64)),
            
            _ => Err(KsError::runtime("Logic error!"))
        }
    }

    fn greater(&self, left: &Value, right: &Value) -> KsResult<Value> {
        match (left, right) {
            (Value::Integer(left), Value::Integer(right)) => 
                Ok(Value::Boolean(*left > *right)),
            
            (Value::Integer(left), Value::Float(right)) => 
                Ok(Value::Boolean(*left as f64 > *right)),
            
            (Value::Float(left), Value::Integer(right)) => 
                Ok(Value::Boolean(*left > *right as f64)),
            
            _ => Err(KsError::runtime("Logic error!"))
        }
    }

    fn less_equal(&self, left: &Value, right: &Value) -> KsResult<Value> {
        match (left, right) {
            (Value::Integer(left), Value::Integer(right)) => 
                Ok(Value::Boolean(*left <= *right)),
            
            (Value::Integer(left), Value::Float(right)) => 
                Ok(Value::Boolean(*left as f64 <= *right)),
            
            (Value::Float(left), Value::Integer(right)) => 
                Ok(Value::Boolean(*left <= *right as f64)),
            
            _ => Err(KsError::runtime("Logic error!"))
        }
    }

    fn less(&self, left: &Value, right: &Value) -> KsResult<Value> {
        match (left, right) {
            (Value::Integer(left), Value::Integer(right)) => 
                Ok(Value::Boolean(*left < *right)),
            
            (Value::Integer(left), Value::Float(right)) => 
                Ok(Value::Boolean((*left as f64) < *right)),
            
            (Value::Float(left), Value::Integer(right)) => 
                Ok(Value::Boolean(*left < *right as f64)),
            
            _ => Err(KsError::runtime("Logic error!"))
        }
    }

    fn not(&self, value: &Value) -> KsResult<Value> {
        if let Value::Boolean(boolean) = value {
            Ok(Value::Boolean(!boolean))
        } else {
            Err(KsError::runtime("Logic error!"))
        }
    }

    fn and(&self, left: &Value, right: &Value) -> KsResult<Value> {
        match (left, right) {
            (Value::Boolean(left), Value::Boolean(right)) => 
                Ok(Value::Boolean(*left && *right)),
            
            _ => Err(KsError::runtime("Logic error!"))
        }
    }

    fn or(&self, left: &Value, right: &Value) -> KsResult<Value> {
        match (left, right) {
            (Value::Boolean(left), Value::Boolean(right)) => 
                Ok(Value::Boolean(*left || *right)),
            
            _ => Err(KsError::runtime("Logic error!"))
        }
    }

    fn clone(&self, reference: u64) -> KsResult<Variable> {
        let variable = self.environment.variable(&reference)?;
        let mut variable = variable.clone();
        variable.clear();
        
        Ok(variable)
    }

    fn interpret(&mut self) -> KsResult<()> {
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
                self.variable_stack.push(VariableStack::Variable(variable));
                self.step();
            },

            Some(Instruction::LoadVar(name)) => {
                let reference = self.environment.find_reference(name);
                if let Some(reference) = reference {
                    self.variable_stack.push(VariableStack::Reference(reference));
                } else {
                    return Err(KsError::runtime(&format!("Cannot find variable {}!", name)));
                }

                self.step();
            },

            Some(Instruction::Add) => {
                let right = self.extract_variable()?;
                let left = self.extract_variable()?;

                
            },

            Some(Instruction::Minus) => {

            },

            Some(Instruction::Mul) => {

            },

            Some(Instruction::Div) => {

            },

            Some(Instruction::Eq) => {

            },

            Some(Instruction::GreaterEq) => {

            },

            Some(Instruction::Greater) => {

            },

            Some(Instruction::LessEq) => {

            },

            Some(Instruction::Less) => {

            },

            Some(Instruction::NotEq) => {

            },

            Some(Instruction::And) => {

            },

            Some(Instruction::Or) => {

            },

            Some(Instruction::Not) => {

            },

            Some(Instruction::Call { args }) => {
                
            },

            Some(Instruction::Store(name)) => {
                let name = name.clone();
                self.define_variable(&name)?;
            },

            _ => self.exit_function()
        }

        Ok(())
    }

    pub fn start(&mut self) -> KsResult<()> {
        self.enter_function(MAIN_FUNCTION)?;

        while self.call_stack.len() != 0 {
            self.interpret()?;
        }


        Ok(())
    }
}
