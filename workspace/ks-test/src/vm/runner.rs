use ks_global::utils::{ks_error::KsError, ks_result::KsResult};
use ks_vm_new::{Collection, Constant, Instruction, Variable};

use crate::drivers::KsDriver;

#[test]
fn load_const_null() -> KsResult<()> {
    let driver = KsDriver::runner(Instruction::LoadConst(Constant::Null))?;

    assert_eq!(driver.runner.program_counter(), 1);
    assert_eq!(driver.runner.acc[0], 0);
    assert_eq!(driver.gvs.storage[0], Some(Variable::null()));

    Ok(())
}

#[test]
fn load_const_int() -> KsResult<()> {
    let int = 10i64;

    let driver = KsDriver::runner(Instruction::LoadConst(Constant::Integer(int)))?;

    assert_eq!(driver.runner.program_counter(), 1);
    assert_eq!(driver.runner.acc[0], 0);
    assert_eq!(driver.gvs.storage[0], Some(Variable::from(int)));

    Ok(())
}

#[test]
fn load_const_float() -> KsResult<()> {
    let float = 3.14;

    let driver = KsDriver::runner(Instruction::LoadConst(Constant::Float(float)))?;

    assert_eq!(driver.runner.program_counter(), 1);
    assert_eq!(driver.runner.acc[0], 0);
    assert_eq!(driver.gvs.storage[0], Some(Variable::from(float)));

    Ok(())
}

#[test]
fn load_const_string() -> KsResult<()> {
    let string = String::from("Hello World");

    let driver = KsDriver::runner(Instruction::LoadConst(Constant::String(string.clone())))?;

    assert_eq!(driver.runner.program_counter(), 1);
    assert_eq!(driver.runner.acc[0], 0);
    assert_eq!(driver.gvs.storage[0], Some(Variable::string(0)));
    assert_eq!(driver.gvs.collections[0], Collection::String(string));

    Ok(())
}

#[test]
fn load_const_boolean() -> KsResult<()> {
    let boolean = false;

    let driver = KsDriver::runner(Instruction::LoadConst(Constant::Boolean(boolean)))?;

    assert_eq!(driver.runner.program_counter(), 1);
    assert_eq!(driver.runner.acc[0], 0);
    assert_eq!(driver.gvs.storage[0], Some(Variable::from(boolean)));

    Ok(())
}

#[test]
fn load_var() -> KsResult<()> {
    let mut int = Variable::from(67);
    int.owners += 1;
    let storage_id = 0;

    let gvs = KsDriver::gvs_storage(vec![Some(int)]);
    let runner = KsDriver::runner_default(None, Some(vec![storage_id]), false, None);

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::LoadVar(0))?;

    let variable = driver.gvs.storage[0].clone().unwrap();

    assert_eq!(variable.owners, 2);
    assert_eq!(driver.runner.program_counter(), 1);
    assert_eq!(driver.runner.acc, vec![0]);

    Ok(())
}

#[test]
fn load_var_invalid_storage_id() -> KsResult<()> {
    let storage_id = 5;
    let runner = KsDriver::runner_default(None, Some(vec![storage_id]), false, None);

    let err = KsDriver::runner_configured(runner, None, Instruction::LoadVar(0)).unwrap_err();
    assert_eq!(
        err,
        KsError::runtime(&format!("Cannot access variable {}", storage_id))
    );

    Ok(())
}

#[test]
fn load_var_invalid_slot() -> KsResult<()> {
    let slot = 10;

    let err = KsDriver::runner(Instruction::LoadVar(slot)).unwrap_err();
    assert_eq!(
        err,
        KsError::runtime(&format!("Cannot get storage_id by slot {}", slot))
    );

    Ok(())
}

#[test]
fn jump_positive() -> KsResult<()> {
    let runner = KsDriver::runner_default(None, None, false, None);
    let jump_offset = 32;

    let driver = KsDriver::runner_configured(runner, None, Instruction::Jump(jump_offset))?;

    assert_eq!(driver.runner.program_counter(), jump_offset as usize);
    assert_eq!(driver.runner.prevent_step, false);

    Ok(())
}

#[test]
fn jump_negative() -> KsResult<()> {
    let initial_pc = 64;
    let jump_offset = -5;

    let runner = KsDriver::runner_default(None, None, false, Some(initial_pc));
    let driver = KsDriver::runner_configured(runner, None, Instruction::Jump(jump_offset))?;

    assert_eq!(
        driver.runner.program_counter(),
        initial_pc.saturating_add_signed(jump_offset as isize)
    );
    assert_eq!(driver.runner.prevent_step, false);

    Ok(())
}
