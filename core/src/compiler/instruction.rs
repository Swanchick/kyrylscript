use super::constant::Constant;

#[derive(Debug, PartialEq)]
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
    Return,
    LoadList,
    And,
    Or,
    Not,
    LoadTuple,
    Store(String),
    LoadConst(Constant),
    LoadVar(String),
    Closure(String),
    CallDynamic { args: i32 },
    Call { function: String, args: usize },
}
