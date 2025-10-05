use crate::parser::expression::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum IdentifierTail {
    Name(String),
    Index(Expression),
}