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
