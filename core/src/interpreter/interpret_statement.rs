use std::cell::RefCell;

use std::io;
use std::rc::Rc;

use crate::interpreter::enviroment::Environment;
use crate::parser::statement::Statement;
use crate::global::data_type::DataType;

use super::interpreter::Interpreter;
use super::return_value::Return;
use super::value::{Value, ValueType};

pub struct InterpretStatement<'a> {
    interpreter: &'a mut Interpreter
}

impl<'a> InterpretStatement<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> InterpretStatement<'a> {
        InterpretStatement { interpreter: interpreter }
    }

    pub fn interpret_statement(&mut self, statement: Statement) -> io::Result<Return> {        
        match statement {
            Statement::VariableDeclaration { name, public, data_type, value } => {                                
                let value = if let Some(expression) = value {
                    self.interpreter.interpret_expression(expression)?
                } else {
                    Value::new(None, ValueType::Null)
                };

                if let Some(data_type) = data_type {
                    let value_data_type = value.get_type().get_data_type();

                    if value_data_type != data_type && !DataType::is_void(&value_data_type) {
                        return Err(io::Error::new(io::ErrorKind::InvalidData, "Variable declaration type mismatch!"))
                    }
                }

                if public {
                    self.interpreter.global_define_variable(name.as_str(), value)?;
                } else {
                    self.interpreter.define_variable(name.as_str(), value)?;
                }

                Ok(Return::Nothing)
            },
            Statement::Assignment { name, value } => {
                let value = self.interpreter.interpret_expression(value)?;

                self.interpreter.assign_variable(&name, value)?;

                Ok(Return::Nothing)
            },
            Statement::AssignmentIndex { name, index, value } => {
                let mut list_value = self.interpreter.get_variable(&name)?;
                let list_value_type = list_value.get_type_mut();
                let value_to_assign = self.interpreter.interpret_expression(value)?;

                let mut indeces: Vec<i32> = Vec::new();

                for i in index {
                    let value = self.interpreter.interpret_expression(i)?;
                    let value_type = value.get_type().clone();
                    
                    if let ValueType::Integer(i32_index) = value_type {
                        indeces.push(i32_index);
                    }
                }

                self.interpret_index_assigment(list_value_type, indeces, value_to_assign)?;
                self.interpreter.assign_variable(&name, list_value)?;

                Ok(Return::Nothing)
            },
            Statement::AddValue { name, value } => {
                let value = self.interpreter.interpret_expression(value)?;
                self.interpret_add_equal(&name, value)?;

                Ok(Return::Nothing)
            },
            Statement::RemoveValue { name, value } => {
                let value = self.interpreter.interpret_expression(value)?;
                self.interpret_minus_equal(&name, value)?;

                Ok(Return::Nothing)
            },
            Statement::ReturnStatement { value } => {
                if let Some(expression) = value {
                    Ok(Return::Success(self.interpreter.interpret_expression(expression)?))
                } else {
                    Ok(Return::Success(Value::new(None, ValueType::Null)))
                }
            },
            Statement::IfStatement { condition, body, else_body } => {
                let value = self.interpreter.interpret_expression(condition)?;
                let value_type = value.get_type().clone();
                if let ValueType::Boolean(condition) = value_type {
                    if condition {
                        self.interpreter.enter_enviroment();
                        let value = self.interpret_block(body)?;
                        self.interpreter.exit_enviroment()?;

                        if let Return::Success(value) = value {
                            return Ok(Return::Success(value));
                        }
                    } else {
                        if let Some(body) = else_body {
                            self.interpreter.enter_enviroment();
                            let value = self.interpret_block(body)?;
                            self.interpreter.exit_enviroment()?;

                            if let Return::Success(value) = value {
                                return Ok(Return::Success(value));
                            }
                        }
                    }
                    
                    Ok(Return::Nothing)
                } else {
                    Err(io::Error::new(io::ErrorKind::InvalidData, "Not boolean type in if condition"))
                }
            },
            Statement::WhileStatement { condition, body } => {
                let value = self.interpreter.interpret_expression(condition.clone())?;
                let value_type = value.get_type();

                if let ValueType::Boolean(boolean) = value_type {
                    let mut boolean = boolean.clone();
                    
                    while boolean {
                        self.interpreter.enter_enviroment();
                        let return_value = self.interpret_block(body.clone())?;
                        self.interpreter.exit_enviroment()?;

                        if let Return::Success(return_value) = return_value {
                            return Ok(Return::Success(return_value));
                        }

                        let value = self.interpreter.interpret_expression(condition.clone())?;
                        let value_type = value.get_type();
                        if let ValueType::Boolean(new_boolean) = value_type {
                            boolean = new_boolean.clone();
                        }
                    }
                }

                Ok(Return::Nothing)
            },
            Statement::ForLoopStatement { name, list, body } => {
                let list = self.interpreter.interpret_expression(list)?;
                let list_type = list.get_type();
                
                self.interpret_for_loop(name, list_type, body)?;
                
                Ok(Return::Nothing)
            },
            Statement::Expression { value } => {
                self.interpreter.interpret_expression(value)?;

                Ok(Return::Nothing)
            },
            Statement::Function { name, public, return_type, parameters, body } => {
                let mut capture = Environment::new();
                {
                    let local = self.interpreter.get_local();
                    let local = local.borrow();

                    capture = local.partially_clone();
                }
                

                let value = Value::new(None, ValueType::Function { 
                    return_type, 
                    parameters, 
                    body, 
                    capture: Rc::new(RefCell::new(capture))
                });
                
                if public {
                    self.interpreter.global_define_variable(name.as_str(), value)?;
                } else {
                    self.interpreter.define_variable(name.as_str(), value)?;
                }
                
                Ok(Return::Nothing)
            },
            Statement::EarlyReturn { name, body } => {
                let value = self.interpreter.get_variable(&name)?;

                if DataType::is_void(&value.get_data_type()) {
                    let return_data = if let Some(body) = body {
                        let return_result = self.interpret_block(body)?;

                        match return_result {
                            Return::Success(_) => return_result,
                            Return::Nothing => Return::Success(Value::new(None, ValueType::Null))
                        }
                    } else {
                        Return::Success(Value::new(None, ValueType::Null))
                    };

                    return Ok(return_data);
                }
                
                Ok(Return::Nothing)
            },

            Statement::Use { file_name, body, global: _ } => {
                let current_file = self.interpreter.source_file.clone();

                self.interpreter.source_file = file_name;
                self.interpret_block(body)?;

                self.interpreter.source_file = current_file;

                Ok(Return::Nothing)
            }
        } 
    }

    fn interpret_for_loop(&mut self, name: String, list_value: &ValueType, body: Vec<Statement>) -> io::Result<()> {
        match list_value {
            ValueType::String(str) => {
                for char in str.chars() {
                    self.interpreter.enter_enviroment();

                    self.interpreter.define_variable(name.as_str(), Value::new(None, ValueType::String(char.to_string())))?;
                    
                    self.interpret_block(body.clone())?;
                    self.interpreter.exit_enviroment()?;
                }
                
                Ok(())
            },
            ValueType::List { references, data_type: _ } => {
                for reference in references {
                    self.interpreter.enter_enviroment();
                    self.interpreter.define_variable_by_reference(name.as_str(), reference.clone())?;
                    self.interpret_block(body.clone())?;
                    self.interpreter.exit_enviroment()?;
                }

                Ok(())
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Unsupported type for loop!"))
        }
    }

    fn replace_char_at(&self, s: String, index: usize, new_char: char) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        if index >= chars.len() {
            panic!("Index out of bounds");
        }
        chars[index] = new_char;
        chars.into_iter().collect()
    }

    fn interpret_index_assigment(&mut self, list_value: &mut ValueType, indeces: Vec<i32>, value_to_assign: Value) -> io::Result<()> {
        match list_value {
            ValueType::List { references: _, data_type: _ } => {
                self.interpret_assign_list_index(list_value, indeces, value_to_assign)?;
            }
            ValueType::String(str) => {
                self.interpret_assign_string_index(str, indeces, value_to_assign)?;
            }
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid data type!"))
        }

        Ok(())
    }

    fn interpret_assign_string_index(&mut self, string: &mut String, indeces: Vec<i32>, value_to_assign: Value) -> io::Result<()> {
        if indeces.len() != 1 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "String is not a matrix or something"))
        }

        let index = indeces[0] as usize;

        let value_type = value_to_assign.get_type();

        if let ValueType::String(string_to_change) = value_type {
            if string_to_change.len() != 1 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "You have to change with a char, not with a string!"));
            }

            let chars: Vec<char> = string_to_change.chars().collect();
            let new_char: char = chars[0];

            *string = self.replace_char_at(string.clone(), index, new_char);

            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Expected string to change the char inside of the string!"))
        }

        
    }

    fn interpret_assign_list_index(&mut self, list_value: &mut ValueType, indeces: Vec<i32>, value_to_assign: Value) -> io::Result<()> {
        if let ValueType::List { references, data_type: _ } = list_value {
            let index = indeces[0] as usize;
            if index >= references.len() {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Out of index!"));
            }

            let are_we_changing_child = indeces.len() == 1;

            let child_reference = references[index];
            let mut child = self.interpreter.get_variable_reference(child_reference)?;

            if are_we_changing_child {
                if child.get_type().get_data_type() != value_to_assign.get_type().get_data_type() {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Expected the same data type!"));
                }

                self.interpreter.assign_variable_on_reference(child_reference, value_to_assign)?;


                return Ok(())
            }

            let indeces: Vec<i32> = Vec::from(&indeces[1..]);
            
            let value_type = child.get_type_mut();

            self.interpret_index_assigment(value_type, indeces, value_to_assign)?;
        }

        

        Ok(())
    }

    fn interpret_add_equal(&mut self, name: &str, value: Value) -> io::Result<()> {
        let original_value = self.interpreter.get_variable(&name)?; 
        let reference = original_value.get_reference();

        match (original_value.get_type().clone(), value.get_type().clone()) {
            (ValueType::Integer(n1), ValueType::Integer(n2)) => {
                let value = Value::new(reference, ValueType::Integer(n1 + n2));

                self.interpreter.assign_variable(name, value)?;
            },
            (ValueType::Float(n1), ValueType::Integer(n2)) => {
                let value = Value::new(reference, ValueType::Float(n1 + (n2.clone() as f64)));

                self.interpreter.assign_variable(name, value)?;
            },
            (ValueType::Integer(n1), ValueType::Float(n2)) => {
                let value = Value::new(reference, ValueType::Float((n1.clone() as f64) + n2));

                self.interpreter.assign_variable(name, value)?;
            },
            (ValueType::Float(n1), ValueType::Float(n2)) => {
                let value = Value::new(reference,  ValueType::Float(n1 + n2));

                self.interpreter.assign_variable(name, value)?;
            },
            (ValueType::String(mut str1), ValueType::String(str2)) => {
                str1.push_str(&str2);
                let value = Value::new(reference, ValueType::String(str1));

                self.interpreter.assign_variable(name, value)?;
            },
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }

        Ok(())
    }

    fn interpret_minus_equal(&mut self, name: &str, value: Value) -> io::Result<()> {
        let original_value = self.interpreter.get_variable(&name)?; 
        let reference = original_value.get_reference();

        match (original_value.get_type().clone(), value.get_type().clone()) {
            (ValueType::Integer(n1), ValueType::Integer(n2)) => {
                let value = Value::new(reference, ValueType::Integer(n1 - n2));

                self.interpreter.assign_variable(name, value)?;
            },
            (ValueType::Float(n1), ValueType::Integer(n2)) => {
                let value = Value::new(reference, ValueType::Float(n1 - (n2.clone() as f64)));

                self.interpreter.assign_variable(name, value)?;
            },
            (ValueType::Integer(n1), ValueType::Float(n2)) => {
                let value = Value::new(reference, ValueType::Float((n1.clone() as f64) - n2));

                self.interpreter.assign_variable(name, value)?;
            },
            (ValueType::Float(n1), ValueType::Float(n2)) => {
                let value = Value::new(reference,  ValueType::Float(n1 - n2));

                self.interpreter.assign_variable(name, value)?;
            },
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Different or unsupported data types!"))
        }

        Ok(())
    }

    fn interpret_block(&mut self, body: Vec<Statement>) -> io::Result<Return> {
        for statement in body {
            let value = self.interpret_statement(statement)?;

            if let Return::Success(value) = value {
                return Ok(Return::Success(value));
            }
        }

        Ok(Return::Nothing)
    }
}