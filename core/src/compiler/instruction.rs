use super::constant::Constant;

#[derive(Debug, PartialEq)]
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
    Assign(String),
    LoadConst(Constant),
    LoadVar(String),
    Closure(String),
    Call { args: usize },
    
    // List & Tuple
    LoadList,
    LoadTuple,
    LoadFromList,
    ListLen,
    LoadFromTuple(usize),
    AssignListIndex,
    AssignTupleIndex(usize),
    
    // System
    Enter,
    Exit,
}
