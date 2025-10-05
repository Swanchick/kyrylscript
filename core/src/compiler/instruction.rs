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
    Closure(String),
    Call { args: usize },
    
    // List & Tuple & Module
    LoadList(usize),
    LoadTuple(usize),
    LoadFromList,
    ListLen,
    LoadFromTuple(usize),
    AssignListIndex,
    AssignTupleIndex(usize),
    LoadFromModule(String),
    AssignModule(String),
    
    // System
    Enter,
    Exit,
}
