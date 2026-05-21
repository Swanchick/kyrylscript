#[derive(Debug, PartialEq, Clone)]
pub enum Constant {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
}
