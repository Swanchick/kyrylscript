use std::collections::HashMap;

use ks_global::utils::ks_result::KsResult;
use ks_vm_new::Constant;
use ks_vm_new::Instruction;
use ks_vm_new::Program;

use crate::drivers::KsDriver;

#[test]
fn create_main_function() -> KsResult<()> {
    let output = Program::new(Vec::new(), HashMap::new());

    let driver = KsDriver::new("compiler/create_main_function.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(program, output);

    Ok(())
}

#[test]
fn simple_variable_declaration() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(100)),
        Instruction::Store,
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/simple_variable_declaration.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

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
        Instruction::Store,
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/expression.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

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
        Instruction::ClearAcc,
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/expression_statement.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn simple_identifier() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Store,
        Instruction::LoadVar(0),
        Instruction::Store,
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/simple_identifier.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

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
        Instruction::LoadConst(Constant::Integer(1)),
        Instruction::LoadFunction(0),
        Instruction::Store,
    ];

    let mut functions = HashMap::<String, usize>::new();
    functions.insert(String::from("add"), 1);

    let test_program = Program::new(instructions, functions);

    let driver = KsDriver::new("compiler/function_declaration.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn should_create_return_at_the_end() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::Jump(1),
        Instruction::Return,
        Instruction::LoadConst(Constant::Integer(1)),
        Instruction::LoadFunction(0),
        Instruction::Store,
    ];

    let mut functions = HashMap::<String, usize>::new();
    functions.insert(String::from("add"), 1);

    let test_program = Program::new(instructions, functions);

    let driver = KsDriver::new("compiler/should_create_return_at_the_end.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn function_with_parameters() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::Jump(7),                         // Skiping function to store it
        Instruction::Store,                           // Storing parameter a
        Instruction::Store,                           // Storing parameter b
        Instruction::LoadVar(0),                      // Loading var a to variable stack
        Instruction::LoadVar(1),                      // Loading var b to variable stack
        Instruction::Add,                             // Sum them
        Instruction::Free(2),                         // Free local variables
        Instruction::Return,                          // And return
        Instruction::LoadConst(Constant::Integer(1)), // Defining function pointer
        Instruction::LoadFunction(0),                 // Save function with function pointer
        Instruction::Store,                           // Saving function from variable stack
    ];

    let mut functions = HashMap::<String, usize>::new();
    functions.insert(String::from("sum"), 1);

    let test_program = Program::new(instructions, functions);

    let driver = KsDriver::new("compiler/function_with_parameters.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

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
        Instruction::LoadConst(Constant::Integer(1)),
        Instruction::LoadFunction(0),
        Instruction::Store,
        Instruction::LoadVar(0),
        Instruction::Call,
        Instruction::ClearAcc,
    ];

    let mut functions = HashMap::<String, usize>::new();
    functions.insert(String::from("add"), 1);

    let test_program = Program::new(instructions, functions);

    let driver = KsDriver::new("compiler/function_call.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn function_call_with_parameters() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::Jump(7),                         // Skiping function to store it
        Instruction::Store,                           // Storing parameter a
        Instruction::Store,                           // Storing parameter b
        Instruction::LoadVar(0),                      // Loading var a to variable stack
        Instruction::LoadVar(1),                      // Loading var b to variable stack
        Instruction::Add,                             // Sum them
        Instruction::Free(2),                         // Free ownership of the local variables
        Instruction::Return,                          // And return
        Instruction::LoadConst(Constant::Integer(1)), // Defining function pointer as variable and save to variable stack
        Instruction::LoadFunction(0),
        Instruction::Store,      // Saving function from variable stack
        Instruction::LoadVar(0), // Loading variable stored on variable_id. It's a function
        Instruction::LoadConst(Constant::Integer(20)), // Loading constant 20
        Instruction::LoadConst(Constant::Integer(10)), // Loading constant 10
        Instruction::Call,       // Calling function with 2 arguments stored in variable stack
        Instruction::ClearAcc,   // Ending an expression
    ];

    let mut functions = HashMap::<String, usize>::new();
    functions.insert(String::from("add"), 1);

    let test_program = Program::new(instructions, functions);

    let driver = KsDriver::new("compiler/function_call_with_parameters.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn assignment_statment() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Store,
        Instruction::LoadVar(0),
        Instruction::LoadConst(Constant::Integer(20)),
        Instruction::Assign,
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/assignment_statment.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
#[ignore = "Broken parser, does not provide the structure, instead panics!"]
fn add_value_statment() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Store,
        Instruction::LoadVar(0),
        Instruction::LoadVar(0),
        Instruction::LoadConst(Constant::Integer(20)),
        Instruction::Add,
        Instruction::Assign,
        Instruction::Free(1),
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/add_value_statment.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
#[ignore = "Broken parser, does not provide the structure, instead panics!"]
fn remove_value_statment() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Store,
        Instruction::LoadVar(0),
        Instruction::LoadVar(0),
        Instruction::LoadConst(Constant::Integer(20)),
        Instruction::Minus,
        Instruction::Assign,
        Instruction::Free(1),
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/remove_value_statment.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn if_statement() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Store,
        Instruction::LoadVar(0),
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Eq,
        Instruction::JumpIfFalse(3),
        Instruction::LoadVar(0),
        Instruction::LoadConst(Constant::Integer(20)),
        Instruction::Assign,
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/if_statement.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn if_statement_with_else() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Store,
        Instruction::LoadVar(0),
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Eq,
        Instruction::JumpIfFalse(4),
        Instruction::LoadVar(0),
        Instruction::LoadConst(Constant::Integer(20)),
        Instruction::Assign,
        Instruction::Jump(3),
        Instruction::LoadVar(0),
        Instruction::LoadConst(Constant::Integer(30)),
        Instruction::Assign,
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/if_statement_with_else.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn while_statement() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(0)),
        Instruction::Store,
        Instruction::Jump(5),
        Instruction::LoadVar(0),
        Instruction::LoadVar(0),
        Instruction::LoadConst(Constant::Integer(1)),
        Instruction::Add,
        Instruction::Assign,
        Instruction::LoadVar(0),
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::GreaterEq,
        Instruction::JumpIfTrue(-8),
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/while_statement.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn for_statement() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(0)),
        Instruction::LoadConst(Constant::Integer(1)),
        Instruction::LoadConst(Constant::Integer(2)),
        Instruction::LoadCollection(3),
        Instruction::Store, // list_iter = [0, 1, 2]
        Instruction::LoadConst(Constant::Integer(0)),
        Instruction::Store, // iterator = 0,
        Instruction::LoadConst(Constant::Null),
        Instruction::Store, // our preallocated variable
        Instruction::LoadVar(2),
        Instruction::LoadVar(1),
        Instruction::LoadVar(0),
        Instruction::LoadFromCollection,
        Instruction::Assign, // changing the variable to the list number
        Instruction::LoadVar(1),
        Instruction::Increment, // iterator++
        Instruction::LoadVar(0),
        Instruction::CollectionLen,
        Instruction::GreaterEq, // iterator >= list_iter.len()
        Instruction::JumpIfFalse(-10),
        Instruction::Free(3),
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/for_statement.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

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
        Instruction::LoadCollection(5),
        Instruction::Store,
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/list_expression.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn unary_operator() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Boolean(true)),
        Instruction::Store,
        Instruction::LoadVar(0),
        Instruction::Not,
        Instruction::Store,
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Minus,
        Instruction::Store,
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/unary_operator.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn front_unary_operator() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Store,
        Instruction::LoadVar(0),
        Instruction::Increment,
        Instruction::ClearAcc,
        Instruction::LoadVar(0),
        Instruction::Decrement,
        Instruction::ClearAcc,
        Instruction::LoadVar(0),
        Instruction::Clone,
        Instruction::Store,
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/front_unary_operator.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn tuple_literal() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::LoadConst(Constant::String(String::from("Hello World"))),
        Instruction::LoadConst(Constant::Float(3.14)),
        Instruction::LoadConst(Constant::Boolean(true)),
        Instruction::LoadCollection(4),
        Instruction::Store,
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/tuple_literal.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn module_literal() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::LoadConst(Constant::Integer(20)),
        Instruction::LoadConst(Constant::String(String::from("Kyryl"))),
        Instruction::LoadCollection(3),
        Instruction::Store,
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/module_literal.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn complex_module() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::LoadConst(Constant::String(String::from("Monobank"))),
        Instruction::LoadCollection(2),
        Instruction::LoadConst(Constant::String(String::from("Kyryl"))),
        Instruction::LoadCollection(2),
        Instruction::Store,
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/complex_module.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn access_module_children() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::String(String::from("Hello World"))),
        Instruction::LoadConst(Constant::String(String::from("Hi"))),
        Instruction::LoadCollection(1),
        Instruction::LoadCollection(2),
        Instruction::Store,
        Instruction::LoadVar(0),
        Instruction::LoadConst(Constant::Integer(1)),
        Instruction::LoadFromCollection,
        Instruction::LoadConst(Constant::Integer(0)),
        Instruction::LoadFromCollection,
        Instruction::ClearAcc,
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/access_module_children.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn complex_access() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::String(String::from("Headphones"))),
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::LoadConst(Constant::String(String::from("A PEN"))), // name
        Instruction::LoadCollection(1),
        Instruction::LoadCollection(3),
        Instruction::LoadConst(Constant::String(String::from("Rust"))),
        Instruction::LoadConst(Constant::String(String::from("KyrylScript"))),
        Instruction::LoadConst(Constant::String(String::from("JavaScript"))),
        Instruction::LoadConst(Constant::String(String::from("Lua"))),
        Instruction::LoadCollection(4), // languages
        Instruction::LoadConst(Constant::String(String::from("Kyryl"))), // name
        Instruction::LoadCollection(3),
        Instruction::Store,
        Instruction::LoadVar(0),
        Instruction::LoadConst(Constant::Integer(2)),
        Instruction::LoadFromCollection,
        Instruction::ClearAcc, // person.name;
        Instruction::LoadVar(0),
        Instruction::LoadConst(Constant::Integer(1)),
        Instruction::LoadFromCollection,
        Instruction::LoadConst(Constant::Integer(1)),
        Instruction::LoadFromCollection,
        Instruction::ClearAcc, // person.languages[1];
        Instruction::LoadVar(0),
        Instruction::LoadConst(Constant::Integer(0)),
        Instruction::LoadFromCollection,
        Instruction::LoadConst(Constant::Integer(2)),
        Instruction::LoadFromCollection,
        Instruction::ClearAcc, // person.items_on_the_table->2;
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/complex_access.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn complex_assignment_statement() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::String(String::from("Egg"))),
        Instruction::LoadConst(Constant::String(String::from("Soup"))),
        Instruction::LoadConst(Constant::String(String::from("Becon"))),
        Instruction::LoadCollection(3),
        Instruction::LoadCollection(1),
        Instruction::Store,
        Instruction::LoadVar(0),
        Instruction::LoadConst(Constant::Integer(0)),
        Instruction::LoadFromCollection,
        Instruction::LoadConst(Constant::Integer(2)),
        Instruction::LoadFromCollection,
        Instruction::LoadConst(Constant::String(String::from("Food"))),
        Instruction::Assign,
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/complex_assignment_statement.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn scope_enter_exit() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(20)),
        Instruction::Store,
        Instruction::LoadConst(Constant::Boolean(true)),
        Instruction::JumpIfFalse(3),
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Store,
        Instruction::Free(1),
        Instruction::LoadConst(Constant::Integer(23)),
        Instruction::Store,
        Instruction::Jump(3),
        Instruction::LoadConst(Constant::String(String::from("Hello World"))),
        Instruction::Store,
        Instruction::Free(1),
        Instruction::LoadConst(Constant::Boolean(true)),
        Instruction::JumpIfTrue(-4),
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Store,
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::LoadConst(Constant::Integer(20)),
        Instruction::LoadConst(Constant::Integer(30)),
        Instruction::LoadConst(Constant::Integer(40)),
        Instruction::LoadCollection(4),
        Instruction::Store,
        Instruction::LoadConst(Constant::Integer(0)),
        Instruction::Store,
        Instruction::LoadConst(Constant::Null),
        Instruction::Store,
        Instruction::LoadVar(5),
        Instruction::LoadVar(4),
        Instruction::LoadVar(3),
        Instruction::LoadFromCollection,
        Instruction::Assign,
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Store,
        Instruction::Free(1),
        Instruction::LoadVar(4),
        Instruction::Increment,
        Instruction::LoadVar(3),
        Instruction::CollectionLen,
        Instruction::GreaterEq,
        Instruction::JumpIfFalse(-13),
        Instruction::Free(3),
        Instruction::LoadConst(Constant::Integer(345)),
        Instruction::Store,
    ];

    let test_program = Program::new(instructions, HashMap::new());

    let driver = KsDriver::new("compiler/scope_enter_exit.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn function_scope_store_name_register() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Store,
        Instruction::Jump(7),
        Instruction::LoadCapture(0),
        Instruction::Store,
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Store,
        Instruction::LoadVar(0),
        Instruction::Free(2),
        Instruction::Return,
        Instruction::LoadConst(Constant::Integer(3)),
        Instruction::LoadVar(0),
        Instruction::LoadFunction(1),
        Instruction::Store,
        Instruction::LoadVar(1),
        Instruction::Call,
        Instruction::ClearAcc,
    ];

    let mut functions = HashMap::<String, usize>::new();
    functions.insert(String::from("return_the_variable"), 3);

    let test_program = Program::new(instructions, functions);

    let driver = KsDriver::new("compiler/function_scope_store_name_register.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn function_return_in_if_statement() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::Jump(12),
        Instruction::Store,
        Instruction::LoadVar(0),
        Instruction::LoadConst(Constant::Integer(20)),
        Instruction::Eq,
        Instruction::JumpIfFalse(3),
        Instruction::LoadConst(Constant::Null),
        Instruction::Free(1),
        Instruction::Return,
        Instruction::LoadConst(Constant::Integer(20)),
        Instruction::Store,
        Instruction::Free(2),
        Instruction::Return,
        Instruction::LoadConst(Constant::Integer(1)),
        Instruction::LoadFunction(0),
        Instruction::Store,
    ];

    let mut functions = HashMap::<String, usize>::new();
    functions.insert(String::from("test"), 1);

    let test_program = Program::new(instructions, functions);

    let driver = KsDriver::new("compiler/function_return_in_if_statement.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn multiple_function_scoping() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::LoadConst(Constant::Integer(783)),
        Instruction::Store,
        Instruction::Jump(19),
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Store,
        Instruction::Jump(9),
        Instruction::LoadCapture(0),
        Instruction::Store,
        Instruction::LoadConst(Constant::Integer(20)),
        Instruction::Store,
        Instruction::LoadVar(0),
        Instruction::LoadVar(1),
        Instruction::Add,
        Instruction::Free(2),
        Instruction::Return,
        Instruction::LoadConst(Constant::Integer(6)),
        Instruction::LoadVar(0),
        Instruction::LoadFunction(1),
        Instruction::Store,
        Instruction::LoadVar(1),
        Instruction::Free(2),
        Instruction::Return,
        Instruction::LoadConst(Constant::Integer(3)),
        Instruction::LoadFunction(0),
        Instruction::Store,
        Instruction::LoadVar(1),
        Instruction::Call,
        Instruction::Call,
        Instruction::Store,
    ];

    let mut functions = HashMap::<String, usize>::new();
    functions.insert(String::from("foo"), 3);
    functions.insert(String::from("bar"), 6);

    let test_program = Program::new(instructions, functions);

    let driver = KsDriver::new("compiler/multiple_function_scoping.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(test_program, program);

    Ok(())
}

