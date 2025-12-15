use super::expression::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum IdentifierTail {
    Name(String),
    Index(Expression),
    TupleIndex(i32),
    Call(Vec<Expression>),
}
