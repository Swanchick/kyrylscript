use crate::*;
use lexer::lexer::Lexer;
use lexer::token::Token;

#[test]
fn test_lexer_easy() {
    let source = concat!("function main() {\n", "}\n");

    let mut lexer = Lexer::new(source.to_string());
    lexer.lexer().unwrap();

    let expected_tokens: Vec<Token> = vec![
        Token::Function,
        Token::Identifier(String::from("main")),
        Token::LeftParenthesis,
        Token::RightParenthesis,
        Token::LeftBrace,
        Token::RightBrace,
    ];

    let tokens = lexer.get_tokens();

    assert_eq!(tokens, &expected_tokens);
}

#[test]
fn test_lexer_from_file() {
    let source = concat!(
        "function main() {\n",
        "    let value: float = 10.2f;\n",
        "    let value2: int = 10;\n",
        "    print(\"Hello World\");\n",
        "}\n"
    );

    let mut lexer = Lexer::new(source.to_string());
    lexer.lexer().unwrap();

    let expected_tokens: Vec<Token> = vec![
        Token::Function,
        Token::Identifier(String::from("main")),
        Token::LeftParenthesis,
        Token::RightParenthesis,
        Token::LeftBrace,
        Token::Let,
        Token::Identifier(String::from("value")),
        Token::Colon,
        Token::Float,
        Token::Equal,
        Token::FloatLiteral(10.2),
        Token::Semicolon,
        Token::Let,
        Token::Identifier(String::from("value2")),
        Token::Colon,
        Token::Int,
        Token::Equal,
        Token::IntegerLiteral(10),
        Token::Semicolon,
        Token::Identifier(String::from("print")),
        Token::LeftParenthesis,
        Token::StringLiteral(String::from("Hello World")),
        Token::RightParenthesis,
        Token::Semicolon,
        Token::RightBrace,
    ];

    let tokens = lexer.get_tokens();

    assert_eq!(tokens, &expected_tokens);
}

#[test]
fn test_lexer_identefier_underscore() {
    let source = concat!("function test_function() {\n", "}\n");

    let mut lexer = Lexer::new(source.to_string());
    lexer.lexer().unwrap();

    let expected_tokens: Vec<Token> = vec![
        Token::Function,
        Token::Identifier(String::from("test_function")),
        Token::LeftParenthesis,
        Token::RightParenthesis,
        Token::LeftBrace,
        Token::RightBrace,
    ];

    let tokens = lexer.get_tokens();

    assert_eq!(tokens, &expected_tokens);
}
