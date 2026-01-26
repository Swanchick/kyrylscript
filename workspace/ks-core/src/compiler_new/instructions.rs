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

    // Statements
    End,
    Return,
    Clone,
    JumpIfFalse(i32),
    Jump(i32),
    Store(String),
    PubStore(String),
    Assign,
    LoadConst(Constant),
    LoadVar(String),
    LoadVarSave(String),
    Closure(String),
    Call(usize),

    // List & Tuple & Module
    LoadModule(usize),
    LoadList(usize),
    LoadTuple(usize),
    LoadFromList,
    LoadFromListSave,
    ListLen,
    LoadFromTuple(usize),
    LoadFromTupleSave(usize),
    LoadFromModule(String),
    LoadFromModuleSave(String),
    AssignModule(String),

    // System
    Enter,
    Exit,
}
