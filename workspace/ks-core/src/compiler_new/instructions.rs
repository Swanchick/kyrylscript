use super::constant::Constant;
use super::types::{FunctionPointer, Pointer, VariableId};

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

    // Statements
    End,
    Return,
    Clone,
    JumpIfFalse(Pointer),
    Jump(Pointer),
    Store(VariableId),
    PubStore(VariableId),
    Assign,
    LoadConst(Constant),
    LoadVar(VariableId),
    LoadVarSave(VariableId),
    Closure(VariableId),
    Call(FunctionPointer),

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
