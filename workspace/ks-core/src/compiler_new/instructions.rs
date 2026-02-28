use super::constant::Constant;
use super::types::{ArgumentSize, Offset, VariableId};

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    // Expressions
    Add,
    Minus,
    Mul,
    Div,
    Eq,
    GreaterEq,
    Greater,
    LessEq,
    Less,
    NotEq,
    And,
    Or,
    Not,
    Increment,
    Decrement,
    Clone,
    Power,

    // Statements
    End,
    Return,
    JumpIfFalse(Offset),
    JumpIfTrue(Offset),
    Jump(Offset),
    Store(VariableId),
    PubStore(VariableId),
    Assign,
    LoadConst(Constant),
    LoadVar(VariableId),
    Closure(VariableId),
    Call(ArgumentSize),
    AssignVar(VariableId),

    LoadCollection(usize),
    LoadFromCollection,
    AssignCollection,
    CollectionLen,

    // System
    Enter,
    Exit,
}
