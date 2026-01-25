use crate::drivers::KsDriver;

use ks_core::lexer::token::Token;
use ks_global::utils::ks_result::KsResult;

#[test]
fn lexer_easy() -> KsResult<()> {
    let driver = KsDriver::new("lexer/easy.ks");
    let lexer = driver.lexer()?;

    let expected_tokens = vec![
        Token::Function,
        Token::Identifier(String::from("main")),
        Token::LeftParenthesis,
        Token::RightParenthesis,
        Token::LeftBrace,
        Token::RightBrace,
    ];

    let tokens = lexer.get_tokens();
    assert_eq!(tokens, &expected_tokens);
    Ok(())
}

#[test]
fn from_file() -> KsResult<()> {
    let driver = KsDriver::new("lexer/script.ks");
    let lexer = driver.lexer()?;

    let expected_tokens = vec![
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
        Token::Identifier(String::from("println")),
        Token::LeftParenthesis,
        Token::StringLiteral(String::from("Hello World")),
        Token::RightParenthesis,
        Token::Semicolon,
        Token::RightBrace,
    ];

    let tokens = lexer.get_tokens();

    assert_eq!(tokens, &expected_tokens);

    Ok(())
}

#[test]
fn identefier_underscore() -> KsResult<()> {
    let driver = KsDriver::new("lexer/identefier_underscore.ks");
    let lexer = driver.lexer()?;

    let expected_tokens = vec![
        Token::Function,
        Token::Identifier(String::from("test_function")),
        Token::LeftParenthesis,
        Token::RightParenthesis,
        Token::LeftBrace,
        Token::RightBrace,
    ];

    let tokens = lexer.get_tokens();
    assert_eq!(tokens, &expected_tokens);

    Ok(())
}
