use super::native_function::NativeFunction;

#[derive(Debug, Clone)]
pub enum NativeTypes {
    Function(NativeFunction),
    Int(String, i32),
    Float(String, f64),
    String(String, String),
    Boolean(String, bool),
}
