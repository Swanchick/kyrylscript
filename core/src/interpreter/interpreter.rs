use std::rc::Rc;
use std::cell::RefCell;
use std::io;

use crate::native_registry::native_registry::NativeRegistry;
use crate::native_registry::native_types::NativeTypes;
use crate::global::data_type::DataType;
use crate::parser::expression::Expression;
use crate::parser::parameter::Parameter;
use crate::parser::statement::Statement;

use super::enviroment::Environment;
use super::interpret_expression::InterpretExpression;
use super::interpret_statement::InterpretStatement;
use super::return_value::Return;
use super::value::{Value, ValueType};

#[derive(Debug)]
pub struct Interpreter {
    global: Rc<RefCell<Environment>>,
    local: Rc<RefCell<Environment>>,
    pub source_file: String
}

impl Interpreter {
    pub fn new(global: Rc<RefCell<Environment>>) -> Interpreter {
        let local = Rc::new(RefCell::new(Environment::with_parent(global.clone())));
        
        let registry = NativeRegistry::get();
        {
            let mut registry = registry.borrow_mut();
            if let None = registry.global {
                registry.global = Some(global.clone());
            }

            registry.local = Some(local.clone());
            let mut env = local.borrow_mut();

            for (name, native) in registry.get_natives() {
                match native {
                    NativeTypes::Function(function) => {
                        let _ = env.define_variable(name.clone(), Value::new(
                            None, 
                            ValueType::RustFucntion { return_type: function.return_type.clone() }
                        ));
                    }
                }
            }
        }

        Interpreter {
            global: global.clone(),
            local: local,
            source_file: String::new()
        }
    }

    pub fn empty() -> Interpreter {
        let local = Rc::new(RefCell::new(Environment::new()));
        
        Interpreter {
            global: local.clone(),
            local: local,
            source_file: String::new()
        }
    }

    pub fn get_local(&self) -> Rc<RefCell<Environment>> {
        self.local.clone()
    }

    pub fn get_global(&self) -> Rc<RefCell<Environment>> {
        self.global.clone()
    }

    pub fn create_reference(&mut self, reference: u64) {
        let mut local = self.local.borrow_mut();
        local.create_reference(reference);
    }

    pub fn create_value(&mut self, value: Value) -> u64 {
        let mut local = self.local.borrow_mut();
        local.create_value_without_name(value)
    }

    pub fn get_variable(&self, name: &str) -> io::Result<Value> {
        let local = self.local.borrow();
        local.get_variable(name)
    }

    pub fn get_variable_reference(&self, reference: u64) -> io::Result<Value> {
        let local = self.local.borrow();

        local.get_by_reference(reference)
    }

    pub fn define_variable(&mut self, name: &str, value: Value) -> io::Result<()> {
        let mut local = self.local.borrow_mut();

        local.define_variable(name.to_string(), value);
        
        Ok(())
    }

    pub fn global_define_variable(&mut self, name: &str, value: Value) -> io::Result<()> {
        let mut global = self.global.borrow_mut();
        
        global.define_variable(name.to_string(), value);
        
        Ok(())
    }

    pub fn define_variable_by_reference(&mut self, name: &str, reference: u64) -> io::Result<()> {
        let mut local = self.local.borrow_mut();

        local.create_value_reference(name.to_string(), reference);
        
        Ok(())
    }

    pub fn assign_variable(&mut self, name: &str, value: Value) -> io::Result<()> {
        let mut local = self.local.borrow_mut();

        local.assign_variable(name, value)?;

        Ok(())
    }

    pub fn assign_variable_on_reference(&mut self, reference: u64, value: Value) -> io::Result<()> {
        let mut local = self.local.borrow_mut();

        local.assign_variable_on_reference(reference, value)?;
        Ok(())
    }

    pub fn same_scope(&self, reference: u64) -> bool {
        let local = self.local.borrow();

        local.same_scope_reference(reference)
    } 

    pub fn variable_exists(&self, reference: u64) -> bool {
        let local = self.local.borrow();

        local.variable_exists(reference)
    }

    pub fn enter_enviroment(&mut self) {
        let previous = self.local.clone();
        let new_local = Rc::new(RefCell::new(Environment::with_parent(previous)));
        
        let native = NativeRegistry::get();
        {
            let mut native = native.borrow_mut();

            native.local = Some(new_local.clone());
        }

        self.local = new_local;
    }

