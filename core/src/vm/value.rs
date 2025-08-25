#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Integer(i32),
    Float(f64),
    String(String),
    Boolean(bool),
    List(Vec<u64>),
    Tuple(Vec<u64>),
    Function(String),
    NativeFunction(String)
}