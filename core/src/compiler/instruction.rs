use super::constant::Constant;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    // Math
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
    
    
    // List
    LoadList,
    LoadTuple,
    LoadFromList,
    LoadFromTuple(usize),
    AssignListIndex,
    AssignTupleIndex(usize),
    
    // Statements
    End,
    Return,
    Clone,
    JumpIfFalse(usize),
    Store(String),
    Assign(String),
    LoadConst(Constant),
    LoadVar(String),
    Closure(String),
    Call { args: usize },
}
