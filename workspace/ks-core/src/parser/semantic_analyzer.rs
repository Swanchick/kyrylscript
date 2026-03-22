use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;

use super::data_type::DataType;
use crate::parser::identifier_tail::IdentifierTail;
use crate::parser::operator::Operator;

use super::analyzer_environment::AnalyzerEnvironment;
use super::expression::Expression;

#[derive(Debug, Clone)]
pub struct SemanticAnalyzer {
    global: Rc<RefCell<AnalyzerEnvironment>>,
    local: Rc<RefCell<AnalyzerEnvironment>>,
}

impl SemanticAnalyzer {
    pub fn new() -> SemanticAnalyzer {
        let global = Rc::new(RefCell::new(AnalyzerEnvironment::new()));
        let local = Rc::new(RefCell::new(AnalyzerEnvironment::with_parent(
            global.clone(),
        )));

        SemanticAnalyzer { global, local }
    }

    pub fn with_global(global: Rc<RefCell<AnalyzerEnvironment>>) -> SemanticAnalyzer {
        let local = Rc::new(RefCell::new(AnalyzerEnvironment::with_parent(
            global.clone(),
        )));

        SemanticAnalyzer {
            global: global.clone(),
            local: local,
        }
    }

    pub fn get_global(&self) -> Rc<RefCell<AnalyzerEnvironment>> {
        self.global.clone()
    }

    pub fn register_rust_function(&mut self, name: String, return_type: DataType) {
        self.local.borrow_mut().add(
            name,
            DataType::RustFunction {
                return_type: Box::new(return_type),
            },
        );
    }

    pub fn get_variable(&self, name: &str) -> KsResult<DataType> {
        let env = self.local.borrow();
        env.get_variable_type(name)
    }

    pub fn check_null(&self, data_type: &DataType) -> KsResult<()> {
        match data_type {
            DataType::Void(_) => Err(KsError::ks_type(
                "Attempt to perform an operation with a null value",
            )),
            _ => Ok(()),
        }
    }

    pub fn enter_function_environment(&mut self) {
        let parent = self.local.clone();
        self.local = Rc::new(RefCell::new(AnalyzerEnvironment::with_parent(
            parent.clone(),
        )));
    }

