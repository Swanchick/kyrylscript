use super::ks_error_type::KsErrorType;

pub struct KsError {
    error_type: KsErrorType,
    message: String
} 

impl KsError {
    fn new(message: &str, error_type: KsErrorType) -> KsError {
        KsError { 
            error_type, 
            message: message.to_string() 
        }
    }
    
    pub fn run_time(message: &str) -> KsError {
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
}
