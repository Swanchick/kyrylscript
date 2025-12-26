use crate::ast::ast::Ast;
use crate::ast::expression::Expression;
use crate::ast::identifier_tail::IdentifierTail;
use crate::ast::operator::Operator;
use crate::lexer::lexer::Lexer;
use crate::lexer::token::Token;

fn string_to_tokens(source: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(source.to_string());
    lexer.lexer().unwrap();
    lexer.get_tokens().clone()
}

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
fn test_expression_identifier_simple() {
    let test_expression =
        Expression::Identifier(vec![IdentifierTail::Name(String::from("variable_name"))]);

    let tokens = vec![Token::Identifier(String::from("variable_name"))];

    let mut ast = Ast::new(tokens);
    let expression = ast.expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_expression_identifier_identifier_call_empty() {
    let test_expression = Expression::Identifier(vec![
        IdentifierTail::Name(String::from("function_name")),
        IdentifierTail::Call(Vec::new()),
    ]);

    let tokens = string_to_tokens("function_name()");

    let mut ast = Ast::new(tokens);
    let expression = ast.expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_expression_identifier_identifier_call() {
    let test_expression = Expression::Identifier(vec![
        IdentifierTail::Name(String::from("function_name")),
        IdentifierTail::Call(Vec::new()),
    ]);

    let tokens = string_to_tokens("function_name(arg1, arg2)");

    let mut ast = Ast::new(tokens);
    let expression = ast.expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_expression_identifier_module_access() {
    let test_expression = Expression::Identifier(vec![
        IdentifierTail::Name(String::from("person")),
        IdentifierTail::Name(String::from("name")),
    ]);

    let tokens = string_to_tokens("person.name");

    let mut ast = Ast::new(tokens);
    let expression = ast.expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_expression_identifier_list_index() {
    let test_expression = Expression::Identifier(vec![
        IdentifierTail::Name(String::from("my_list")),
        IdentifierTail::Index(Expression::IntegerLiteral(10)),
    ]);

    let tokens = string_to_tokens("my_list[10]");

    let mut ast = Ast::new(tokens);
    let expression = ast.expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_expression_identifier_tuple_index() {
    let test_expression = Expression::Identifier(vec![
        IdentifierTail::Name(String::from("my_tuple")),
        IdentifierTail::TupleIndex(10),
    ]);

    let tokens = string_to_tokens("my_tuple->10");

    let mut ast = Ast::new(tokens);
    let expression = ast.expression().unwrap();

    assert_eq!(expression, test_expression);
}
