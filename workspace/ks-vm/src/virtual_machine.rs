use std::collections::HashMap;

use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;

use super::call_stack::CallStack;
use super::constant::Constant;
use super::constants::{FUNCTION_ENCAPSULATION, MAIN_FUNCTION, MAX_DEPTH_RECURSION, MIN_SCOPES};
use super::environment::{Depth, Environment, Reference};
use super::function::Function;
use super::instruction::Instruction;
use super::native::native_registry::NativeRegistry;
use super::native::native_types::NativeType;
use super::tail_stack::TailStack;
use super::variable::value::Value;
use super::variable::var_info::VarInfo;
use super::variable::variable::Variable;
use super::variable::variable_stack::VariableStack;

pub struct VirtualMachine {
    environment: Environment,
    variable_stack: Vec<VariableStack>,
    call_stack: Vec<CallStack>,
    compilation: HashMap<String, Function>,
    tail_stack: Option<TailStack>,
}

impl VirtualMachine {
    pub fn from(compilation: HashMap<String, Function>) -> VirtualMachine {
        VirtualMachine {
            environment: Environment::new(),
            variable_stack: Vec::new(),
            call_stack: vec![CallStack::new(Vec::new())],
            compilation,
            tail_stack: None,
        }
    }

    fn enter_function(&mut self, name: &str) -> KsResult<Vec<String>> {
        if self.call_stack.len() >= MAX_DEPTH_RECURSION {
            return Err(KsError::runtime("Reached the maximum recursion depth!"));
        }

        let function = self.compilation.get(name);
        if let Some(function) = function {
            let instructions = function.get_instructions();
            let call_stack = CallStack::new(instructions.to_vec());
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

    fn depth(&self) -> Depth {
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

                if let Some(NativeType::Function(_)) = native.get_native(name) {
                    Value::NativeFunction(name.clone())
                } else {
                    Value::Function(name.clone())
                }
            }
            Constant::Null => Value::Null,
        };

        Variable::empty(value)
    }

    fn load_var(&mut self, name: String, save: bool) -> KsResult<()> {
        let reference = self.environment.find_reference(&name)?;
        if save {
            self.tail_stack = Some(TailStack::Variable(name));
        }

        self.variable_stack
            .push(VariableStack::Reference(reference));
        self.step()?;

        Ok(())
    }

    fn define_variable(&mut self, name: &str) -> KsResult<()> {
        let variable_stack = self.variable_stack.pop();

        match variable_stack {
            Some(VariableStack::Variable(variable)) => {
                self.environment.define_variable(name, variable)?;
            }
            Some(VariableStack::Reference(reference)) => {
                self.environment.define_name_reference(name, &reference)?;
            }
            _ => {
                self.environment.define_variable(name, Variable::null())?;
            }
        }

        self.step()?;

        Ok(())
    }

    fn public_define_variable(&mut self, name: &str) -> KsResult<()> {
        let variable_stack = self.variable_stack.pop();

        match variable_stack {
            Some(VariableStack::Variable(variable)) => {
                self.environment
                    .define_variable_at_depth(name, variable, 0)?;
            }

            Some(VariableStack::Reference(reference)) => {
                self.environment
                    .define_name_reference_at_depth(name, &reference, 0)?;
            }

            _ => self
                .environment
                .define_variable_at_depth(name, Variable::null(), 0)?,
        }

        self.step()?;

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

        if let Some(NativeType::Function(native_function)) = native_function {
            let variable = (native_function.function)(&mut self.environment, args)?;
            self.variable_stack.push(VariableStack::Variable(variable));
        }

        Ok(())
    }

    pub fn call_function(&mut self, name: &str) -> KsResult<()> {
        let mut arg_names = self.enter_function(name)?;
        arg_names.reverse();

        for arg_name in arg_names {
            let arg_stack = self.variable_stack.pop();

            match arg_stack {
                Some(VariableStack::Variable(variable)) => {
                    let reference = self.environment.define_variable(&arg_name, variable)?;
                    self.environment.set_depth(&reference, self.depth())?;
                }
                Some(VariableStack::Reference(reference)) => self
                    .environment
                    .define_name_reference(&arg_name, &reference)?,
                _ => return Err(KsError::runtime("Cannot find argument")),
            }
        }

        Ok(())
    }

