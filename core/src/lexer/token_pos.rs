#[derive(Debug, Clone)]
pub struct TokenPos {
    source: Option<String>,
    line: i32,
}

impl TokenPos {
    pub fn from(source: Option<String>, line: i32) -> TokenPos {
        TokenPos {
            source: source,
            line: line,
        }
    }

    pub fn get_source(&self) -> &Option<String> {
        &self.source
    }

    pub fn get_line(&self) -> &i32 {
        &self.line
    }
}
