use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;

use crate::lexer::token::Token;

use super::expression::Expression;
use super::identifier_tail::IdentifierTail;

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
        self.parse_literal()
    }

    fn parse_literal(&mut self) -> KsResult<Expression> {
        match self.advance() {
            Ok(Token::Null) => Ok(Expression::NullLiteral),
            Ok(Token::IntegerLiteral(integer)) => Ok(Expression::IntegerLiteral(*integer)),
            Ok(Token::FloatLiteral(float)) => Ok(Expression::FloatLiteral(*float)),
            Ok(Token::True) => Ok(Expression::BooleanLiteral(true)),
            Ok(Token::False) => Ok(Expression::BooleanLiteral(false)),
            Ok(Token::StringLiteral(string)) => Ok(Expression::StringLiteral(string.to_owned())),
            Ok(Token::Identifier(_)) => {
                self.back();
                let identifier = self.parse_identifier()?;
                Ok(Expression::Identifier(identifier))
            }
            _ => todo!(),
        }
    }

    fn parse_identifier(&mut self) -> KsResult<Vec<IdentifierTail>> {
        let mut segments = Vec::<IdentifierTail>::new();

        loop {
            match self.advance() {
                Ok(Token::Identifier(name)) => segments.push(IdentifierTail::Name(name.to_owned())),
                Ok(Token::Dot) => {}
                Ok(Token::LeftSquareBracket) => {
                    let expression = self.expression()?;
                    self.consume_token(Token::RightSquareBracket)?;

                    segments.push(IdentifierTail::Index(expression));
                }
                _ => break,
            }
        }

        Ok(segments)
    }

    fn back(&mut self) {
        self.current_token = self.current_token.saturating_sub(1);
    }

    fn advance(&mut self) -> KsResult<&Token> {
        if let Some(token) = self.tokens.get(self.current_token) {
            self.current_token += 1;

            Ok(token)
        } else {
            Err(KsError::parse("No more tokens found!"))
        }
    }

    fn peek(&mut self) -> KsResult<&Token> {
        if let Some(token) = self.tokens.get(self.current_token) {
            Ok(token)
        } else {
            Err(KsError::parse("No more tokens found!"))
        }
    }

    fn consume_token(&mut self, expected_token: Token) -> KsResult<()> {
        let token_to_compare = self.peek()?;

        if *token_to_compare == expected_token {
            Ok(())
        } else {
            Err(KsError::parse(&format!(
                "Expected token: {:?} got {:?}!",
                expected_token, token_to_compare
            )))
        }
    }
}
