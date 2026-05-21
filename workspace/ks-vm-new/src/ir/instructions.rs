use crate::types::{ArgumentSize, CaptureSize, Offset, VariableId};

use super::constant::Constant;

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

    ClearAcc,
    Return,
    Free(usize),
    JumpIfFalse(Offset),
    JumpIfTrue(Offset),
    Jump(Offset),
    Store,
    PubStore,
    Assign,
    LoadConst(Constant),
    LoadVar(VariableId),
    Closure(VariableId),
    Call(ArgumentSize),
    AssignVar(VariableId),
    LoadCapture(VariableId),
    LoadFunction(CaptureSize),

    LoadCollection(usize),
    LoadFromCollection,
    AssignCollection,
    CollectionLen,
}
