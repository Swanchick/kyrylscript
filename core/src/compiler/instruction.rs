use crate::parser::expression::Expression;

pub enum Instruction {
    Add,
    Minus,
    Mul,
    Div,
    Store(String),
    LoadConst(Expression),
    LoadVar(String),
    Return(Option<Expression>),
    Closure(String),
    CallDynamic {
        args: i32
    },
    Call {
        function: String,
        args: i32,
    },
}