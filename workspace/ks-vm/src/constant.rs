#[derive(Debug, PartialEq, Clone)]
pub enum Constant {
    Integer(i32),
    Float(f64),
    String(String),
    Boolean(bool),
    Function(usize),
    Null,
}