    fn extract_function(&mut self, args: usize) -> KsResult<()> {
        let stack_len = self.variable_stack.len();
        let stack = self.variable_stack.remove(stack_len - args - 1);

        match stack {
            VariableStack::Variable(function) => match function.value() {
                Value::Function(name) => self.call_function(name)?,
                Value::NativeFunction(name) => {
                    self.call_native_function(name, args)?;
                    self.step()?;
                }
                _ => return Err(KsError::runtime("It's not a function!")),
            },
            VariableStack::Reference(reference) => {
                let function = {
                    let function = self.environment.variable(&reference)?;
                    function.clone()
                };

                match function.value() {
                    Value::Function(name) => self.call_function(name)?,
                    Value::NativeFunction(name) => {
                        self.call_native_function(name, args)?;
                        self.step()?;
                    }
                    _ => return Err(KsError::runtime("It's not a function!")),
                }
            }
        }

        Ok(())
    }

    fn extract_variable_from_stack(&mut self) -> KsResult<Variable> {
        let last_stack = self.variable_stack.pop();

        match last_stack {
            Some(VariableStack::Variable(variable)) => Ok(variable),
            Some(VariableStack::Reference(reference)) => {
                let variable = self.environment.variable(&reference)?;

                Ok(variable.clone())
            }
            _ => Err(KsError::runtime("No variable were provided!")),
        }
    }

    fn add(&self, left: &Value, right: &Value) -> KsResult<Value> {
        match (left, right) {
            (Value::Integer(left), Value::Integer(right)) => Ok(Value::Integer(*left + *right)),

            (Value::Integer(left), Value::Float(right)) => Ok(Value::Float(*left as f64 + *right)),

            (Value::Float(left), Value::Integer(right)) => Ok(Value::Float(*left + *right as f64)),

            (Value::String(left), Value::String(right)) => {
                let mut left = left.clone();
                left.push_str(&right);

                Ok(Value::String(left))
            }

            _ => Err(KsError::runtime("Arithmetic error!")),
        }
    }

    fn minus(&self, left: &Value, right: &Value) -> KsResult<Value> {
        match (left, right) {
            (Value::Integer(left), Value::Integer(right)) => Ok(Value::Integer(*left - *right)),

            (Value::Integer(left), Value::Float(right)) => Ok(Value::Float(*left as f64 - *right)),

            (Value::Float(left), Value::Integer(right)) => Ok(Value::Float(*left - *right as f64)),

            _ => Err(KsError::runtime("Arithmetic error!")),
        }
    }

    fn multiply(&self, left: &Value, right: &Value) -> KsResult<Value> {
        match (left, right) {
            (Value::Integer(left), Value::Integer(right)) => Ok(Value::Integer(*left * *right)),

            (Value::Integer(left), Value::Float(right)) => Ok(Value::Float(*left as f64 * *right)),

            (Value::Float(left), Value::Integer(right)) => Ok(Value::Float(*left * *right as f64)),

            _ => Err(KsError::runtime("Arithmetic error!")),
        }
    }

    fn divide(&self, left: &Value, right: &Value) -> KsResult<Value> {
        match (left, right) {
            (Value::Integer(left), Value::Integer(right)) => {
                if *right == 0 {
                    return Err(KsError::runtime("Division zero Error"));
                }

                Ok(Value::Integer(*left / *right))
            }

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

            _ => Err(KsError::runtime("Arithmetic error!")),
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
            (Value::Integer(left), Value::Integer(right)) => Ok(Value::Boolean(*left >= *right)),

            (Value::Integer(left), Value::Float(right)) => {
                Ok(Value::Boolean(*left as f64 >= *right))
            }

            (Value::Float(left), Value::Integer(right)) => {
                Ok(Value::Boolean(*left >= *right as f64))
            }

            _ => Err(KsError::runtime("Logic error!")),
        }
    }

    fn greater(&self, left: &Value, right: &Value) -> KsResult<Value> {
        match (left, right) {
            (Value::Integer(left), Value::Integer(right)) => Ok(Value::Boolean(*left > *right)),

            (Value::Integer(left), Value::Float(right)) => {
                Ok(Value::Boolean(*left as f64 > *right))
            }

            (Value::Float(left), Value::Integer(right)) => {
                Ok(Value::Boolean(*left > *right as f64))
            }

            _ => Err(KsError::runtime("Logic error!")),
        }
    }

