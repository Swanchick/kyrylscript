#[derive(Debug, PartialEq)]
pub enum KsErrorType {
    Token,
    Parse,
    Type,
    RunTime,
    Native,
}
