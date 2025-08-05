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
fn compiler_simple_variable_declaration() {
    let source = concat!(
        "let a = 10;"
    );

    let statements: Vec<Statement> = load_statments(source);

    let expected_instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Store(String::from("a"))
    ];

    let compiler = Compiler::new(statements);
    let instruction = compiler.compile_statments();

    assert_eq!(expected_instructions, instruction);
}

#[test]
fn compiler_variable_declaration_with_expression() {
    let source = concat!(
        "let result = (20 + 30) / 2;"
    );

    let statements: Vec<Statement> = load_statments(source);

    let expected_instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(20)),
        Instruction::LoadConst(Constant::Integer(30)),
        Instruction::Add,
        Instruction::LoadConst(Constant::Integer(2)),
        Instruction::Store(String::from("result"))
    ];

    let compiler = Compiler::new(statements);
    let instruction = compiler.compile_statments();

    assert_eq!(expected_instructions, instruction);
}