use crate::{
    compiler::{
        compiler::Compiler, constant::Constant, instruction::Instruction
    }, 
    global::ks_path::KsPath, 
    lexer::lexer::Lexer, 
    parser::{
        parser::Parser, 
        statement::Statement
    }
};

fn load_statments(source: &str) -> Vec<Statement> {
    let source = source.to_string();
    
    let mut lexer = Lexer::new(source);
    lexer.lexer().unwrap();

    let mut parser = Parser::new(
        lexer.get_tokens().clone(), 
        lexer.get_token_pos().clone(), 
        KsPath::new(), 
        KsPath::new()
    );

    parser.parse_block_statement().unwrap()
}


#[test]
fn simple_variable_declaration() {
    let source = concat!(
        "let a = 10;"
    );

    let statements: Vec<Statement> = load_statments(source);

    let expected_instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Store(String::from("a"))
    ];

    let mut compiler = Compiler::new();
    compiler.start_compile(&statements);

    let instructions = compiler.get_instructions("main").unwrap().get_instructions();
    assert_eq!(&expected_instructions, instructions);
}

#[test]
fn variable_declaration_with_expression() {
    let source = concat!(
        "let result = (20 + 30) / 2;"
    );

    let statements: Vec<Statement> = load_statments(source);

    let expected_instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(20)),
        Instruction::LoadConst(Constant::Integer(30)),
        Instruction::Add,
        Instruction::LoadConst(Constant::Integer(2)),
        Instruction::Div,
        Instruction::Store(String::from("result"))
    ];

    let mut compiler = Compiler::new();
    compiler.start_compile(&statements);

    let instructions = compiler.get_instructions("main").unwrap().get_instructions();
    assert_eq!(&expected_instructions, instructions);
}

