use crate::global::data_type::DataType;
use crate::parser::statement::Statement;

use super::parameter::Parameter;

use super::operator::Operator;

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    NullLiteral,
    IntegerLiteral(i32),  
    FloatLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    Identifier(String),
    FunctionCall(String, Vec<Expression>),
    ListLiteral(Vec<Expression>),
    TupleLiteral(Vec<Expression>),
    FunctionLiteral { 
        parameters: Vec<Parameter>, 
        return_type: DataType, 
        block: Vec<Statement>
    },
    ListIndex {
        left: Box<Expression>,
        index: Box<Expression>
    },
    TupleIndex {
        left: Box<Expression>,
        indeces: Vec<i32>
    },
    BinaryOp {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>
    },
    UnaryOp {
        expression: Box<Expression>,
        operator: Operator
    },
    FrontUnaryOp {
        expression: Box<Expression>,
        operator: Operator
    },
}