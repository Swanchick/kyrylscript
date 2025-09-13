use std::collections::HashMap;

use crate::compiler::constant::Constant;
use crate::compiler::function::Function;
use crate::compiler::instruction::Instruction;
use crate::global::constants::{
    DEAFULT_FUNCTION, FUNCTION_ENCAPSULATION, MAIN_FUNCTION, MAX_DEPTH_RECURSION, MIN_SCOPES
};
use crate::global::utils::ks_error::KsError;
use crate::global::utils::ks_result::KsResult;
use crate::native_registry::native_registry::NativeRegistry;
use crate::native_registry::native_types::NativeTypes;

use super::variable_stack::VariableStack;
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
            call_stack: vec![
                CallStack::new(DEAFULT_FUNCTION, Vec::new()),
            ],
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

    fn exit_function(&mut self) -> KsResult<()> {
        self.call_stack.pop();
        self.environment.exit()?;

        if self.call_stack.len() != 0 {
            self.step()?;
        }

        Ok(())
    }

    fn enter_scope(&mut self) -> KsResult<()> {
        self.environment.enter();

        let call_stack = self.call_stack_last_mut()?;
        call_stack.enter_scope();

        Ok(())
    }

    fn exit_scope(&mut self) -> KsResult<()> {
        self.environment.exit()?;

        let call_stack = self.call_stack_last_mut()?;
        call_stack.exit_scope();

        Ok(())
    }

    fn depth(&self) -> usize {
        self.environment.depth()
    }

    fn call_stack_last(&self) -> KsResult<&CallStack> {
        if let Some(call_stack) = self.call_stack.last() {
            Ok(call_stack)
        } else {
            Err(KsError::runtime("There is no more callstacks!"))
        }
    }

    fn call_stack_last_mut(&mut self) -> KsResult<&mut CallStack> {
        if let Some(call_stack) = self.call_stack.last_mut() {
            Ok(call_stack)
        } else {
            Err(KsError::runtime("There is no more callstacks!"))
        }
    }

    fn step(&mut self) -> KsResult<()> {
        let call_stack = self.call_stack_last_mut()?;
        call_stack.step();

        Ok(())
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
                self.environment.define_name_reference(name, &reference)?;
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
        for _ in 0..args_size {
            let arg = self.extract_variable_from_stack()?;
            args.push(arg);
        }

        args.reverse();

        if let Some(NativeTypes::NativeFunction(native_function)) = native_function {
            (native_function.function)(args)?;
        }

        Ok(())
    }

    pub fn call_function(&mut self, name: &str) -> KsResult<()> {
        let arg_names = self.enter_function(name)?;

        for arg_name in arg_names {
            let arg_stack = self.variable_stack.pop();

            match arg_stack {
                Some(VariableStack::Variable(mut variable)) => {
                    variable.set_depth(self.depth());
                    self.environment.define_variable(&arg_name, variable);
                }
                Some(VariableStack::Reference(reference)) => 
                    self.environment.define_name_reference(&arg_name, &reference)?,
                _ => 
                    return Err(KsError::runtime("Cannot find argument")) 
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
                    self.call_function(name)?,
                Value::NativeFunction(name) => {
                    self.call_native_function(name, args)?;
                    self.step()?;
                },
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

    fn clone(&self, stack: Option<VariableStack>) -> KsResult<Variable> {
        match stack {
            Some(VariableStack::Reference(reference)) => {
                let variable = self.environment.variable(&reference)?;
                let mut variable = variable.clone();
                variable.set_depth(self.depth());
                variable.clear();

                Ok(variable)
            },
            Some(VariableStack::Variable(variable)) => Ok(variable),
            _ => Err(KsError::runtime("No variable were provided!"))
        }
    }

    fn value_to_variable_stack(&mut self, value: Value) {
        let variable = Variable::empty(value, self.depth());

        self.variable_stack.push(VariableStack::Variable(variable));
    }

    fn on_return(&mut self) -> KsResult<()> {
        let call_stack = self.call_stack_last()?;
        let scopes = call_stack.scopes();

        if scopes != 0 {
            for _ in 0..scopes  {
                self.exit_scope()?;
            }
        }

        self.exit_function()?;
        
        Ok(())
    }    

    fn jump(&mut self, distance: i32) -> KsResult<()> {
        let call_stack = self.call_stack_last_mut()?;
        call_stack.add_steps(distance);
        Ok(())
    }

    fn check_boolean(&self, variable: &Variable) -> KsResult<bool> {
        if let Value::Boolean(boolean) = variable.value() {
            Ok(*boolean)
        } else {
            Err(KsError::runtime("The value is not a boolean!"))
        }
    }

    fn jump_if_false(&mut self, distance: i32) -> KsResult<()> {
        let variable_stack = self.variable_stack.pop();
        match variable_stack {
            Some(VariableStack::Variable(variable)) => {
                let result = self.check_boolean(&variable)?;

                if !result {
                    let call_stack = self.call_stack_last_mut()?;
                    call_stack.add_steps(distance);
                } else {
                    self.step()?;
                }
            },
            Some(VariableStack::Reference(reference)) => {
                let variable = self.environment.variable(&reference)?;
                let result = self.check_boolean(variable)?;

                if !result {
                    let call_stack = self.call_stack_last_mut()?;
                    call_stack.add_steps(distance);
                } else {
                    self.step()?;
                }
            },
            _ => 
                return Err(KsError::runtime("There is no more variable stacks!"))
        }

        Ok(())
    }

    fn load_references_collection(&mut self, size: usize) -> KsResult<Vec<u64>> {
        let mut references: Vec<u64> = Vec::new();

        for _ in 0..size {
            let stack = self.variable_stack.pop();

            match stack {
                Some(VariableStack::Variable(variable)) => {
                    let reference = self.environment.define_reference(variable)?;
                    references.push(reference);
                },
                Some(VariableStack::Reference(reference)) => 
                    references.push(reference),
                _ => 
                    break
            }
        }

        references.reverse();
        Ok(references)
    }

    fn list_len(&mut self) -> KsResult<()> {
        let stack = self.variable_stack.pop();

        match stack {
            Some(VariableStack::Variable(variable)) => {
                if let Value::List(references) = variable.value() {
                    let variable = Variable::empty(
                        Value::Integer(references.len() as i32), 
                        self.depth()
                    );

                    self.variable_stack.push(VariableStack::Variable(variable));
                }
            },
            Some(VariableStack::Reference(reference)) => {
                let variable = self.environment.variable(&reference)?;

                if let Value::List(references) = variable.value() {
                    let variable = Variable::empty(
                        Value::Integer(references.len() as i32), 
                        self.depth()
                    );

                    self.variable_stack.push(VariableStack::Variable(variable));
                }
            },
            _ => 
                return Err(KsError::runtime("There is no more variable stacks!"))
        }
        
        self.step()?;

        Ok(())
    }

    fn load_integer(&mut self) -> KsResult<i32> {
        let stack = self.variable_stack.pop();

        match stack {
            Some(VariableStack::Variable(variable)) => {
                if let Value::Integer(integer) = variable.value() {
                    return Ok(*integer)
                }
            },
            Some(VariableStack::Reference(reference)) => {
                let variable = self.environment.variable(&reference)?;
                if let Value::Integer(integer) = variable.value() {
                    return Ok(*integer);
                }
            },
            _ => 
                return Err(KsError::runtime("There is no more variable stacks!"))
        }
        
        Err(KsError::runtime("Cannot load integer"))
    }

    fn load_from_list(&mut self) -> KsResult<()> {
        let index = self.load_integer()? as usize;

        let stack = self.variable_stack.pop();

        match stack {
            Some(VariableStack::Variable(variable)) => {
                if let Value::List(list) = variable.value() {
                    let reference = list.get(index);
                    if let Some(reference) = reference {
                        self.variable_stack.push(VariableStack::Reference(*reference));
                    } else {
                        return Err(KsError::runtime("List indexing out of bounces!"));
                    }
                }
            },
            Some(VariableStack::Reference(reference)) => {
                let variable = self.environment.variable(&reference)?;
                if let Value::List(list) = variable.value() {
                    let reference = list.get(index);
                    if let Some(reference) = reference {
                        self.variable_stack.push(VariableStack::Reference(*reference));
                    } else {
                        return Err(KsError::runtime("List indexing out of bounces!"));
                    }
                }
            },
            _ => 
                return Err(KsError::runtime("There is no more variable stacks!"))
        }

        self.step()?;
        
        Ok(())
    }

    fn load_from_tuple(&mut self, index: usize) -> KsResult<()> {
        let stack = self.variable_stack.pop();

        match stack {
            Some(VariableStack::Variable(variable)) => {
                if let Value::Tuple(list) = variable.value() {
                    let reference = list.get(index);
                    if let Some(reference) = reference {
                        self.variable_stack.push(VariableStack::Reference(*reference));
                    } else {
                        return Err(KsError::runtime("List indexing out of bounces!"));
                    }
                }
            },
            Some(VariableStack::Reference(reference)) => {
                let variable = self.environment.variable(&reference)?;
                if let Value::Tuple(list) = variable.value() {
                    let reference = list.get(index);
                    if let Some(reference) = reference {
                        self.variable_stack.push(VariableStack::Reference(*reference));
                    } else {
                        return Err(KsError::runtime("List indexing out of bounces!"));
                    }
                }
            },
            _ => 
                return Err(KsError::runtime("There is no more variable stacks!"))
        }

        self.step()?;
        
        Ok(())
    }

    fn assign_with_reference(&mut self, name: String, reference: u64, assign_reference: u64) -> KsResult<()> {
        let assign_variable = self.environment.variable(&assign_reference)?;
        let variable = self.environment.variable(&reference)?;
        let assign_depth = assign_variable.depth();
        let variable_depth = variable.depth();

        self.environment.free(&reference)?;
        self.environment.add_variable_owner(assign_reference, assign_depth)?;

        let same_scope_or_lower = assign_depth <= variable_depth;
        let scope_difference = variable_depth < assign_depth;
        if same_scope_or_lower {
            self.environment.assign_to_name(&name, &assign_reference)?;
        } else if scope_difference {
            self.environment.anchor(
                variable_depth, 
                assign_reference
            )?;

            self.environment.assign_to_name(&name, &assign_reference)?;
        }

        Ok(())
    }

    fn assign(&mut self, name: String) -> KsResult<()> {
        let reference = self.environment.reference(&name)?;
        let stack = self.variable_stack.pop();

        match stack {
            Some(VariableStack::Variable(variable)) =>
                self.environment.assign_to_reference(reference, variable)?,
            Some(VariableStack::Reference(assign_reference)) => 
                self.assign_with_reference(name, reference, assign_reference)?,
            _ => 
                return Err(KsError::runtime("There is no more variable stacks!"))
        }
        
        Ok(())
    }

    fn interpret(&mut self) -> KsResult<()> {
        let instruction = {
            let call_stack = self.call_stack_last();
            if let Ok(call_stack) = call_stack {
                call_stack.peek()
            } else {
                None
            }
        };

        match instruction {
            Some(Instruction::LoadConst(constant)) => {
                let variable = self.constant_to_variable(&constant);
                self.variable_stack.push(VariableStack::Variable(variable));
                self.step()?;
            },

            Some(Instruction::LoadVar(name)) => {                
                let reference = self.environment.find_reference(&name);

                if let Some(reference) = reference {
                    self.variable_stack.push(VariableStack::Reference(reference));
                } else {
                    return Err(KsError::runtime(&format!("Cannot find variable {}!", name)));
                }

                self.step()?;
            },

            Some(Instruction::Add) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;

                let value = self.add(left.value(), right.value())?;
                self.value_to_variable_stack(value);

                self.step()?;
            },

            Some(Instruction::Minus) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;
                
                let value = self.minus(left.value(), right.value())?;
                self.value_to_variable_stack(value);

                self.step()?;
            },

            Some(Instruction::Mul) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;
                
                let value = self.multiply(left.value(), right.value())?;
                self.value_to_variable_stack(value);

                self.step()?;
            },

            Some(Instruction::Div) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;
                
                let value = self.divide(left.value(), right.value())?;
                self.value_to_variable_stack(value);

                self.step()?;
            },

            Some(Instruction::Eq) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;
                
                let value = self.equal(left.value(), right.value())?;
                self.value_to_variable_stack(value);

                self.step()?;
            },

            Some(Instruction::GreaterEq) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;
                
                let value = self.greater_equal(left.value(), right.value())?;
                self.value_to_variable_stack(value);

                self.step()?;
            },

            Some(Instruction::Greater) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;
                
                let value = self.greater(left.value(), right.value())?;
                self.value_to_variable_stack(value);

                self.step()?;
            },

            Some(Instruction::LessEq) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;

                let value = self.less_equal(left.value(), right.value())?;
                self.value_to_variable_stack(value);

                self.step()?;
            },

            Some(Instruction::Less) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;
                
                let value = self.less(left.value(), right.value())?;
                self.value_to_variable_stack(value);
    
                self.step()?;
            },

            Some(Instruction::NotEq) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;
                
                let value = self.not_equal(left.value(), right.value())?;
                self.value_to_variable_stack(value);

                self.step()?;
            },

            Some(Instruction::And) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;
                
                let value = self.and(left.value(), right.value())?;
                self.value_to_variable_stack(value);

                self.step()?;
            },

            Some(Instruction::Or) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;
                
                let value = self.or(left.value(), right.value())?;
                self.value_to_variable_stack(value);

                self.step()?;
            },

            Some(Instruction::Not) => {
                let variable = self.extract_variable_from_stack()?;
                
                let value = self.not(variable.value())?;
                self.value_to_variable_stack(value);

                self.step()?;
            },

            Some(Instruction::Clone) => {
                let variable = self.variable_stack.pop();

                let variable = self.clone(variable)?;
                self.variable_stack.push(VariableStack::Variable(variable));

                self.step()?;
            },

            Some(Instruction::Store(name)) => {
                let name = name.clone();
                self.define_variable(&name)?;

                self.step()?;
            },
            
            Some(Instruction::Assign(name)) => {
                let name = name.clone();
                self.assign(name)?;
                
                self.step()?;
            },

            Some(Instruction::Enter) => {
                self.enter_scope()?;
                self.step()?;
            },
            
            Some(Instruction::Exit) => {
                self.exit_scope()?;
                self.step()?;
            },

            Some(Instruction::End) => {
                self.variable_stack.clear();
                self.step()?;
            },

            Some(Instruction::LoadList(size)) => {
                let referneces = self.load_references_collection(*size)?;
                let variable = Variable::empty(Value::List(referneces), self.depth());
                self.variable_stack.push(VariableStack::Variable(variable));

                self.step()?;
            },

            Some(Instruction::LoadTuple(size)) => {
                let referneces = self.load_references_collection(*size)?;
                let variable = Variable::empty(Value::Tuple(referneces), self.depth());
                self.variable_stack.push(VariableStack::Variable(variable));

                self.step()?;
            },

            Some(Instruction::LoadFromList) => self.load_from_list()?,
            Some(Instruction::LoadFromTuple(index)) => self.load_from_tuple(*index)?,
            Some(Instruction::ListLen) => self.list_len()?,
            Some(Instruction::Return) => self.on_return()?,
            Some(Instruction::Call { args }) => self.extract_function(args.clone())?,
            Some(Instruction::Jump(distance)) => self.jump(*distance)?,
            Some(Instruction::JumpIfFalse(distance)) => self.jump_if_false(*distance)?,

            _ => {
                self.exit_function()?;
                self.variable_stack.push(VariableStack::Variable(Variable::null(self.depth())));
            }
        }

        Ok(())
    }

    fn load_native(&mut self) {
        let native = NativeRegistry::get();
        let native = native.borrow();

        for (name, _) in native.get_natives() {
            self.environment.define_variable(name, Variable::empty(Value::NativeFunction(name.clone()), self.depth()));
        }
    }

    pub fn initialize(&mut self) -> KsResult<()> {
        self.enter_scope()?;
        self.load_native();
        self.enter_function(MAIN_FUNCTION)?;

        while self.call_stack.len() > MIN_SCOPES {
            self.interpret()?;
        }
        
        Ok(())
    }

    pub fn call(&mut self, name: &str) -> KsResult<()> {
        let function_name = &format!(
            "{}{}", 
            FUNCTION_ENCAPSULATION, 
            name
        );
        
        self.enter_function(function_name)?;

        while self.call_stack.len() > MIN_SCOPES {
            self.interpret()?;
        }

        Ok(())
    }
}
