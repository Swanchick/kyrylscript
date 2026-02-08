use super::constant::Constant;
use super::types::{Offset, Pointer, VariableId};

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
    Jump(Offset),
    Store(VariableId),
    PubStore(VariableId),
    Assign,
    LoadConst(Constant),
    LoadVar(VariableId),
    LoadVarSave(VariableId),
    Closure(VariableId),
    Call(Pointer),

    // List & Tuple & Module
    LoadModule(VariableId),
    LoadList(usize),
    LoadTuple(usize),
    LoadFromList,
    LoadFromListSave,
    ListLen,
    LoadFromTuple(usize),
    LoadFromTupleSave(usize),
    LoadFromModule(VariableId),
    LoadFromModuleSave(String),
    AssignModule(String),

    // System
    Enter,
    Exit,
}
