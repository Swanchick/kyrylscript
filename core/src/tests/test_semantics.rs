use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::global::ks_path::KsPath;


#[test]
fn test_variable_declatarion_with_type() {
    let source = String::from("let variable: int = 123;");
    let mut lexer = Lexer::new(source);

    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone(), lexer.get_token_pos().clone(), KsPath::new(), KsPath::new());
    parser.parse_block_statement().unwrap();
}


#[test]
fn test_variable_declatarion_with_type_error() {
    let source = String::from("let variable: float = 123;");
    let mut lexer = Lexer::new(source);

    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone(), lexer.get_token_pos().clone(), KsPath::new(), KsPath::new());
    let err = parser.parse_block_statement().unwrap_err();


    assert_eq!(err.to_string(), "Different data types in expression and actual data type.")
}


#[test]
fn test_function_enviroment_parameters() {
    let source = concat!(
        "function foo(bar: int): int {\n",
        "    let a: int = bar;\n",
        "}\n",
    );

    let mut lexer = Lexer::new(source.to_string());

    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone(), lexer.get_token_pos().clone(), KsPath::new(), KsPath::new());
    parser.parse_block_statement().unwrap();
}


#[test]
fn test_function_enviroment_parameters_error() {
    let source = concat!(
        "function foo(bar: float): int {\n",
        "    let a: int = bar;\n",
        "}\n",
    );

    let mut lexer = Lexer::new(source.to_string());

    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone(), lexer.get_token_pos().clone(), KsPath::new(), KsPath::new());
    assert_eq!(parser.parse_block_statement().unwrap_err().to_string(), "Different data types in expression and actual data type.");
}

#[test]
fn test_function_enviroment_parameters_out_of_function() {
    let source = concat!(
        "function foo(bar: float): int {\n",
        "}\n",
        "let a: int = bar;"
    );

    let mut lexer = Lexer::new(source.to_string());

    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone(), lexer.get_token_pos().clone(), KsPath::new(), KsPath::new());
    assert_eq!(parser.parse_block_statement().unwrap_err().to_string(), "Variable bar not found!");
}

#[test]
fn test_function_enviroment_return_mismatch() {
    let source = concat!(
        "function foo(): float {\n",
        "    return 100;\n",
        "}\n",
    );

    let mut lexer = Lexer::new(source.to_string());

    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone(), lexer.get_token_pos().clone(), KsPath::new(), KsPath::new());
    assert_eq!(parser.parse_block_statement().unwrap_err().to_string(), "Mismatch return and function return types!");
}


#[test]
fn test_function_enviroment_if_condition_mismatch() {
    let source = concat!(
        "if 20 + 30 {\n",
        "}\n",
    );

    let mut lexer = Lexer::new(source.to_string());

    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone(), lexer.get_token_pos().clone(), KsPath::new(), KsPath::new());
    assert_eq!(parser.parse_block_statement().unwrap_err().to_string(), "If statment condition mismatch data_type, expected bool!");
}


#[test]
fn test_function_enviroment_if_enviroment_error() {
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

    let mut parser = Parser::new(lexer.get_tokens().clone(), lexer.get_token_pos().clone(), KsPath::new(), KsPath::new());
    assert_eq!(parser.parse_block_statement().unwrap_err().to_string(), "Variable c not found!");
}


#[test]
fn test_function_enviroment_for_type_mismatch() {
    let source = concat!(
        "let a = 20;\n",
        "for i in a {\n",
        "   let c = i;\n",
        "}\n",
    );

    let mut lexer = Lexer::new(source.to_string());

    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone(), lexer.get_token_pos().clone(), KsPath::new(), KsPath::new());
    assert_eq!(parser.parse_block_statement().unwrap_err().to_string(), "For loop statement mismatch type!");
}

#[test]
fn test_function_enviroment_expression_mismatch() {
    let source = concat!(
        "let a = \"Hello\";\n",
        "let b = 20;\n",
        "let c = a + b;\n"
    );

    let mut lexer = Lexer::new(source.to_string());

    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone(), lexer.get_token_pos().clone(), KsPath::new(), KsPath::new());
    assert_eq!(parser.parse_block_statement().unwrap_err().to_string(), "Arithmetic type error!");
}

#[test]
fn test_function_enviroment_null_error() {
    let source = concat!(
        "let a: int = null;\n",
        "let b = a + 10;"
    );

    let mut lexer = Lexer::new(source.to_string());

    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone(), lexer.get_token_pos().clone(), KsPath::new(), KsPath::new());
    assert_eq!(parser.parse_block_statement().unwrap_err().to_string(), "Attempt to perform an operation with a null value");
}



#[test]
fn test_function_assigment_error() {
    let source = concat!(
        "let a: int = null;\n",
        "a = \"Hello!\";\n"
    );

    let mut lexer = Lexer::new(source.to_string());

    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone(), lexer.get_token_pos().clone(), KsPath::new(), KsPath::new());
    assert_eq!(parser.parse_block_statement().unwrap_err().to_string(), "Assignment value mismatch!");
}


#[test]
fn test_function_tuple_index() {    
    let source = concat!(
        "let a: ((int, bool), string) = ((123, true), \"Hello\");\n",
        "let b: string = a.0 .1;\n"
    );

    let mut lexer = Lexer::new(source.to_string());
    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone(), lexer.get_token_pos().clone(), KsPath::new(), KsPath::new());

    assert_eq!(parser.parse_block_statement().unwrap_err().to_string(), "Different data types in expression and actual data type.")
}

