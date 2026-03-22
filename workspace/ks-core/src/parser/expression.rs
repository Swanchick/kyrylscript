use std::collections::BTreeMap;

use super::data_type::DataType;
use super::identifier_tail::IdentifierTail;
use super::statement::Statement;

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
    ListLiteral(Vec<Expression>),
    TupleLiteral(Vec<Expression>),
    Module(BTreeMap<String, Expression>),
    FunctionLiteral {
        parameters: Vec<Parameter>,
        return_type: DataType,
        block: Vec<Statement>,
        captured: Vec<String>,
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