#[test]
fn function_curring() -> KsResult<()> {
    let instructions: Vec<Instruction> = vec![
        Instruction::Jump(29),
        Instruction::Store,
        Instruction::Jump(22),
        Instruction::Store,
        Instruction::LoadCapture(0),
        Instruction::Store,
        Instruction::Jump(12),
        Instruction::Store,
        Instruction::LoadCapture(0),
        Instruction::Store,
        Instruction::LoadCapture(1),
        Instruction::Store,
        Instruction::LoadVar(1),
        Instruction::LoadVar(2),
        Instruction::Add,
        Instruction::LoadVar(0),
        Instruction::Add,
        Instruction::Free(3),
        Instruction::Return,
        Instruction::LoadConst(Constant::Integer(7)),
        Instruction::LoadVar(1),
        Instruction::LoadVar(0),
        Instruction::LoadFunction(2),
        Instruction::Free(2),
        Instruction::Return,
        Instruction::LoadConst(Constant::Integer(3)),
        Instruction::LoadVar(0),
        Instruction::LoadFunction(1),
        Instruction::Free(1),
        Instruction::Return,
        Instruction::LoadConst(Constant::Integer(1)),
        Instruction::LoadFunction(0),
        Instruction::Store,
        Instruction::LoadVar(0),
        Instruction::LoadConst(Constant::Integer(10)),
        Instruction::Call,
        Instruction::LoadConst(Constant::Integer(20)),
        Instruction::Call,
        Instruction::LoadConst(Constant::Integer(30)),
        Instruction::Call,
        Instruction::Store,
    ];

    let mut functions = HashMap::<String, usize>::new();
    functions.insert(String::from("curry"), 1);

    let test_program = Program::new(instructions, functions);

    let driver = KsDriver::new("compiler/function_curring.ks");
    let compiler = driver.compiler_new()?;
    let program = compiler.program();

    assert_eq!(test_program, program);

    Ok(())
}
