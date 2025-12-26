use std::collections::HashMap;

use super::identifier_tail::IdentifierTail;
use super::statement::Statement;
use super::data_type::DataType;

use super::parameter::Parameter;

use super::operator::Operator;

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    NullLiteral,
    IntegerLiteral(i32),
    FloatLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    Identifier(Vec<IdentifierTail>),
    FunctionCall(String, Vec<Expression>),
    ListLiteral(Vec<Expression>),
    TupleLiteral(Vec<Expression>),
    Module(HashMap<String, Expression>),
    FunctionLiteral {
        parameters: Vec<Parameter>,
        return_type: DataType,
        block: Vec<Statement>,
    },
    ListIndex {
        left: Box<Expression>,
        index: Box<Expression>,
    },
    TupleIndex {
        left: Box<Expression>,
        indeces: Vec<i32>,
    },
    BinaryOp {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    UnaryOp {
        expression: Box<Expression>,
        operator: Operator,
    },
    FrontUnaryOp {
        expression: Box<Expression>,
        operator: Operator,
    },
}