    pub fn exit_enviroment(&mut self) -> io::Result<()> {
        let new_env = {
            let local = self.local.clone();
            let local_borrow = local.borrow();

            if let Some(parent) = local_borrow.get_parent() {
                Some(parent.clone())
            } else {
                None
            }
        };

        if let Some(env) = new_env {            
            let native = NativeRegistry::get();
            {
                let mut native = native.borrow_mut();

                native.local = Some(env.clone());
            }
            
            self.local = env;
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "No parent enviroment!"))
        }
    }

    pub fn interpret_statements(&mut self, statements: Vec<Statement>) -> io::Result<Return> {
        for statement in statements {
            let result = self.interpret_statement(statement)?;

            if let Return::Success(_) = &result {
                return Ok(result);
            }
        }

        Ok(Return::Nothing)
    }

    pub fn interpret_statement(&mut self, statement: Statement) -> io::Result<Return> {
        let mut interpret_statement = InterpretStatement::new(self);

        interpret_statement.interpret_statement(statement)
    }

    pub fn interpret_expression(&mut self, expression: Expression) -> io::Result<Value> {
        let mut interpret_expression = InterpretExpression::new(self);

        interpret_expression.interpret_expression(expression)
    }

    fn move_to_parent(&mut self, value: Value) {
        let mut local = self.local.borrow_mut();

        local.move_to_parent(value);
    }

    fn append_environment(&mut self, env: Rc<RefCell<Environment>>) {
        let mut local = self.local.borrow_mut();
        local.append_environment(env.clone());
    }

    pub fn call_native_function(&self, name: &str, args: Vec<Value>) -> io::Result<Value> {
        let registry = NativeRegistry::get();
        let registry = registry.borrow();
        let native = registry.get_native(name);

        if let Some(NativeTypes::Function(native_function)) = native {
            // (native_function.function)(args.clone())
            todo!()
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, format!("Variable {} is not a function!", name)))
        }
    }

    pub fn call_internal_function(
        &mut self,
        name: &str,
        args: Vec<Value>, 
        parameters: &Vec<Parameter>, 
        body: &Vec<Statement>, 
        capture: Rc<RefCell<Environment>>,
        return_type: &DataType
    ) -> io::Result<Value> {
        if args.len() != parameters.len() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Missmatch in function's singature \"{}\"!", name)));
        }

        self.enter_enviroment();

        self.append_environment(capture.clone());

        for (arg, parameter) in args.iter().zip(parameters) {
            if arg.get_type().get_data_type() != parameter.data_type && !DataType::is_void(&arg.get_data_type()) {
                return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Missmatch in function's singature \"{}\"!", name)));
            }

            if let Some(reference) = arg.get_reference() {
                self.define_variable_by_reference(parameter.name.as_str(), reference)?;
            } else {
                self.define_variable(parameter.name.as_str(), arg.clone())?;
            }
        }

        let result = self.interpret_statements(body.to_vec())?;
        
        match result {
            Return::Success(mut value) => {
                if let ValueType::List { references, data_type: _ } | ValueType::Tuple { references, data_types: _ } = value.get_type() {
                    for reference in references {
                        let list_value = self.get_variable_reference(*reference)?;
                        self.move_to_parent(list_value);
                    }
                }
                
                if let Some(reference) = value.get_reference() {
                    if self.same_scope(reference) {
                        value.clear_reference();
                    }
                }
                
                self.exit_enviroment()?;

                if value.get_data_type() != *return_type {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData, 
                        format!("Different return types in {} ({} != {})", name, return_type, value.get_data_type())
                    ));
                } 
                
                Ok(value)
            },
            Return::Nothing => {
                self.exit_enviroment()?;
                Ok(Value::new(None, ValueType::Null))
            }
        }
    }

    pub fn call_function(&mut self, name: &str, args: Vec<Value>) -> io::Result<Value> {        
        let value = self.get_variable(name)?;

        match value.get_type() {
            ValueType::Function { return_type, parameters, body, capture } => {
                self.call_internal_function(name, args, parameters, body, capture.clone(), return_type)
            },
            ValueType::RustFucntion { return_type: _ } => {
                self.call_native_function(name, args)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, format!("Variable {} is not a function!", name)))
        }

        
    }
}
