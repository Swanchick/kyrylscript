use std::collections::HashMap;

use crate::compiler::constant::Constant;
use crate::compiler::function::Function;
use crate::compiler::instruction::Instruction;
use crate::global::constants::{
    MAIN_FUNCTION, 
    MAX_DEPTH_RECURSION
};
use crate::global::utils::ks_error::KsError;
use crate::global::utils::ks_result::KsResult;
use crate::native_registry::native_registry::NativeRegistry;
use crate::native_registry::native_types::NativeTypes;
use crate::vm::variable_stack::VariableStack;

use super::call_stack::CallStack;
use super::environment::Environment;
use super::variable::Variable;
use super::value::Value;

pub struct VirtualMachine {
    environment: Environment,
    variable_stack: Vec<VariableStack>,
    call_stack: Vec<CallStack>,
    compilation: HashMap<String, Function>
}

impl VirtualMachine {
    pub fn from(compilation: HashMap<String, Function>) -> VirtualMachine {
        VirtualMachine { 
            environment: Environment::new(),
            variable_stack: Vec::new(),
            call_stack: Vec::new(),
            compilation
        }
    }

    fn enter_function(&mut self, name: &str) -> KsResult<Vec<String>> {
        if self.call_stack.len() >= MAX_DEPTH_RECURSION {
            return Err(KsError::runtime("Reached the maximum recursion depth!"));
        }
        
        let function = self.compilation.get(name);
        if let Some(function) = function {
            let instructions = function.get_instructions();
            let call_stack = CallStack::new(name, instructions.to_vec());
            self.call_stack.push(call_stack);
            self.environment.enter();

            Ok(function.get_args().to_vec())
        } else {
            Err(KsError::runtime(&format!("Cannot find function {}", name)))
        }
    }

    fn enter_scope(&mut self) {
        self.environment.enter();

        if let Some(call_stack) = self.call_stack_last_mut() {
            call_stack.enter_scope();
        }
    }

    fn exit_scope(&mut self) {
        self.environment.exit();

        if let Some(call_stack) = self.call_stack_last_mut() {
            call_stack.exit_scope();
        }
    }

    fn exit_function(&mut self) {
        self.call_stack.pop();
        self.environment.exit();
    }

    fn depth(&self) -> usize {
        self.environment.depth()
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

        Variable::empty(value, self.depth())
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
                Variable::null(self.depth())
            )
        }

        Ok(())
    }

    fn call_native_function(&mut self, name: &str, args_size: usize) -> KsResult<()> {
        let native = NativeRegistry::get();
        let native = native.borrow();
        let native_function = native.get_native(name);

        let mut args: Vec<Variable> = Vec::new();
        for i in 1..args_size {
            let arg = self.variable_stack.pop();
            
            match arg {
                Some(VariableStack::Variable(variable)) => 
                    args.push(variable),
                Some(VariableStack::Reference(reference)) => {
                    let variable = self.environment.variable(&reference)?;
                    let variable = variable.clone();

                    args.push(variable);
                },
                _ => unreachable!()
            }
        }

        if let Some(NativeTypes::NativeFunction(native_function)) = native_function {
            (native_function.function)(args);
        }
        

        Ok(())
    }

    pub fn call_function(&mut self, name: &str, args: usize) -> KsResult<()> {
        let arg_names = self.enter_function(name)?;

        for i in 1..args {
            let arg = self.variable_stack.pop();
            let arg_name = arg_names.get(i as usize);
            if let (Some(arg), Some(arg_name)) = (arg, arg_name) {
                match arg {
                    VariableStack::Variable(variable) => {
                        self.environment.define_variable(&arg_name, variable);
                    },
                    VariableStack::Reference(reference) => {
                        self.environment.define_reference(name, &reference);
                    }
                }
            }
        }

        Ok(())
    }

    fn extract_function(&mut self, args: usize) -> KsResult<()> {
        let stack = self.variable_stack.pop();
        if let Some(VariableStack::Reference(reference)) = stack {
            let function = {
                let function = self.environment.variable(&reference)?;
                function.clone()
            };

            match function.value() {
                Value::Function(name) => 
                    self.call_function(name, args)?,
                Value::NativeFunction(name) =>
                    self.call_native_function(name, args)?,
                _ => 
                    return Err(KsError::runtime("It's not a function!"))
            }
        }

        Ok(())
    }

    fn extract_variable_from_stack(&mut self) -> KsResult<Variable> {
        let last_stack = self.variable_stack.pop();
        
        match last_stack {
            Some(VariableStack::Variable(variable)) => {
                Ok(variable)
            },
            Some(VariableStack::Reference(reference)) => {
                let variable = self.environment.variable(&reference)?;

                Ok(variable.clone())
            },
            _ => Err(KsError::runtime("No variable!"))
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

    fn value_to_variable_stack(&mut self, value: Value) {
        let variable = Variable::empty(value, self.depth());
        self.variable_stack.push(VariableStack::Variable(variable));
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
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;
                
                let value = self.add(left.value(), right.value())?;
                self.value_to_variable_stack(value);
            },

            Some(Instruction::Minus) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;
                
                let value = self.minus(left.value(), right.value())?;
                self.value_to_variable_stack(value);
            },

            Some(Instruction::Mul) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;
                
                let value = self.multiply(left.value(), right.value())?;
                self.value_to_variable_stack(value);
            },

            Some(Instruction::Div) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;
                
                let value = self.divide(left.value(), right.value())?;
                self.value_to_variable_stack(value);
            },

            Some(Instruction::Eq) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;
                
                let value = self.equal(left.value(), right.value())?;
                self.value_to_variable_stack(value);
            },

            Some(Instruction::GreaterEq) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;
                
                let value = self.greater_equal(left.value(), right.value())?;
                self.value_to_variable_stack(value);
            },

            Some(Instruction::Greater) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;
                
                let value = self.greater(left.value(), right.value())?;
                self.value_to_variable_stack(value);
            },

            Some(Instruction::LessEq) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;
                
                let value = self.less_equal(left.value(), right.value())?;
                self.value_to_variable_stack(value);
            },

            Some(Instruction::Less) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;
                
                let value = self.less(left.value(), right.value())?;
                self.value_to_variable_stack(value);
            },

            Some(Instruction::NotEq) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;
                
                let value = self.not_equal(left.value(), right.value())?;
                self.value_to_variable_stack(value);
            },

            Some(Instruction::And) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;
                
                let value = self.and(left.value(), right.value())?;
                self.value_to_variable_stack(value);
            },

            Some(Instruction::Or) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;
                
                let value = self.or(left.value(), right.value())?;
                self.value_to_variable_stack(value);
            },

            Some(Instruction::Not) => {
                let variable = self.extract_variable_from_stack()?;
                
                let value = self.not(variable.value())?;
                self.value_to_variable_stack(value);
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
