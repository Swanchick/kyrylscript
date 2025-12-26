use crate::ast::ast::Ast;
use crate::ast::expression::Expression;
use crate::ast::identifier_tail::IdentifierTail;
use crate::ast::operator::Operator;
use crate::lexer::token::Token;

#[test]
fn test_expression_null_literal() {
    let test_expression = Expression::NullLiteral;
    let tokens = vec![Token::Null];

    let mut ast = Ast::new(tokens);
    let expression = ast.expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_expression_int_literal() {
    let test_expression = Expression::IntegerLiteral(123432);
    let tokens = vec![Token::IntegerLiteral(123432)];

    let mut ast = Ast::new(tokens);
    let expression = ast.expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_expression_float_literal() {
    let test_expression = Expression::FloatLiteral(8392.238);
    let tokens = vec![Token::FloatLiteral(8392.238)];

    let mut ast = Ast::new(tokens);
    let expression = ast.expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_expression_string_literal() {
    let test_expression = Expression::StringLiteral(String::from("Something here"));
    let tokens = vec![Token::StringLiteral(String::from("Something here"))];

    let mut ast = Ast::new(tokens);
    let expression = ast.expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_expression_boolean_literal_true() {
    let test_expression = Expression::BooleanLiteral(true);
    let tokens = vec![Token::True];

    let mut ast = Ast::new(tokens);
    let expression = ast.expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_expression_boolean_literal_false() {
    let test_expression = Expression::BooleanLiteral(false);
    let tokens = vec![Token::False];

    let mut ast = Ast::new(tokens);
    let expression = ast.expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_expression_identifier() {
    let test_expression =
        Expression::Identifier(vec![IdentifierTail::Name(String::from("variable_name"))]);

    let tokens = vec![Token::Identifier(String::from("variable_name"))];

    let mut ast = Ast::new(tokens);
    let expression = ast.expression().unwrap();

    assert_eq!(expression, test_expression);
}
