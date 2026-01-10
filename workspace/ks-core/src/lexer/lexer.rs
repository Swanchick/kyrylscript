use std::fs::read_to_string;

use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;

use super::token::COMMENT;
use super::token::Token;
use super::token::{get_token, is_symbol};

use super::lexer_state::LexerState;
use super::token_pos::TokenPos;

pub struct Lexer {
    tokens: Vec<Token>,
    token_pos: Vec<TokenPos>,
    source_lines: Vec<String>,
    source_path: Option<String>,
    current_line_pos: i32,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        let source_lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();

        Lexer {
            tokens: Vec::new(),
            token_pos: Vec::new(),
            source_lines: source_lines,
            source_path: None,
            current_line_pos: 0,
        }
    }

    pub fn load(source_path: &str) -> KsResult<Lexer> {
        let result = read_to_string(source_path);

        match result {
            Ok(source) => {
                let source_lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();

                Ok(Lexer {
                    tokens: Vec::new(),
                    token_pos: Vec::new(),
                    source_lines: source_lines,
                    source_path: Some(source_path.to_string()),
                    current_line_pos: 0,
                })
            }
            Err(_) => Err(KsError::parse(&format!(
                "Cannot find file with that path: {source_path}!"
            ))),
        }
    }

    pub fn get_tokens(&self) -> &[Token] {
        &self.tokens
    }

    pub fn get_token_pos(&self) -> &[TokenPos] {
        &self.token_pos
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);

        let token_pos = TokenPos::from(self.source_path.clone(), self.current_line_pos);
        self.token_pos.push(token_pos);
    }

    fn add_token_text(&mut self, buffer: &str) {
        if let Some(keyword) = get_token(buffer) {
            self.add_token(keyword);
        } else {
            self.add_token(Token::Identifier(buffer.to_string()));
        }
    }

    pub fn lex_line(&mut self, mut line: String) -> KsResult<()> {
        line.push(' ');
        let mut cur: usize = 0;
        let mut state = LexerState::None;

        let mut buffer = String::new();

        while cur < line.len() {
            let current_char = line.chars().nth(cur).unwrap();

            match state {
                LexerState::None => {
                    if current_char.is_whitespace() {
                        if buffer.len() > 0 {
                            self.add_token_text(&buffer);
                            buffer.clear();
                        }
                    } else if current_char.is_alphabetic() {
                        state = LexerState::Identifier;
                        buffer.push(current_char);
                    } else if current_char.is_numeric() {
                        state = LexerState::Number;
                        buffer.push(current_char);
                    } else if current_char == '"' {
                        state = LexerState::String;
                    } else if is_symbol(current_char) {
                        state = LexerState::Symbol;
                        buffer.push(current_char);
                    }
                }

                LexerState::String => {
                    if current_char == '"' {
                        self.add_token(Token::StringLiteral(buffer.clone()));
                        buffer.clear();
                        state = LexerState::None;
                    } else {
                        buffer.push(current_char);
                    }
                }

                LexerState::Number => {
                    if current_char.is_numeric() || current_char == '.' {
                        buffer.push(current_char);
                    } else if current_char == 'f' {
                        if let Ok(num) = buffer.parse::<f64>() {
                            self.add_token(Token::FloatLiteral(num));
                            buffer.clear();
                            state = LexerState::None;
                        } else {
                            return Err(KsError::token("Invalid float literal"));
                        }
                    } else {
                        if let Ok(num) = buffer.parse::<i32>() {
                            self.add_token(Token::IntegerLiteral(num));
                            buffer.clear();
                            state = LexerState::None;

                            continue;
                        } else {
                            return Err(KsError::token("Invalid integer literal"));
                        }
                    }
                }

                LexerState::Identifier => {
                    if current_char.is_alphabetic()
                        || current_char.is_numeric()
                        || current_char == '_'
                    {
                        buffer.push(current_char);
                    } else {
                        self.add_token_text(buffer.as_str());
                        buffer.clear();
                        state = LexerState::None;

                        continue;
                    }
                }

                LexerState::Symbol => {
                    if is_symbol(current_char) && cur != line.len() {
                        buffer.push(current_char);
                    } else {
                        // Checking if the symbol is a comment and if it is indeed then break the loop to proceed to the next line
                        // easy
                        if buffer.contains(COMMENT) {
                            break;
                        }

                        self.get_symbols(&buffer);
                        buffer.clear();
                        state = LexerState::None;

                        continue;
                    }
                }
            }

            cur += 1;
        }

        Ok(())
    }

    fn get_symbols(&mut self, buffer: &str) {
        let chars: Vec<char> = buffer.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let mut matched = false;

            for j in (i + 1..=chars.len()).rev() {
                let slice: String = chars[i..j].iter().collect();

                if let Some(token) = get_token(&slice) {
                    self.add_token(token);
                    i = j;
                    matched = true;
                    break;
                }
            }

            if !matched {
                let single = chars[i].to_string();
                if let Some(token) = get_token(&single) {
                    self.add_token(token);
                }

                i += 1;
            }
        }
    }

    pub fn lexer(&mut self) -> KsResult<()> {
        for line in self.source_lines.clone() {
            let line = line.clone();

            self.lex_line(line)?;

            self.current_line_pos += 1;
        }

        Ok(())
    }
}
