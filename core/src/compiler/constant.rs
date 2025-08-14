#[derive(Debug, PartialEq)]
pub enum Constant {
    Integer(i32),
    Float(f64),
    String(String),
    Boolean(bool),
    Function(String),
    Null,
}
