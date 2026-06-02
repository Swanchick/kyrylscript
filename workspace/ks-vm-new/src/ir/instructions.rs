use crate::types::{CaptureSize, Offset, VariableId};

use super::constant::Constant;

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
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
    Call,
    LoadCapture(VariableId),
    LoadFunction(CaptureSize),

    LoadCollection(usize),
    LoadFromCollection,
    CollectionLen,
}
