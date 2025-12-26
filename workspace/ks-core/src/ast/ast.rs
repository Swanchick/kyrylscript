use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;

use crate::lexer::token::Token;

use super::expression::Expression;

pub struct Ast {
    tokens: Vec<Token>,
    current_token: usize,
}

impl Ast {
    pub fn new(tokens: Vec<Token>) -> Ast {
        Ast {
            tokens,
            current_token: 0,
        }
    }

    pub fn expression(&mut self) -> KsResult<Expression> {
        self.parser_literal()
    }

    fn parser_literal(&mut self) -> KsResult<Expression> {
        match self.advance() {
            Ok(Token::Null) => Ok(Expression::NullLiteral),
            _ => todo!(),
        }
    }

    fn advance(&mut self) -> KsResult<&Token> {
        if let Some(token) = self.tokens.get(self.current_token) {
            self.current_token += 1;

            Ok(token)
        } else {
            Err(KsError::parse("No more tokens found!"))
        }
    }
}
