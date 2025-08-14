use std::cell::RefCell;
use std::rc::Rc;
use std::io;

use crate::native_registry::native_function::NativeFunction;
use crate::parser::operator::Operator;
use crate::global::data_type::DataType;

use super::analyzer_enviroment::AnalyzerEnviroment;
use super::expression::Expression;

#[derive(Debug, Clone)]
pub struct SemanticAnalyzer {
    global: Rc<RefCell<AnalyzerEnviroment>>,
    local: Rc<RefCell<AnalyzerEnviroment>>
}


impl SemanticAnalyzer {
    pub fn new() -> SemanticAnalyzer {
        let global = Rc::new(RefCell::new(AnalyzerEnviroment::new()));
        let local = Rc::new(RefCell::new(AnalyzerEnviroment::with_parent(global.clone())));

        SemanticAnalyzer {
            global,
            local
        }
    }

    pub fn with_global(global: Rc<RefCell<AnalyzerEnviroment>>) -> SemanticAnalyzer {
        let local = Rc::new(RefCell::new(AnalyzerEnviroment::with_parent(global.clone())));
        
        SemanticAnalyzer { 
            global: global.clone(), 
            local: local
        }
    }

    pub fn get_global(&self) -> Rc<RefCell<AnalyzerEnviroment>> {
        self.global.clone()
    } 

    pub fn register_rust_function(&mut self, name: String, function: &NativeFunction) {
        self.local.borrow_mut().add(name, DataType::RustFunction { return_type: Box::new(function.return_type.clone()) });
    }

    pub fn get_variable(&self, name: &str) -> io::Result<DataType> {
        let env = self.local.borrow();
        env.get_variable_type(name)
    }

