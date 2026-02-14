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
    let instructions: Vec<Instruction> = vec![
        Instruction::Jump(1),
        Instruction::Return,
        Instruction::LoadConst(Constant::Function(1)),
        Instruction::Store(0),
    ];

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

#[test]
fn function_call() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::Jump(4),
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::LoadConst(Constant::Integer(20)),
        Instruction::Add,
        Instruction::Return,
        Instruction::LoadConst(Constant::Function(1)),
        Instruction::Store(0),
        Instruction::LoadVar(0),
        Instruction::Call(0),
        Instruction::End,
    ];

    let mut functions = HashMap::<String, usize>::new();
    functions.insert(String::from("add"), 1);

    let test_program = Program::new(instructions, functions);

    let driver = KsDriver::new("compiler/function_call.ks");
    let program = driver.compiler_new()?;

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn function_call_with_parameters() -> KsResult<()> {
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
        Instruction::LoadVar(0), // Loading variable stored on variable_id. It's a function
        Instruction::LoadConst(Constant::Integer(10)), // Loading constant 10
        Instruction::LoadConst(Constant::Integer(20)), // Loading constant 20
        Instruction::Call(2),    // Calling function with 2 arguments stored in variable stack
        Instruction::End,        // Ending an expression
    ];

    let mut functions = HashMap::<String, usize>::new();
    functions.insert(String::from("add"), 1);

    let test_program = Program::new(instructions, functions);

    let driver = KsDriver::new("compiler/function_call_with_parameters.ks");
    let program = driver.compiler_new()?;

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn assignment_statment() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Store(0),
        Instruction::AssignVar(0),
        Instruction::LoadConst(Constant::Integer(20)),
        Instruction::Assign,
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/assignment_statment.ks");
    let program = driver.compiler_new()?;

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
#[ignore = "Broken parser, does not provide the structure, instead panics!"]
fn add_value_statment() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Store(0),
        Instruction::AssignVar(0),
        Instruction::LoadVar(0),
        Instruction::LoadConst(Constant::Integer(20)),
        Instruction::Add,
        Instruction::Assign,
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/add_value_statment.ks");
    let program = driver.compiler_new()?;

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
#[ignore = "Broken parser, does not provide the structure, instead panics!"]
fn remove_value_statment() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Store(0),
        Instruction::AssignVar(0),
        Instruction::LoadVar(0),
        Instruction::LoadConst(Constant::Integer(20)),
        Instruction::Minus,
        Instruction::Assign,
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/assignment_statment.ks");
    let program = driver.compiler_new()?;

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn if_statement() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Store(0),
        Instruction::LoadVar(0),
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Eq,
        Instruction::JumpIfFalse(5),
        Instruction::Enter,
        Instruction::AssignVar(0),
        Instruction::LoadConst(Constant::Integer(20)),
        Instruction::Assign,
        Instruction::Exit,
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/if_statement.ks");
    let program = driver.compiler_new()?;

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn if_statement_with_else() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Store(0),
        Instruction::LoadVar(0),
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Eq,
        Instruction::JumpIfFalse(6),
        Instruction::Enter,
        Instruction::AssignVar(0),
        Instruction::LoadConst(Constant::Integer(20)),
        Instruction::Assign,
        Instruction::Exit,
        Instruction::Jump(5),
        Instruction::Enter,
        Instruction::AssignVar(0),
        Instruction::LoadConst(Constant::Integer(30)),
        Instruction::Assign,
        Instruction::Exit,
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/if_statement_with_else.ks");
    let program = driver.compiler_new()?;

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn while_statement() -> KsResult<()> {
    // let instructions: Vec<Instruction> = vec![
    //     Instruction::LoadConst(Constant::Integer(0)),
    //     Instruction::Store(0),
    //     Instruction::LoadVar(0),
    //     Instruction::LoadConst(Constant::Integer(10)),
    //     Instruction::GreaterEq,
    //     Instruction::JumpIfFalse(8),
    //     Instruction::Enter,
    //     Instruction::AssignVar(0),
    //     Instruction::LoadVar(0),
    //     Instruction::LoadConst(Constant::Integer(1)),
    //     Instruction::Add,
    //     Instruction::Assign,
    //     Instruction::Exit,
    //     Instruction::Jump(-11),
    // ];

    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(0)),
        Instruction::Store(0),
        Instruction::Jump(7),
        Instruction::Enter,
        Instruction::AssignVar(0),
        Instruction::LoadVar(0),
        Instruction::LoadConst(Constant::Integer(1)),
        Instruction::Add,
        Instruction::Assign,
        Instruction::Exit,
        Instruction::LoadVar(0),
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::GreaterEq,
        Instruction::JumpIfTrue(-10),
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/while_statement.ks");
    let program = driver.compiler_new()?;

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn for_statement() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::Enter,
        Instruction::LoadConst(Constant::Integer(0)),
        Instruction::LoadConst(Constant::Integer(1)),
        Instruction::LoadConst(Constant::Integer(2)),
        Instruction::LoadList(3),
        Instruction::Store(0), // list_iter = [0, 1, 2]
        Instruction::LoadConst(Constant::Integer(0)),
        Instruction::Store(1), // iterator = 0,
        Instruction::LoadConst(Constant::Null),
        Instruction::Store(2), // our preallocated variable
        Instruction::Enter,    // Empty scope
        Instruction::AssignVar(2),
        Instruction::LoadVar(1),
        Instruction::LoadVar(0),
        Instruction::LoadFromList,
        Instruction::Assign, // changing the variable to the list number
        Instruction::Exit,   // Empty scope
        Instruction::LoadVar(1),
        Instruction::Increment, // iterator++
        Instruction::LoadVar(0),
        Instruction::ListLen,
        Instruction::GreaterEq, // iterator >= list_iter.len()
        Instruction::JumpIfFalse(-12),
        Instruction::Exit,
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/for_statement.ks");
    let program = driver.compiler_new()?;

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn list_expression() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::LoadConst(Constant::Integer(20)),
        Instruction::LoadConst(Constant::Integer(30)),
        Instruction::LoadConst(Constant::Integer(40)),
        Instruction::LoadConst(Constant::Integer(50)),
        Instruction::LoadList(5),
        Instruction::Store(0),
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/list_expression.ks");
    let program = driver.compiler_new()?;

    assert_eq!(test_program, program);

    Ok(())
}
