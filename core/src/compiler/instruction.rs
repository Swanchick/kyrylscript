use super::constant::Constant;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Add,
    Minus,
    Mul,
    Div,
    Return,
    Store(String),
    LoadConst(Constant),
    LoadVar(String),
    Closure(String),
    CallDynamic { args: i32 },
    Call { function: String, args: i32 },
}