    fn less_equal(&self, left: &Value, right: &Value) -> KsResult<Value> {
        match (left, right) {
            (Value::Integer(left), Value::Integer(right)) => Ok(Value::Boolean(*left <= *right)),

            (Value::Integer(left), Value::Float(right)) => {
                Ok(Value::Boolean(*left as f64 <= *right))
            }

            (Value::Float(left), Value::Integer(right)) => {
                Ok(Value::Boolean(*left <= *right as f64))
            }

            _ => Err(KsError::runtime("Logic error!")),
        }
    }

    fn less(&self, left: &Value, right: &Value) -> KsResult<Value> {
        match (left, right) {
            (Value::Integer(left), Value::Integer(right)) => Ok(Value::Boolean(*left < *right)),

            (Value::Integer(left), Value::Float(right)) => {
                Ok(Value::Boolean((*left as f64) < *right))
            }

            (Value::Float(left), Value::Integer(right)) => {
                Ok(Value::Boolean(*left < *right as f64))
            }

            _ => Err(KsError::runtime("Logic error!")),
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
            (Value::Boolean(left), Value::Boolean(right)) => Ok(Value::Boolean(*left && *right)),

            _ => Err(KsError::runtime("Logic error!")),
        }
    }

    fn or(&self, left: &Value, right: &Value) -> KsResult<Value> {
        match (left, right) {
            (Value::Boolean(left), Value::Boolean(right)) => Ok(Value::Boolean(*left || *right)),

            _ => Err(KsError::runtime("Logic error!")),
        }
    }

    fn clone(&mut self) -> KsResult<()> {
        let stack = self.variable_stack.pop();

        match stack {
            Some(VariableStack::Reference(reference)) => {
                let variable = self.environment.clone(reference)?;

                self.variable_stack.push(VariableStack::Variable(variable));
            }
            Some(VariableStack::Variable(_)) => {
                return Err(KsError::runtime("You cannot clone the expression!"));
            }
            _ => return Err(KsError::runtime("No variable were provided!")),
        }

        self.step()?;

        Ok(())
    }

    fn value_to_variable_stack(&mut self, value: Value) {
        let variable = Variable::empty(value);

        self.variable_stack.push(VariableStack::Variable(variable));
    }

    fn on_return(&mut self) -> KsResult<()> {
        let call_stack = self.call_stack_last()?;
        let call_stack_depth = call_stack.scopes();
        let current_depth = self.depth();
        let depth_to_return = current_depth - call_stack_depth;
        let stack = self.variable_stack.pop();

        match stack {
            Some(VariableStack::Variable(variable)) => {
                let reference = self
                    .environment
                    .define_reference_at_depth(variable, current_depth - 1)?;

                self.environment
                    .anchor_reference(current_depth - 1, reference)?;
                let variable = self.environment.variable_remove(&reference)?;
                self.variable_stack.push(VariableStack::Variable(variable));
            }
            Some(VariableStack::Reference(reference)) => {
                let variable_info = self.environment.info(&reference)?;
                let variable_depth = *variable_info.depth();

                let variable_inside_function = variable_depth >= depth_to_return;

                if variable_inside_function {
                    self.environment
                        .anchor_reference(depth_to_return - 1, reference)?;
                }

                self.variable_stack
                    .push(VariableStack::Reference(reference));
            }

            _ => return Err(KsError::runtime("No variable were provided")),
        }

        if call_stack_depth != 0 {
            for _ in 0..call_stack_depth {
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
            }
            Some(VariableStack::Reference(reference)) => {
                let variable = self.environment.variable(&reference)?;
                let result = self.check_boolean(variable)?;

                if !result {
                    let call_stack = self.call_stack_last_mut()?;
                    call_stack.add_steps(distance);
                } else {
                    self.step()?;
                }
            }
            _ => return Err(KsError::runtime("There is no more variable stacks!")),
        }

        Ok(())
    }

