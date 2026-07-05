pub struct VMError {
    pub message: String,
}

impl From<String> for VMError {
    fn from(message: String) -> Self {
        Self { message }
    }
}

impl From<&str> for VMError {
    fn from(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}
