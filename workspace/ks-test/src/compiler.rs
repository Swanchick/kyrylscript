use std::collections::HashMap;

use ks_core::compiler_new::constant::Constant;
use ks_core::compiler_new::instructions::Instruction;
use ks_core::compiler_new::program::Program;
use ks_global::utils::ks_result::KsResult;

use crate::drivers::KsDriver;

#[test]
fn create_main_function() -> KsResult<()> {
    let output = Program::new(Vec::new(), HashMap::new());

    let driver = KsDriver::new("compiler/create_main_function.ks");
    let program = driver.compiler_new()?;

    assert_eq!(program, output);

    Ok(())
}

#[test]
fn simple_variable_declaration() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(100)),
        Instruction::Store(0),
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/simple_variable_declaration.ks");
    let program = driver.compiler_new()?;

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn expression() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::LoadConst(Constant::Integer(20)),
        Instruction::Add,
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::LoadConst(Constant::Integer(2)),
        Instruction::Minus,
        Instruction::Mul,
        Instruction::Store(0),
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/expression.ks");
    let program = driver.compiler_new()?;

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn expression_statement() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::LoadConst(Constant::Integer(20)),
        Instruction::Add,
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Add,
        Instruction::LoadConst(Constant::Integer(2)),
        Instruction::Minus,
        Instruction::End,
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/expression_statement.ks");
    let program = driver.compiler_new()?;

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn simple_identifier() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Store(0),
        Instruction::LoadVar(0),
        Instruction::Store(1),
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/simple_identifier.ks");
    let program = driver.compiler_new()?;

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn function_declaration() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::Jump(4),
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::LoadConst(Constant::Integer(20)),
        Instruction::Add,
        Instruction::Return,
        Instruction::LoadConst(Constant::Function(1)),
        Instruction::Store(0),
    ];

    let mut functions = HashMap::<String, usize>::new();
    functions.insert(String::from("add"), 1);

    let test_program = Program::new(instructions, functions);

    let driver = KsDriver::new("compiler/function_declaration.ks");
    let program = driver.compiler_new()?;

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn should_create_return_at_the_end() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![Instruction::Jump(1), Instruction::Return];

    let mut functions = HashMap::<String, usize>::new();
    functions.insert(String::from("add"), 1);

    let test_program = Program::new(instructions, functions);

    let driver = KsDriver::new("compiler/should_create_return_at_the_end.ks");
    let program = driver.compiler_new()?;

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn function_with_parameters() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::Jump(6),                          // Skiping function to store it
        Instruction::Store(2),                         // Storing parameter b
        Instruction::Store(1),                         // Storing parameter a
        Instruction::LoadVar(1),                       // Loading var a to variable stack
        Instruction::LoadVar(2),                       // Loading var b to variable stack
        Instruction::Add,                              // Sum them
        Instruction::Return,                           // And return
        Instruction::LoadConst(Constant::Function(1)), // Defining function pointer as variable and save to variable stack
        Instruction::Store(0),                         // Saving function from variable stack
    ];

    let mut functions = HashMap::<String, usize>::new();
    functions.insert(String::from("sum"), 1);

    let test_program = Program::new(instructions, functions);

    let driver = KsDriver::new("compiler/function_with_parameters.ks");
    let program = driver.compiler_new()?;

    assert_eq!(test_program, program);

    Ok(())
}