    pub fn exit_function_environment(&mut self) -> KsResult<()> {
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
            Err(KsError::ks_type("No parent environment!"))
        }
    }

    fn plus(&self, left: DataType, right: DataType) -> KsResult<DataType> {
        self.check_null(&left)?;
        self.check_null(&right)?;

        match (left, right) {
            (DataType::Int, DataType::Int) => Ok(DataType::Int),

            (DataType::Float, DataType::Int)
            | (DataType::Int, DataType::Float)
            | (DataType::Float, DataType::Float) => Ok(DataType::Float),

            (DataType::String, DataType::String) => Ok(DataType::String),
            _ => Err(KsError::ks_type("Arithmetic type error!")),
        }
    }

    fn arithmetic(&self, left: DataType, right: DataType) -> KsResult<DataType> {
        self.check_null(&left)?;
        self.check_null(&right)?;

        match (left, right) {
            (DataType::Int, DataType::Int) => Ok(DataType::Int),

            (DataType::Float, DataType::Int)
            | (DataType::Int, DataType::Float)
            | (DataType::Float, DataType::Float) => Ok(DataType::Float),

            _ => Err(KsError::ks_type("Arithmetic type error!")),
        }
    }

    fn division(&self, left: DataType, right: DataType) -> KsResult<DataType> {
        self.check_null(&left)?;
        self.check_null(&right)?;

        match (left, right) {
            (DataType::Int, DataType::Int)
            | (DataType::Float, DataType::Int)
            | (DataType::Int, DataType::Float)
            | (DataType::Float, DataType::Float) => Ok(DataType::Float),

            _ => Err(KsError::ks_type("Division type error!")),
        }
    }

    fn boolean(&self, left: DataType, right: DataType) -> KsResult<DataType> {
        self.check_null(&left)?;
        self.check_null(&right)?;

        match (left, right) {
            (DataType::Bool, DataType::Bool) => Ok(DataType::Bool),
            _ => Err(KsError::ks_type("Logic type error!")),
        }
    }

    fn comparison(&self, left: DataType, right: DataType) -> KsResult<DataType> {
        if left == right {
            Ok(DataType::Bool)
        } else {
            Err(KsError::ks_type("Arithmetic type error!"))
        }
    }

    fn logic(&self, left: DataType, right: DataType) -> KsResult<DataType> {
        self.check_null(&left)?;
        self.check_null(&right)?;

        match (left, right) {
            (DataType::Int, DataType::Int)
            | (DataType::Float, DataType::Int)
            | (DataType::Int, DataType::Float)
            | (DataType::Float, DataType::Float) => Ok(DataType::Bool),
            _ => Err(KsError::ks_type("Arithmetic type error!")),
        }
    }

    fn binary_operation(
        &self,
        operator: &Operator,
        left: DataType,
        right: DataType,
    ) -> KsResult<DataType> {
        match operator {
            Operator::Plus => self.plus(left, right),
            Operator::Minus | Operator::Multiply => self.arithmetic(left, right),
            Operator::Divide => self.division(left, right),
            Operator::And | Operator::Or => self.boolean(left, right),
            Operator::EqualEqual | Operator::NotEqual => self.comparison(left, right),
            Operator::GreaterEqual | Operator::Greater | Operator::LessEqual | Operator::Less => {
                self.logic(left, right)
            }

            _ => unreachable!(),
        }
    }

    fn unary_operation(&self, operator: &Operator, right: DataType) -> KsResult<DataType> {
        match (operator, right) {
            (Operator::Minus, DataType::Int) => Ok(DataType::Int),
            (Operator::Minus, DataType::Float) => Ok(DataType::Float),
            (Operator::Not, DataType::Bool) => Ok(DataType::Bool),
            _ => Err(KsError::ks_type("Invalid operator in unary operation!")),
        }
    }

    fn front_unary_operation(&self, operator: &Operator, left: DataType) -> KsResult<DataType> {
        match (operator, left) {
            (Operator::PlusPlus, DataType::Int) => Ok(DataType::Int),
            (Operator::PlusPlus, DataType::Float) => Ok(DataType::Float),
            (Operator::MinusMinus, DataType::Int) => Ok(DataType::Int),
            (Operator::MinusMinus, DataType::Float) => Ok(DataType::Float),
            (Operator::Clone, data_type) => Ok(data_type),
            _ => Err(KsError::ks_type(
                "Invalid operator in front unary operation!",
            )),
        }
    }

    pub fn get_data_type_from_segments(&self, segments: &[IdentifierTail]) -> KsResult<DataType> {
        let mut last_segment: Option<DataType> = None;

        for identifier in segments {
            match identifier {
                IdentifierTail::Name(name) => {
                    if let Some(data_type) = last_segment.clone() {
                        if let DataType::Module(module) = data_type {
                            let data_type = module.get(name);
                            if let Some(data_type) = data_type {
                                last_segment = Some(data_type.clone());
                            } else {
                                return Err(KsError::ks_type(&format!(
                                    "Cannot find {} in module!",
                                    name
                                )));
                            }
                        } else {
                            return Err(KsError::ks_type("This is not a module!"));
                        }
                    } else {
                        let data_type = self.get_variable(&name)?;
                        last_segment = Some(data_type);
                    }
                }
                IdentifierTail::Index(index) => {
                    let index_data_type = self.get_data_type(&index)?;
                    if index_data_type != DataType::Int {
                        return Err(KsError::ks_type("Invalid data type for list indexing"));
                    }

                    if let Some(DataType::List(data_type)) = last_segment {
                        last_segment = Some(*data_type);
                    }
                }
                IdentifierTail::TupleIndex(index) => {
                    if let Some(DataType::Tuple(data_types)) = &last_segment {
                        if let Some(data_type) = data_types.get(*index as usize) {
                            last_segment = Some(data_type.clone());
                        } else {
                            return Err(KsError::ks_type(&format!(
                                "Cannot access the type by tuple index {}",
                                index
                            )));
                        }
                    }
                }
                IdentifierTail::Call(call_parameters) => match last_segment {
                    Some(DataType::Function {
                        parameters,
                        return_type,
                    }) => {
                        for (call_parameter, parameter) in call_parameters.iter().zip(parameters) {
                            let call_parameter = self.get_data_type(call_parameter)?;

                            if call_parameter != parameter && !DataType::is_void(&call_parameter) {
                                return Err(KsError::ks_type("Function signature mismatch"));
                            }
                        }

                        last_segment = Some(*return_type);
                    }
                    Some(DataType::RustFunction { return_type }) => {
                        last_segment = Some(*return_type);
                    }
                    _ => {
                        return Err(KsError::ks_type("It's not a function!"));
                    }
                },
            }
        }

        if let Some(data_type) = last_segment {
            Ok(data_type)
        } else {
            Err(KsError::ks_type("Cannot get type from identifier segment!"))
        }
    }

    pub fn get_data_type(&self, expression: &Expression) -> KsResult<DataType> {
        match expression {
            Expression::BinaryOp {
                left,
                operator,
                right,
            } => {
                let left = self.get_data_type(left)?;
                let right = self.get_data_type(right)?;

                self.binary_operation(operator, left, right)
            }

            Expression::UnaryOp {
                expression,
                operator,
            } => {
                let right = self.get_data_type(expression)?;
                self.unary_operation(operator, right)
            }

            Expression::FrontUnaryOp {
                expression,
                operator,
            } => {
                let left = self.get_data_type(expression)?;
                self.front_unary_operation(operator, left)
            }

            Expression::ListLiteral(children) => {
                if children.len() == 0 {
                    return Err(KsError::ks_type("List empty!"));
                }

                let first = self.get_data_type(&children[0].clone())?;

                for child in children.iter() {
                    let child = self.get_data_type(&child.clone())?;

                    if first == child {
                        continue;
                    }

                    return Err(KsError::ks_type("Children has different types in list!"));
                }

                Ok(DataType::List(Box::new(first)))
            }

            Expression::TupleLiteral(expressions) => {
                let mut data_types: Vec<DataType> = Vec::new();

                for expression in expressions {
                    let data_type = self.get_data_type(expression)?;
                    data_types.push(data_type);
                }

                Ok(DataType::Tuple(data_types))
            }
            Expression::FunctionLiteral {
                parameters,
                return_type,
                block: _,
                captured: _,
            } => {
                let mut data_types: Vec<DataType> = Vec::new();

                for parameter in parameters {
                    data_types.push(parameter.data_type.clone());
                }

                Ok(DataType::Function {
                    parameters: data_types,
                    return_type: Box::new(return_type.clone()),
                })
            }
            Expression::Module(module) => {
                let mut module_type: BTreeMap<String, DataType> = BTreeMap::new();

                for (field_name, expression) in module {
                    let data_type = self.get_data_type(expression)?;
                    module_type.insert(field_name.clone(), data_type);
                }

                Ok(DataType::Module(module_type))
            }
            Expression::Identifier(segments) => self.get_data_type_from_segments(segments),
            Expression::IntegerLiteral(_) => Ok(DataType::Int),
            Expression::FloatLiteral(_) => Ok(DataType::Float),
            Expression::StringLiteral(_) => Ok(DataType::String),
            Expression::BooleanLiteral(_) => Ok(DataType::Bool),
            Expression::NullLiteral => Ok(DataType::void()),
        }
    }

    pub fn save_variable(&mut self, name: String, data_type: DataType) {
        let mut local = self.local.borrow_mut();
        local.add(name, data_type);
    }

    pub fn global_save_variable(&mut self, name: String, data_type: DataType) {
        self.global.borrow_mut().add(name, data_type);
    }
}
