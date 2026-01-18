use ks_core::parser::data_type::DataType;
use ks_core::parser::expression::Expression;
use ks_core::parser::identifier_tail::IdentifierTail;
use ks_core::parser::operator::Operator;
use ks_core::parser::parser::Parser;
use ks_core::parser::statement::Statement;
use ks_global::utils::ks_result::KsResult;

use crate::drivers::KsDriver;

#[test]
fn expression() -> KsResult<()> {
    let driver = KsDriver::new("parser/expression.ks");
    let statements = driver.parser()?;
    let statement = statements.get(0);

    let test_statement_expression = Some(&Statement::Expression {
        value: Expression::BinaryOp {
            left: Box::new(Expression::BinaryOp {
                left: Box::new(Expression::IntegerLiteral(10)),
                operator: Operator::Plus,
                right: Box::new(Expression::IntegerLiteral(20)),
            }),
            operator: Operator::Plus,
            right: Box::new(Expression::IntegerLiteral(30)),
        },
    });

    assert_eq!(statement, test_statement_expression);

    Ok(())
}

#[test]
fn complex_expression() -> KsResult<()> {
    let driver = KsDriver::new("parser/complex_expression.ks");
    let statements = driver.parser()?;
    let statement = statements.get(0);

    let test_statement_expression = Some(&Statement::Expression {
        value: Expression::BinaryOp {
            left: Box::new(Expression::BinaryOp {
                left: Box::new(Expression::IntegerLiteral(3)),
                operator: Operator::Plus,
                right: Box::new(Expression::BinaryOp {
                    left: Box::new(Expression::IntegerLiteral(2)),
                    operator: Operator::Multiply,
                    right: Box::new(Expression::IntegerLiteral(3)),
                }),
            }),
            operator: Operator::Minus,
            right: Box::new(Expression::IntegerLiteral(8)),
        },
    });

    assert_eq!(statement, test_statement_expression);

    Ok(())
}

#[test]
fn brackets_expression() -> KsResult<()> {
    let driver = KsDriver::new("parser/brackets_expression.ks");
    let statements = driver.parser()?;
    let statement = statements.get(0);

    let test_expression = Some(&Statement::Expression {
        value: Expression::BinaryOp {
            left: Box::new(Expression::BinaryOp {
                left: Box::new(Expression::BinaryOp {
                    left: Box::new(Expression::IntegerLiteral(2)),
                    operator: Operator::Plus,
                    right: Box::new(Expression::IntegerLiteral(2)),
                }),
                operator: Operator::Divide,
                right: Box::new(Expression::IntegerLiteral(3)),
            }),
            operator: Operator::Plus,
            right: Box::new(Expression::BinaryOp {
                left: Box::new(Expression::IntegerLiteral(4)),
                operator: Operator::Multiply,
                right: Box::new(Expression::IntegerLiteral(8)),
            }),
        },
    });

    assert_eq!(statement, test_expression);

    Ok(())
}

#[test]
fn variable_declaration() -> KsResult<()> {
    let driver = KsDriver::new("parser/variable_declaration.ks");
    let statements = driver.parser()?;
    let statement = statements.get(0);

    let test_statement = Some(&Statement::VariableDeclaration {
        name: String::from("number"),
        public: false,
        data_type: Some(DataType::Int),
        value: Some(Expression::IntegerLiteral(10)),
    });

    assert_eq!(statement, test_statement);

    Ok(())
}

#[test]
fn assigment_statement() -> KsResult<()> {
    let driver = KsDriver::new("parser/assigment_statement.ks");
    let statements = driver.parser()?;

    let test_statements = vec![
        Statement::VariableDeclaration {
            name: String::from("example_string"),
            public: false,
            data_type: Some(DataType::String),
            value: Some(Expression::StringLiteral(String::from("Hello, world!"))),
        },
        Statement::Assignment {
            segments: vec![IdentifierTail::Name(String::from("example_string"))],
            value: Expression::StringLiteral(String::from("Another string")),
        },
    ];

    assert_eq!(statements, test_statements);

    Ok(())
}

#[test]
fn list_index() -> KsResult<()> {
    let driver = KsDriver::new("parser/list_index.ks");
    let statements = driver.parser()?;

    let test_statements = vec![Statement::VariableDeclaration {
        name: String::from("some_list"),
        public: false,
        data_type: Some(DataType::String),
        value: Some(Expression::ListLiteral(vec![
            Expression::IntegerLiteral(10),
            Expression::IntegerLiteral(20),
        ])),
    }];

    assert_eq!(statements, test_statements);

    Ok(())
}

#[test]
fn tuple() -> KsResult<()> {
    let driver = KsDriver::new("parser/tuple.ks");
    let statements = driver.parser()?;
    let statement = statements.get(0);

    let test_statement = Some(&Statement::Expression {
        value: Expression::TupleLiteral(vec![
            Expression::IntegerLiteral(10),
            Expression::StringLiteral(String::from("Kurwa")),
        ]),
    });

    assert_eq!(statement, test_statement);

    Ok(())
}

#[test]
fn callback() -> KsResult<()> {
    let driver = KsDriver::new("parser/tuple.ks");
    let statements = driver.parser()?;
    let statement = statements.get(0);

    let test_statement = Some(&Statement::VariableDeclaration {
        name: String::from("test_callback"),
        public: false,
        data_type: None,
        value: Some(Expression::FunctionLiteral {
            parameters: Vec::new(),
            return_type: DataType::void(),
            block: Vec::new(),
        }),
    });

    assert_eq!(statement, test_statement);

    Ok(())
}
