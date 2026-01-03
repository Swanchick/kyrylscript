use super::ks_error_type::KsErrorType;

#[derive(Debug)]
pub struct KsError {
    error_type: KsErrorType,
    message: String,
}

impl KsError {
    fn new(message: &str, error_type: KsErrorType) -> KsError {
        KsError {
            error_type,
            message: message.to_string(),
        }
    }

    pub fn display(&self) {
        match self.error_type {
            KsErrorType::RunTime => print!("Runtime error: "),
            KsErrorType::Native => print!("Native error: "),
            KsErrorType::Parse => print!("Parse error: "),
            KsErrorType::Token => print!("Token error: "),
            KsErrorType::Type => print!("Type error: "),
        }

        println!("{}", self.message);
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn runtime(message: &str) -> KsError {
        KsError::new(message, KsErrorType::RunTime)
    }

    pub fn token(message: &str) -> KsError {
        KsError::new(message, KsErrorType::Token)
    }

    pub fn ks_type(message: &str) -> KsError {
        KsError::new(message, KsErrorType::Type)
    }

    pub fn parse(message: &str) -> KsError {
        KsError::new(message, KsErrorType::Parse)
    }
    pub fn native(message: &str) -> KsError {
        KsError::new(message, KsErrorType::Native)
    }
}