    pub fn check_null(&self, data_type: &DataType) -> io::Result<()> {
        match data_type {
            DataType::Void(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "Attempt to perform an operation with a null value")),
            _ => Ok(())
        }
    }

    pub fn enter_function_enviroment(&mut self) {
        let parent = self.local.clone();
        self.local = Rc::new(RefCell::new(AnalyzerEnviroment::with_parent(parent.clone())));
    }

    pub fn exit_function_enviroment(&mut self) -> io::Result<()> {
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
            self.local = env;
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "No parent enviroment!"))
        }
    }

    fn plus(&self, left: DataType, right: DataType) -> io::Result<DataType> {
        self.check_null(&left)?;
        self.check_null(&right)?;
        
        match (left, right) {
            (DataType::Int, DataType::Int) => Ok(DataType::Int),
            
            (DataType::Float, DataType::Int)
            | (DataType::Int, DataType::Float)
            | (DataType::Float, DataType::Float) => Ok(DataType::Float),
            
            (DataType::String, DataType::String) => Ok(DataType::String),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Arithmetic type error!"))
        }
    }

    fn arithmetic(&self, left: DataType, right: DataType) -> io::Result<DataType> {
        self.check_null(&left)?;
        self.check_null(&right)?;

        match (left, right) {
            (DataType::Int, DataType::Int) => Ok(DataType::Int),
            
            (DataType::Float, DataType::Int)
            | (DataType::Int, DataType::Float)
            | (DataType::Float, DataType::Float) => Ok(DataType::Float),
            
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Arithmetic type error!"))
        }
    }

    fn division(&self, left: DataType, right: DataType) -> io::Result<DataType> {
        self.check_null(&left)?;
        self.check_null(&right)?;

        match (left, right) {
            (DataType::Int, DataType::Int)
            | (DataType::Float, DataType::Int)
            | (DataType::Int, DataType::Float)
            | (DataType::Float, DataType::Float) => Ok(DataType::Float),
            
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Division type error!"))
        }
    }

    fn boolean(&self, left: DataType, right: DataType) -> io::Result<DataType> {
        self.check_null(&left)?;
        self.check_null(&right)?;

        match (left, right) {
            (DataType::Bool, DataType::Bool) => Ok(DataType::Bool),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Logic type error!"))
        }
    }

    fn comparison(&self, left: DataType, right: DataType) -> io::Result<DataType> {
        if left == right {
            Ok(DataType::Bool)
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Arithmetic type error!"))
        }
    }

    fn logic(&self, left: DataType, right: DataType) -> io::Result<DataType> {
        self.check_null(&left)?;
        self.check_null(&right)?;

        match (left, right) {
            (DataType::Int, DataType::Int) 
            | (DataType::Float, DataType::Int)
            | (DataType::Int, DataType::Float)
            | (DataType::Float, DataType::Float) => Ok(DataType::Bool),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Arithmetic type error!"))
        }
    }
    
    fn binary_operation(&self, operator: &Operator, left: DataType, right: DataType) -> io::Result<DataType> {
        match operator {
            Operator::Plus => self.plus(left, right),
            
            Operator::Minus
            | Operator::Multiply => self.arithmetic(left, right),
            
            Operator::Divide => self.division(left, right),
            
            Operator::And
            | Operator::Or => self.boolean(left, right),
            
            Operator::EqualEqual
            | Operator::NotEqual => self.comparison(left, right),
            
            Operator::GreaterEqual
            | Operator::Greater
            | Operator::LessEqual
            | Operator::Less => self.logic(left, right),
            
            _ => unreachable!()
        }
    }

    fn unary_operation(&self, operator: &Operator, right: DataType) -> io::Result<DataType> {
        match (operator, right) {
            (Operator::Minus, DataType::Int) => Ok(DataType::Int),
            (Operator::Minus, DataType::Float) => Ok(DataType::Float),
            (Operator::Not, DataType::Bool) => Ok(DataType::Bool),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid operator in unary operation!"))
        }
    }

    fn front_unary_operation(&self, operator: &Operator, left: DataType) -> io::Result<DataType> {
        match (operator, left) {
            (Operator::PlusPlus, DataType::Int) => Ok(DataType::Int),
            (Operator::PlusPlus, DataType::Float) => Ok(DataType::Float),
            (Operator::MinusMinus, DataType::Int) => Ok(DataType::Int),
            (Operator::MinusMinus, DataType::Float) => Ok(DataType::Float),
            (Operator::Clone, data_type) => Ok(data_type),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid operator in front unary operation!"))
        }
    }

    fn identefier_index(&self, left: DataType, index: DataType) -> io::Result<DataType> {
        match (left, index) {
            (DataType::List(children_type), DataType::Int) => Ok(*children_type),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid data in list indexing operation!"))
        }
    }

    fn tuple_index(&self, mut left: DataType, indeces: &Vec<i32>) -> io::Result<DataType> {
        for index in indeces {
            let index = *index as usize;

            if let DataType::Tuple(children) = &left {
                if index > children.len() {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Tuple out of index!"))
                }
                
                left = children[index].clone();
            } else {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid data in tuple indexing operation!"));
            }
        }

        Ok(left)
    }

    pub fn get_data_type(&self, expression: &Expression) -> io::Result<DataType> {
        match expression {
            Expression::BinaryOp { left, operator, right } => {
                let left = self.get_data_type(left)?;
                let right = self.get_data_type(right)?;

                self.binary_operation(operator, left, right)
            },

            Expression::UnaryOp { expression, operator } => {
                let right = self.get_data_type(expression)?;
                self.unary_operation(operator, right)
            },

            Expression::FrontUnaryOp { expression, operator } => {
                let left = self.get_data_type(expression)?;
                self.front_unary_operation(operator, left)
            },

            Expression::ListLiteral(children) => {
                if children.len() == 0 {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "List empty!"));
                }

                let first = self.get_data_type(&children[0].clone())?;

                for child in children.iter() {
                    let child = self.get_data_type(&child.clone())?;

                    if first == child {
                        continue;
                    }

                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Children has different types in list!"));
                }

                Ok(DataType::List(Box::new(first)))
            },

            Expression::TupleIndex { left, indeces } => {
                let left = self.get_data_type(&left)?;

                self.tuple_index(left, indeces)
            },
            
            Expression::Identifier(name) => {
                match self.local.borrow().get_variable_type(name) {
                    Ok(DataType::Void(_)) => Ok(DataType::void()),
                    Ok(data_type) => Ok(data_type.clone()),
                    Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, format!("Variable {} not found!", name)))
                }
            },

            Expression::FunctionCall(name, call_parameters) => {
                let function = self.get_variable(name)?;

                match function {
                    DataType::RustFunction { return_type } => {
                        Ok(*return_type)
                    }

                    DataType::Function { parameters, return_type } => {
                        for (call_parameter, parameter) in call_parameters.iter().zip(parameters) {
                            let call_parameter = self.get_data_type(call_parameter)?;

                            if call_parameter != parameter && !DataType::is_void(&call_parameter) {
                                return Err(io::Error::new(io::ErrorKind::InvalidData, "Function signature mismatch"));
                            }
                        }

                        Ok(*return_type)
                    }

                    DataType::Void(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "Ти далбайоб?")),
                    _ => Err(io::Error::new(io::ErrorKind::InvalidData, format!("Function {} not found!", name)))
                }
            },

            Expression::ListIndex { left, index } => {
                let left = self.get_data_type(left)?;
                let index_type = self.get_data_type(index)?;

                self.identefier_index(left, index_type)
            },

            Expression::TupleLiteral(expressions) => {
                let mut data_types: Vec<DataType> = Vec::new();

                for expression in expressions {
                    let data_type = self.get_data_type(expression)?;
                    data_types.push(data_type);
                }

                Ok(DataType::Tuple(data_types))
            },
            Expression::FunctionLiteral { parameters, return_type, block: _ } => {
                let mut data_types: Vec<DataType> = Vec::new();

                for parameter in parameters {
                    data_types.push(parameter.data_type.clone());
                }
                
                Ok(DataType::Function { parameters: data_types, return_type: Box::new(return_type.clone()) })
            },
            Expression::IntegerLiteral(_) => Ok(DataType::Int),
            Expression::FloatLiteral(_) => Ok(DataType::Float),
            Expression::StringLiteral(_) => Ok(DataType::String),
            Expression::BooleanLiteral(_) => Ok(DataType::Bool),
            Expression::NullLiteral => Ok(DataType::void())
        }
    }

    pub fn save_variable(&mut self, name: String, data_type: DataType) {
        self.local.borrow_mut().add(name, data_type);
    }

    pub fn global_save_variable(&mut self, name: String, data_type: DataType) {
        self.global.borrow_mut().add(name, data_type);
    }
}