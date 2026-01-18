use ks_core::lexer::lexer::Lexer;
use ks_core::parser::parser::Parser;

#[test]
fn test_variable_declatarion_with_type() {
    let source = String::from("let variable: int = 123;");
    let mut lexer = Lexer::new(source);

    lexer.lexer().unwrap();

    let mut parser = Parser::new();
    parser.set_tokens(lexer.get_tokens().to_vec(), lexer.get_token_pos().to_vec());
    parser.parse_block_statement().unwrap();
}

#[test]
fn test_variable_declatarion_with_type_error() {
    let source = String::from("let variable: float = 123;");
    let mut lexer = Lexer::new(source);

    lexer.lexer().unwrap();

    let mut parser = Parser::new();
    parser.set_tokens(lexer.get_tokens().to_vec(), lexer.get_token_pos().to_vec());
    let err = parser.parse_block_statement().unwrap_err();

    assert_eq!(
        err.message(),
        "Different data types in expression and actual data type."
    )
}

#[test]
fn test_function_environment_parameters() {
    let source = concat!(
        "function foo(bar int): int {\n",
        "    let a: int = bar;\n",
        "}\n",
    );

    let mut lexer = Lexer::new(source.to_string());

    lexer.lexer().unwrap();

    let mut parser = Parser::new();
    parser.set_tokens(lexer.get_tokens().to_vec(), lexer.get_token_pos().to_vec());
    parser.parse_block_statement().unwrap();
}

#[test]
fn test_function_environment_parameters_error() {
    let source = concat!(
        "function foo(bar float): int {\n",
        "    let a: int = bar;\n",
        "}\n",
    );

    let mut lexer = Lexer::new(source.to_string());

    lexer.lexer().unwrap();

    let mut parser = Parser::new();
    parser.set_tokens(lexer.get_tokens().to_vec(), lexer.get_token_pos().to_vec());
    assert_eq!(
        parser.parse_block_statement().unwrap_err().message(),
        "Different data types in expression and actual data type."
    );
}

#[test]
fn test_function_environment_parameters_out_of_function() {
    let source = concat!(
        "function foo(bar float): int {\n",
        "}\n",
        "let a: int = bar;"
    );

    let mut lexer = Lexer::new(source.to_string());

    lexer.lexer().unwrap();

    let mut parser = Parser::new();
    parser.set_tokens(lexer.get_tokens().to_vec(), lexer.get_token_pos().to_vec());
    assert_eq!(
        parser.parse_block_statement().unwrap_err().message(),
        "Variable bar not found!"
    );
}

#[test]
fn test_function_environment_return_mismatch() {
    let source = concat!("function foo() float {\n", "    return 100;\n", "}\n",);

    let mut lexer = Lexer::new(source.to_string());

    lexer.lexer().unwrap();

    let mut parser = Parser::new();
    parser.set_tokens(lexer.get_tokens().to_vec(), lexer.get_token_pos().to_vec());
    assert_eq!(
        parser.parse_block_statement().unwrap_err().message(),
        "Mismatch return and function return types!"
    );
}

#[test]
fn test_function_environment_if_condition_mismatch() {
    let source = concat!("if 20 + 30 {\n", "}\n",);

    let mut lexer = Lexer::new(source.to_string());

    lexer.lexer().unwrap();

    let mut parser = Parser::new();
    parser.set_tokens(lexer.get_tokens().to_vec(), lexer.get_token_pos().to_vec());
    assert_eq!(
        parser.parse_block_statement().unwrap_err().message(),
        "If statment condition mismatch data_type, expected bool!"
    );
}

#[test]
fn test_function_environment_if_environment_error() {
    let source = concat!(
        "let a = 10;\n",
        "let b = 20;\n",
        "if a == b {\n",
        "   let c = 203;\n",
        "}\n",
        "let d = c;\n",
    );

    let mut lexer = Lexer::new(source.to_string());

    lexer.lexer().unwrap();

    let mut parser = Parser::new();
    parser.set_tokens(lexer.get_tokens().to_vec(), lexer.get_token_pos().to_vec());
    assert_eq!(
        parser.parse_block_statement().unwrap_err().message(),
        "Variable c not found!"
    );
}

#[test]
fn test_function_environment_for_type_mismatch() {
    let source = concat!("let a = 20;\n", "for i in a {\n", "   let c = i;\n", "}\n",);

    let mut lexer = Lexer::new(source.to_string());

    lexer.lexer().unwrap();

    let mut parser = Parser::new();
    parser.set_tokens(lexer.get_tokens().to_vec(), lexer.get_token_pos().to_vec());
    assert_eq!(
        parser.parse_block_statement().unwrap_err().message(),
        "For loop statement mismatch type!"
    );
}

#[test]
fn test_function_environment_expression_mismatch() {
    let source = concat!("let a = \"Hello\";\n", "let b = 20;\n", "let c = a + b;\n");

    let mut lexer = Lexer::new(source.to_string());

    lexer.lexer().unwrap();

    let mut parser = Parser::new();
    parser.set_tokens(lexer.get_tokens().to_vec(), lexer.get_token_pos().to_vec());
    assert_eq!(
        parser.parse_block_statement().unwrap_err().message(),
        "Arithmetic type error!"
    );
}

#[test]
fn test_function_environment_null_error() {
    let source = concat!("let a: int = null;\n", "let b = a + 10;");

    let mut lexer = Lexer::new(source.to_string());

    lexer.lexer().unwrap();

    let mut parser = Parser::new();
    parser.set_tokens(lexer.get_tokens().to_vec(), lexer.get_token_pos().to_vec());
    assert_eq!(
        parser.parse_block_statement().unwrap_err().message(),
        "Attempt to perform an operation with a null value"
    );
}

#[test]
fn test_function_assigment_error() {
    let source = concat!("let a: int = null;\n", "a = \"Hello!\";\n");

    let mut lexer = Lexer::new(source.to_string());

    lexer.lexer().unwrap();

    let mut parser = Parser::new();
    parser.set_tokens(lexer.get_tokens().to_vec(), lexer.get_token_pos().to_vec());
    assert_eq!(
        parser.parse_block_statement().unwrap_err().message(),
        "Assignment value mismatch!"
    );
}

#[test]
fn test_function_tuple_index() {
    let source = concat!(
        "let a: ((int, bool), string) = ((123, true), \"Hello\");\n",
        "let b: string = a.0 .1;\n"
    );

    let mut lexer = Lexer::new(source.to_string());
    lexer.lexer().unwrap();

    let mut parser = Parser::new();
    parser.set_tokens(lexer.get_tokens().to_vec(), lexer.get_token_pos().to_vec());

    assert_eq!(
        parser.parse_block_statement().unwrap_err().message(),
        "Different data types in expression and actual data type."
    )
}
