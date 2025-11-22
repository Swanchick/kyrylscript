#[derive(Debug)]
pub enum LexerState {
    None,
    String,
    Number,
    Identifier,
    Symbol,
}