    fn load_references_collection(&mut self, size: usize) -> KsResult<Vec<Reference>> {
        let mut references: Vec<Reference> = Vec::new();

        for _ in 0..size {
            let stack = self.variable_stack.pop();

            match stack {
                Some(VariableStack::Variable(variable)) => {
                    let reference = self.environment.define_reference(variable)?;
                    references.push(reference);
                }
                Some(VariableStack::Reference(reference)) => references.push(reference),
                _ => break,
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
                    let variable = Variable::empty(Value::Integer(references.len() as i32));

                    self.variable_stack.push(VariableStack::Variable(variable));
                }
            }
            Some(VariableStack::Reference(reference)) => {
                let variable = self.environment.variable(&reference)?;

                if let Value::List(references) = variable.value() {
                    let variable = Variable::empty(Value::Integer(references.len() as i32));

                    self.variable_stack.push(VariableStack::Variable(variable));
                }
            }
            _ => return Err(KsError::runtime("There is no more variable stacks!")),
        }

        self.step()?;

        Ok(())
    }

    fn load_integer(&mut self) -> KsResult<i32> {
        let stack = self.variable_stack.pop();

        match stack {
            Some(VariableStack::Variable(variable)) => {
                if let Value::Integer(integer) = variable.value() {
                    return Ok(*integer);
                }
            }
            Some(VariableStack::Reference(reference)) => {
                let variable = self.environment.variable(&reference)?;
                if let Value::Integer(integer) = variable.value() {
                    return Ok(*integer);
                }
            }
            _ => return Err(KsError::runtime("There is no more variable stacks!")),
        }

        Err(KsError::runtime("Cannot load integer"))
    }

    fn load_from_list(&mut self, save: bool) -> KsResult<()> {
        let index = self.load_integer()? as usize;

        let stack = self.variable_stack.pop();

        match stack {
            Some(VariableStack::Variable(variable)) => {
                if let Value::List(list) = variable.value() {
                    let reference = list.get(index);
                    if let Some(reference) = reference {
                        if save {
                            self.tail_stack = Some(TailStack::Index {
                                index: index,
                                info: VarInfo::from(&variable)?,
                            });
                        }

                        self.variable_stack
                            .push(VariableStack::Reference(*reference));
                    } else {
                        return Err(KsError::runtime("List indexing out of bounces!"));
                    }
                }
            }
            Some(VariableStack::Reference(reference)) => {
                let variable = self.environment.variable(&reference)?;
                if let Value::List(list) = variable.value() {
                    let reference = list.get(index);
                    if let Some(reference) = reference {
                        if save {
                            self.tail_stack = Some(TailStack::Index {
                                index: index,
                                info: VarInfo::from(&variable)?,
                            });
                        }

                        self.variable_stack
                            .push(VariableStack::Reference(*reference));
                    } else {
                        return Err(KsError::runtime("List indexing out of bounces!"));
                    }
                }
            }
            _ => return Err(KsError::runtime("There is no more variable stacks!")),
        }

        self.step()?;

        Ok(())
    }

    fn load_from_tuple(&mut self, index: usize, save: bool) -> KsResult<()> {
        let stack = self.variable_stack.pop();

        match stack {
            Some(VariableStack::Variable(variable)) => {
                if let Value::Tuple(list) = variable.value() {
                    let reference = list.get(index);
                    if let Some(reference) = reference {
                        if save {
                            self.tail_stack = Some(TailStack::Index {
                                index: index,
                                info: VarInfo::from(&variable)?,
                            });
                        }

                        self.variable_stack
                            .push(VariableStack::Reference(*reference));
                    } else {
                        return Err(KsError::runtime("List indexing out of bounces!"));
                    }
                }
            }
            Some(VariableStack::Reference(reference)) => {
                let variable = self.environment.variable(&reference)?;
                if let Value::Tuple(list) = variable.value() {
                    let reference = list.get(index);
                    if let Some(reference) = reference {
                        if save {
                            self.tail_stack = Some(TailStack::Index {
                                index: index,
                                info: VarInfo::from(&variable)?,
                            });
                        }

                        self.variable_stack
                            .push(VariableStack::Reference(*reference));
                    } else {
                        return Err(KsError::runtime("List indexing out of bounces!"));
                    }
                }
            }
            _ => return Err(KsError::runtime("There is no more variable stacks!")),
        }

        self.step()?;

        Ok(())
    }

    fn assign_with_reference(
        &mut self,
        reference: Reference,
        assign_reference: Reference,
    ) -> KsResult<()> {
        let assign_depth = {
            let info = self.environment.info(&assign_reference)?;
            *info.depth()
        };
        let variable_depth = {
            let info = self.environment.info(&reference)?;
            *info.depth()
        };

        self.environment.free(&reference)?;
        self.environment.add_owner(&assign_reference)?;

        let scope_difference = variable_depth < assign_depth;

        if scope_difference {
            self.environment
                .anchor_reference(variable_depth, assign_reference)?;
        }

        Ok(())
    }

    fn extract_reference(&self, stack: Option<VariableStack>) -> KsResult<Reference> {
        if let Some(VariableStack::Reference(reference)) = stack {
            Ok(reference)
        } else {
            Err(KsError::runtime("Cannot extact reference!"))
        }
    }

    fn change_reference_holder(&mut self, new_reference: Reference) -> KsResult<()> {
        match &self.tail_stack {
            Some(TailStack::Index {
                index,
                info: tail_info,
            }) => {
                let reference = tail_info.reference()?;
                let list = self.environment.variable_mut(reference)?;
                if let Value::List(references) | Value::Tuple(references) = list.value_mut() {
                    references[*index] = new_reference;
                }
            }
            Some(TailStack::Module { name, info }) => {
                let reference = info.reference()?;
                let module = self.environment.variable_mut(reference)?;
                if let Value::Module(module) = module.value_mut() {
                    module.insert(name.to_string(), new_reference);
                }
            }
            Some(TailStack::Variable(name)) => {
                self.environment.assign_to_name(name, &new_reference)?;
            }
            _ => {}
        }

        Ok(())
    }

    fn assign(&mut self) -> KsResult<()> {
        let assign_value = self.variable_stack.pop();
        let assign_to = self.variable_stack.pop();
        let reference = self.extract_reference(assign_to)?;

        match assign_value {
            Some(VariableStack::Variable(variable)) => {
                self.environment.assign_to_reference(reference, variable)?
            }
            Some(VariableStack::Reference(assign_reference)) => {
                self.assign_with_reference(reference, assign_reference)?;
                self.change_reference_holder(assign_reference)?;
            }
            _ => return Err(KsError::runtime("There is no more variable stacks!")),
        }

        self.step()?;

        Ok(())
    }

    fn load_string(&mut self) -> KsResult<String> {
        let stack = self.variable_stack.pop();

        match stack {
            Some(VariableStack::Variable(variable)) => {
                if let Value::String(string) = variable.value() {
                    Ok(string.to_string())
                } else {
                    Err(KsError::runtime("Expected string!"))
                }
            }
            Some(VariableStack::Reference(reference)) => {
                let variable = self.environment.variable(&reference)?;
                if let Value::String(string) = variable.value() {
                    Ok(string.to_string())
                } else {
                    Err(KsError::runtime("Expected string!"))
                }
            }
            _ => Err(KsError::runtime("Cannot load string from VariableStack!")),
        }
    }

    fn load_module(&mut self, size: usize) -> KsResult<()> {
        let mut module: HashMap<String, Reference> = HashMap::new();

        for _ in 0..size {
            let name = self.load_string()?;
            let stack = self.variable_stack.pop();

            match stack {
                Some(VariableStack::Variable(variable)) => {
                    let reference = self.environment.define_reference(variable)?;

                    module.insert(name, reference);
                }
                Some(VariableStack::Reference(reference)) => {
                    module.insert(name, reference);
                }
                _ => return Err(KsError::runtime("Cannot get stack!")),
            }
        }

        let module_variable = Variable::empty(Value::Module(module));
        self.variable_stack
            .push(VariableStack::Variable(module_variable));

        self.step()?;

        Ok(())
    }

    fn load_from_module(&mut self, name: String, save: bool) -> KsResult<()> {
        let stack = self.variable_stack.pop();

        match stack {
            Some(VariableStack::Variable(variable)) => {
                if let Value::Module(module) = variable.value() {
                    let reference = module.get(&name);
                    if let Some(reference) = reference {
                        if save {
                            self.tail_stack = Some(TailStack::Module {
                                name,
                                info: VarInfo::from(&variable)?,
                            });
                        }

                        self.variable_stack
                            .push(VariableStack::Reference(*reference));
                    } else {
                        return Err(KsError::runtime(&format!(
                            "Module doesn't have field {}",
                            name
                        )));
                    }
                }
            }
            Some(VariableStack::Reference(reference)) => {
                let variable = self.environment.variable(&reference)?;

                if let Value::Module(module) = variable.value() {
                    let reference = module.get(&name);
                    if let Some(reference) = reference {
                        if save {
                            self.tail_stack = Some(TailStack::Module {
                                name,
                                info: VarInfo::from(&variable)?,
                            });
                        }

                        self.variable_stack
                            .push(VariableStack::Reference(*reference));
                    } else {
                        return Err(KsError::runtime(&format!(
                            "Module doesn't have field {}",
                            name
                        )));
                    }
                }
            }
            _ => return Err(KsError::runtime("There is no more variable stacks!")),
        }

        self.step()?;
        Ok(())
    }

    fn on_end(&mut self) -> KsResult<()> {
        while let Some(variable_stack) = self.variable_stack.pop() {
            match variable_stack {
                VariableStack::Variable(variable) => {
                    let reference = self.environment.define_reference(variable)?;
                    self.environment.free(&reference)?;
                }
                VariableStack::Reference(reference) => self.environment.free(&reference)?,
            }
        }

        self.step()?;

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
            }

            Some(Instruction::Add) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;

                let value = self.add(left.value(), right.value())?;
                self.value_to_variable_stack(value);

                self.step()?;
            }

            Some(Instruction::Minus) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;

                let value = self.minus(left.value(), right.value())?;
                self.value_to_variable_stack(value);

                self.step()?;
            }

            Some(Instruction::Mul) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;

                let value = self.multiply(left.value(), right.value())?;
                self.value_to_variable_stack(value);

                self.step()?;
            }

            Some(Instruction::Div) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;

                let value = self.divide(left.value(), right.value())?;
                self.value_to_variable_stack(value);

                self.step()?;
            }

            Some(Instruction::Eq) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;

                let value = self.equal(left.value(), right.value())?;
                self.value_to_variable_stack(value);

                self.step()?;
            }

            Some(Instruction::GreaterEq) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;

                let value = self.greater_equal(left.value(), right.value())?;
                self.value_to_variable_stack(value);

                self.step()?;
            }

            Some(Instruction::Greater) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;

                let value = self.greater(left.value(), right.value())?;
                self.value_to_variable_stack(value);

                self.step()?;
            }

            Some(Instruction::LessEq) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;

                let value = self.less_equal(left.value(), right.value())?;
                self.value_to_variable_stack(value);

                self.step()?;
            }

            Some(Instruction::Less) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;

                let value = self.less(left.value(), right.value())?;
                self.value_to_variable_stack(value);

                self.step()?;
            }

            Some(Instruction::NotEq) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;

                let value = self.not_equal(left.value(), right.value())?;
                self.value_to_variable_stack(value);

                self.step()?;
            }

            Some(Instruction::And) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;

                let value = self.and(left.value(), right.value())?;
                self.value_to_variable_stack(value);

                self.step()?;
            }

            Some(Instruction::Or) => {
                let right = self.extract_variable_from_stack()?;
                let left = self.extract_variable_from_stack()?;

                let value = self.or(left.value(), right.value())?;
                self.value_to_variable_stack(value);

                self.step()?;
            }

            Some(Instruction::Not) => {
                let variable = self.extract_variable_from_stack()?;

                let value = self.not(variable.value())?;
                self.value_to_variable_stack(value);

                self.step()?;
            }

            Some(Instruction::Store(name)) => {
                let name = name.clone();
                self.define_variable(&name)?;
            }

            Some(Instruction::PubStore(name)) => {
                let name = name.clone();
                self.public_define_variable(&name)?;
            }

            Some(Instruction::Enter) => {
                self.enter_scope()?;
                self.step()?;
            }

            Some(Instruction::Exit) => {
                self.exit_scope()?;
                self.step()?;
            }

            Some(Instruction::LoadList(size)) => {
                let referneces = self.load_references_collection(*size)?;
                let variable = Variable::empty(Value::List(referneces));
                self.variable_stack.push(VariableStack::Variable(variable));

                self.step()?;
            }

            Some(Instruction::LoadTuple(size)) => {
                let references = self.load_references_collection(*size)?;
                let variable = Variable::empty(Value::Tuple(references));
                self.variable_stack.push(VariableStack::Variable(variable));

                self.step()?;
            }

            Some(Instruction::End) => self.on_end()?,
            Some(Instruction::Assign) => self.assign()?,
            Some(Instruction::LoadVar(name)) => self.load_var(name.clone(), false)?,
            Some(Instruction::LoadVarSave(name)) => self.load_var(name.clone(), true)?,
            Some(Instruction::Clone) => self.clone()?,
            Some(Instruction::LoadFromList) => self.load_from_list(false)?,
            Some(Instruction::LoadFromListSave) => self.load_from_list(true)?,
            Some(Instruction::LoadFromTuple(index)) => self.load_from_tuple(*index, false)?,
            Some(Instruction::LoadFromTupleSave(index)) => self.load_from_tuple(*index, true)?,
            Some(Instruction::ListLen) => self.list_len()?,
            Some(Instruction::LoadModule(size)) => self.load_module(*size)?,
            Some(Instruction::LoadFromModule(name)) => {
                self.load_from_module(name.clone(), false)?
            }
            Some(Instruction::LoadFromModuleSave(name)) => {
                self.load_from_module(name.clone(), true)?
            }
            Some(Instruction::Return) => self.on_return()?,
            Some(Instruction::Call(args)) => self.extract_function(args.clone())?,
            Some(Instruction::Jump(distance)) => self.jump(*distance)?,
            Some(Instruction::JumpIfFalse(distance)) => self.jump_if_false(*distance)?,

            _ => {
                self.exit_function()?;
                self.variable_stack
                    .push(VariableStack::Variable(Variable::null()));
            }
        }

        Ok(())
    }

    fn load_native(&mut self) -> KsResult<()> {
        let native = NativeRegistry::get();
        let native = native.borrow();

        for (name, native) in native.get_natives() {
            let _ = match native {
                NativeType::Function(_) => self
                    .environment
                    .define_variable(name, Variable::empty(Value::NativeFunction(name.clone())))?,
                NativeType::Int(int) => self
                    .environment
                    .define_variable(name, Variable::empty(Value::Integer(*int)))?,
                NativeType::Float(float) => self
                    .environment
                    .define_variable(name, Variable::empty(Value::Float(*float)))?,
                NativeType::Boolean(boolean) => self
                    .environment
                    .define_variable(name, Variable::empty(Value::Boolean(*boolean)))?,
                NativeType::String(string) => self
                    .environment
                    .define_variable(name, Variable::empty(Value::String(string.clone())))?,
                _ => todo!(),
            };
        }

        Ok(())
    }

    pub fn initialize(&mut self) -> KsResult<()> {
        self.enter_scope()?;
        self.load_native()?;
        self.enter_function(MAIN_FUNCTION)?;

        while self.call_stack.len() > MIN_SCOPES {
            self.interpret()?;
        }

        Ok(())
    }

    pub fn call(&mut self, name: &str) -> KsResult<Variable> {
        let function_name = &format!("{}{}", FUNCTION_ENCAPSULATION, name);

        self.enter_function(function_name)?;

        while self.call_stack.len() > MIN_SCOPES {
            self.interpret()?;
        }

        match self.variable_stack.pop() {
            Some(VariableStack::Reference(reference)) => {
                let variable = self.environment.variable(&reference)?;
                let variable = variable.clone();
                Ok(variable)
            }
            Some(VariableStack::Variable(variable)) => Ok(variable),
            None => Ok(Variable::null()),
        }
    }

    pub fn call_null(&mut self, name: &str) -> KsResult<()> {
        let function_name = &format!("{}{}", FUNCTION_ENCAPSULATION, name);

        self.enter_function(function_name)?;

        while self.call_stack.len() > MIN_SCOPES {
            self.interpret()?;
        }

        Ok(())
    }
}
