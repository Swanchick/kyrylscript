use crate::types::Stack;

#[derive(Debug, PartialEq)]
pub enum Collection {
    String(String),
    Stack(Stack),
}
