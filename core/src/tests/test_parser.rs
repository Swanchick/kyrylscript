use std::vec;

use crate::global::ks_path::KsPath;
use crate::parser::parameter::Parameter;
use crate::*;
use lexer::lexer::Lexer;
use lexer::token::Token;
use parser::parser::Parser;
use parser::operator::Operator;
use parser::expression::Expression;
use parser::statement::Statement;
use global::data_type::DataType;

#[test]
fn test_expression() {
    let test_expression = Expression::BinaryOp {
        left: Box::new(
            Expression::BinaryOp {
                left: Box::new(Expression::IntegerLiteral(10)),
                operator: Operator::Plus,
                right: Box::new(Expression::IntegerLiteral(20))
            }
        ),
        operator: Operator::Plus,
        right: Box::new(Expression::IntegerLiteral(30))
    };

    let tokens = vec![
        Token::IntegerLiteral(10),
        Token::Plus,
        Token::IntegerLiteral(20),
        Token::Plus,
        Token::IntegerLiteral(30)
    ];

    let mut parser = Parser::new(tokens, Vec::new(), KsPath::new(), KsPath::new());
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_complex_expression() {
    let test_expression = Expression::BinaryOp {
        left: Box::new(Expression::BinaryOp {
            left: Box::new(Expression::IntegerLiteral(3)),
            operator: Operator::Plus,
            right: Box::new(Expression::BinaryOp {
                left: Box::new(Expression::IntegerLiteral(2)),
                operator: Operator::Multiply,
                right: Box::new(Expression::IntegerLiteral(3))
            })
        }),
        operator: Operator::Minus,
        right: Box::new(Expression::IntegerLiteral(8))
    };

    // 3 + 2 * 3 - 8

    let tokens = vec![
        Token::IntegerLiteral(3),
        Token::Plus,
        Token::IntegerLiteral(2),
        Token::Multiply,
        Token::IntegerLiteral(3),
        Token::Minus,
        Token::IntegerLiteral(8)
    ];

    let mut parser = Parser::new(tokens, Vec::new(), KsPath::new(), KsPath::new());
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_complex_even_more_complex_expression() {
    let test_expression = Expression::BinaryOp {
        left: Box::new(Expression::BinaryOp {
            left: Box::new(Expression::BinaryOp {
                left: Box::new(Expression::IntegerLiteral(2)),
                operator: Operator::Plus,
                right: Box::new(Expression::IntegerLiteral(2))
            }),
            operator: Operator::Divide,
            right: Box::new(Expression::IntegerLiteral(3))
        }),
        operator: Operator::Plus,
        right: Box::new(Expression::BinaryOp {
            left: Box::new(Expression::IntegerLiteral(4)),
            operator: Operator::Multiply,
            right: Box::new(Expression::IntegerLiteral(8))
        })
    };

    // (2 + 2) / 3 + 4 * 8

    let tokens = vec![
        Token::LeftParenthesis,
        Token::IntegerLiteral(2),
        Token::Plus,
        Token::IntegerLiteral(2),
        Token::RightParenthesis,
        Token::Divide,
        Token::IntegerLiteral(3),
        Token::Plus,
        Token::IntegerLiteral(4),
        Token::Multiply,
        Token::IntegerLiteral(8)
    ];

    let mut parser = Parser::new(tokens, Vec::new(), KsPath::new(), KsPath::new());
    let expression = parser.parse_expression().unwrap();

    println!("{:?}", expression);

    assert_eq!(expression, test_expression);
}

#[test]
fn test_single_expression() {
    let test_expression = Expression::IntegerLiteral(10);

    let tokens = vec![
        Token::IntegerLiteral(10),
    ];

    let mut parser = Parser::new(tokens, Vec::new(), KsPath::new(), KsPath::new());
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_variable_declaration_statement() {
    let tokens = vec![
        Token::Let,
        Token::Identifier(String::from("a")),
        Token::Colon,
        Token::Int,
        Token::Equal,
        Token::IntegerLiteral(10),
        Token::Semicolon
    ];

    let test_statement = Statement::VariableDeclaration {
        name: String::from("a"),
        public: false,
        data_type: Some(DataType::Int),
        value: Some(Expression::IntegerLiteral(10))
    };

    let mut parser = Parser::new(tokens, Vec::new(), KsPath::new(), KsPath::new());
    let statement = parser.parse_statement().unwrap().unwrap();

    assert_eq!(statement, test_statement);
}

#[test]
fn test_expression_boolean_parse() {
    // a == 22 + 33 && b == 23 || c ~= 123

    let test_expression = Expression::BinaryOp {
        left: Box::new(Expression::BinaryOp {
            left: Box::new(Expression::BinaryOp {
                left: Box::new(Expression::Identifier(String::from("a"))),
                operator: Operator::EqualEqual,
                right: Box::new(Expression::BinaryOp {
                    left: Box::new(Expression::IntegerLiteral(22)),
                    operator: Operator::Plus,
                    right: Box::new(Expression::IntegerLiteral(33))
                })
            }),
            operator: Operator::And,
            right: Box::new(Expression::BinaryOp {
                left: Box::new(Expression::Identifier(String::from("b"))),
                operator: Operator::EqualEqual,
                right: Box::new(Expression::IntegerLiteral(23))
            })
        }),
        operator: Operator::Or,
        right: Box::new(Expression::BinaryOp {
            left: Box::new(Expression::Identifier(String::from("c"))),
            operator: Operator::NotEqual,
            right: Box::new(Expression::IntegerLiteral(123))
        })
    };

    let tokens = vec![
        Token::Identifier(String::from("a")),
        Token::EqualEqual,
        Token::IntegerLiteral(22),
        Token::Plus,
        Token::IntegerLiteral(33),
        Token::And,
        Token::Identifier(String::from("b")),
        Token::EqualEqual,
        Token::IntegerLiteral(23),
        Token::Or,
        Token::Identifier(String::from("c")),
        Token::NotEqual,
        Token::IntegerLiteral(123)
    ];


    let mut parser = Parser::new(tokens, Vec::new(), KsPath::new(), KsPath::new());
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_expression_in_parenthesis() {
    let test_expression = Expression::BinaryOp {
        left: Box::new(Expression::IntegerLiteral(2)),
        operator: Operator::Plus,
        right: Box::new(Expression::IntegerLiteral(2))
    };

    let tokens = vec![
        Token::LeftParenthesis,
        Token::IntegerLiteral(2),
        Token::Plus,
        Token::IntegerLiteral(2),
        Token::RightParenthesis
    ];

    let mut parser = Parser::new(tokens, Vec::new(), KsPath::new(), KsPath::new());
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, test_expression);
}


#[test]
fn test_assigment_statement() {
    let source = concat!(
        "let a = \"Hey Hey\";\n",
        "a = \"Hello World\";\n",
    );

    let test_statement = Statement::Assignment {
        name: String::from("a"),
        value: Expression::StringLiteral(String::from("Hello World"))
    };

    let mut lexer = Lexer::new(source.to_string());
    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone(), Vec::new(), KsPath::new(), KsPath::new());
    let _ = parser.parse_statement().unwrap(); // Parsing first line
    let statement = parser.parse_statement().unwrap().unwrap(); // Then second, to ensure that value type is actually type-correct 

    assert_eq!(statement, test_statement);
}

#[test]
fn test_function_call_statement() {
    let source = "print(add(20, 10), 20);";

    let mut lexer = Lexer::new(source.to_string());
    lexer.lexer().unwrap();

    let test_expression = Expression::FunctionCall(String::from("print"), vec![
        Expression::FunctionCall(String::from("add"), vec![Expression::IntegerLiteral(20), Expression::IntegerLiteral(10)]),
        Expression::IntegerLiteral(20)
    ]);

    let mut parser = Parser::new(lexer.get_tokens().clone(), Vec::new(), KsPath::new(), KsPath::new());
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, test_expression);
}


#[test]
fn test_parser_front_unary_op() {
    let mut lexer = Lexer::new(String::from("100 - i++"));
    lexer.lexer().unwrap();

    let test_expression = Expression::BinaryOp {
        left: Box::new(Expression::IntegerLiteral(100)),
        operator: Operator::Minus,
        right: Box::new(Expression::FrontUnaryOp {
            expression: Box::new(Expression::Identifier(String::from("i"))),
            operator: Operator::PlusPlus
        })
    };

    let mut parser = Parser::new(lexer.get_tokens().clone(), Vec::new(), KsPath::new(), KsPath::new());
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_parser_list_expression() {
    let mut lexer = Lexer::new(String::from("[100 + 20, \"Hello\", 230]"));
    lexer.lexer().unwrap();

    let test_expression = Expression::ListLiteral(vec![
        Expression::BinaryOp { left: Box::new(Expression::IntegerLiteral(100)), operator: Operator::Plus, right: Box::new(Expression::IntegerLiteral(20)) },
        Expression::StringLiteral(String::from("Hello")),
        Expression::IntegerLiteral(230)
    ]);

    let mut parser = Parser::new(lexer.get_tokens().clone(), Vec::new(), KsPath::new(), KsPath::new());
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, test_expression);
}


#[test]
fn test_parser_list_index_1() {
    let mut lexer = Lexer::new(String::from("some_list[10]"));
    lexer.lexer().unwrap();

    let test_expression = Expression::ListIndex {
        left: Box::new(Expression::Identifier(String::from("some_list"))),
        index: Box::new(Expression::IntegerLiteral(10))
    };

    let mut parser = Parser::new(lexer.get_tokens().clone(), Vec::new(), KsPath::new(), KsPath::new());
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_parser_list_index_2() {
    let mut lexer = Lexer::new(String::from("[10, 10, 10, 20, 40, 50, 40][2]"));
    lexer.lexer().unwrap();

    let test_expression = Expression::ListIndex {
        left: Box::new(Expression::ListLiteral(vec![
            Expression::IntegerLiteral(10),
            Expression::IntegerLiteral(10),
            Expression::IntegerLiteral(10),
            Expression::IntegerLiteral(20),
            Expression::IntegerLiteral(40),
            Expression::IntegerLiteral(50),
            Expression::IntegerLiteral(40),
        ])),
        index: Box::new(Expression::IntegerLiteral(2))
    };

    let mut parser = Parser::new(lexer.get_tokens().clone(), Vec::new(), KsPath::new(), KsPath::new());
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_parser_list_index_3() {
    let mut lexer = Lexer::new(String::from("[[10, 20], [20, 10]][1][0]"));
    lexer.lexer().unwrap();

    let test_expression = Expression::ListIndex {
        left: Box::new(Expression::ListIndex {
            left: Box::new(Expression::ListLiteral(vec![
                Expression::ListLiteral(vec![
                    Expression::IntegerLiteral(10),
                    Expression::IntegerLiteral(20),
                ]),
                Expression::ListLiteral(vec![
                    Expression::IntegerLiteral(20),
                    Expression::IntegerLiteral(10),
                ]),
            ])),
            index: Box::new(Expression::IntegerLiteral(1))
        }),
        index: Box::new(Expression::IntegerLiteral(0))
    };

    let mut parser = Parser::new(lexer.get_tokens().clone(), Vec::new(), KsPath::new(), KsPath::new());
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_parser_string_index_1() {
    let mut lexer = Lexer::new(String::from("\"Hello worlda asdasd asd\"[10]"));
    lexer.lexer().unwrap();

    let test_expression = Expression::ListIndex {
        left: Box::new(Expression::StringLiteral(String::from("Hello worlda asdasd asd"))),
        index: Box::new(Expression::IntegerLiteral(10))
    };

    let mut parser = Parser::new(lexer.get_tokens().clone(), Vec::new(), KsPath::new(), KsPath::new());
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, test_expression);
}

#[test]
fn test_parser_index_assingment_statment() {
    let mut lexer = Lexer::new(String::from("some_list[10][20] = 20;"));
    lexer.lexer().unwrap();

    let test_statement = Statement::AssignmentIndex { 
        name: String::from("some_list"), 
        index: vec![Expression::IntegerLiteral(10), Expression::IntegerLiteral(20)],
        value: Expression::IntegerLiteral(20)
    };

    let mut parser = Parser::new(lexer.get_tokens().clone(), Vec::new(), KsPath::new(), KsPath::new());
    let statement = parser.parse_statement().unwrap().unwrap();

    assert_eq!(statement, test_statement);
}

#[test]
fn test_parser_tuple() {
    let test_expression = Expression::TupleLiteral(vec![Expression::IntegerLiteral(10), Expression::StringLiteral(String::from("Kurwa"))]);

    let mut lexer = Lexer::new(String::from("(10, \"Kurwa\")"));
    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone(), Vec::new(), KsPath::new(), KsPath::new());
    let expression = parser.parse_expression().unwrap();

    assert_eq!(expression, test_expression)
}


#[test]
fn test_parser_callback() {
    let source = concat!(
        "let test = function() {};\n",
    );

    let test_statement = Statement::VariableDeclaration { 
        name: String::from("test"),
        public: false,
        data_type: None, 
        value:  Some(Expression::FunctionLiteral { parameters: Vec::new(), return_type: DataType::void(), block: Vec::new() })
    };

    let mut lexer = Lexer::new(source.to_string());
    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone(), Vec::new(), KsPath::new(), KsPath::new());
    let statement = parser.parse_statement().unwrap().unwrap();

    assert_eq!(statement, test_statement); 
}

#[test]
fn test_parser_callback_with_type() {
    let source = concat!(
        "let number_32 = function(): int {\n",
        "   return 32;\n",
        "};\n",
    );

    let test_statement = Statement::VariableDeclaration { 
        name: String::from("number_32"), 
        public: false,
        data_type: None, 
        value:  Some(Expression::FunctionLiteral { parameters: Vec::new(), return_type: DataType::Int, block: vec![
            Statement::ReturnStatement { value: Some(Expression::IntegerLiteral(32)) }
        ] })
    };

    let mut lexer = Lexer::new(source.to_string());
    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone(), Vec::new(), KsPath::new(), KsPath::new());
    let statement = parser.parse_statement().unwrap().unwrap();

    assert_eq!(statement, test_statement); 
}


#[test]
fn test_parser_callback_with_parameters_and_type() {
    let source = concat!(
        "let sum = function(a: int, b: int): int {\n",
        "   return a + b;\n",
        "};\n",
    );

    let test_statement = Statement::VariableDeclaration { 
        name: String::from("sum"), 
        public: false,
        data_type: None, 
        value:  Some(Expression::FunctionLiteral { parameters: vec![Parameter {name: String::from("a"), data_type: DataType::Int}, Parameter {name: String::from("b"), data_type: DataType::Int}], return_type: DataType::Int, block: vec![
            Statement::ReturnStatement { 
                value: Some(Expression::BinaryOp { 
                    left: Box::new(Expression::Identifier(String::from("a"))), 
                    operator: Operator::Plus, 
                    right: Box::new(Expression::Identifier(String::from("b"))) 
                }) 
            }
        ]})
    };

    let mut lexer = Lexer::new(source.to_string());
    lexer.lexer().unwrap();

    let mut parser = Parser::new(lexer.get_tokens().clone(), Vec::new(), KsPath::new(), KsPath::new());
    let statement = parser.parse_statement().unwrap().unwrap();

    assert_eq!(statement, test_statement); 
}
