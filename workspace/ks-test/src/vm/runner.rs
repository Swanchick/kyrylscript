use ks_global::utils::ks_result::KsResult;
use ks_vm_new::{Collection, Constant, Instruction, Variable};

use crate::{drivers::KsDriver, vm::runner};

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
    let slot_id = 0;

    let gvs = KsDriver::gvs_storage(vec![Some(int)]);
    let runner = KsDriver::runner_stack(None, Some(vec![slot_id]));

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::LoadVar(0))?;

    let variable = driver.gvs.storage[0].clone().unwrap();

    assert_eq!(variable.owners, 2);
    assert_eq!(driver.runner.program_counter(), 1);
    assert_eq!(driver.runner.acc, vec![0]);

    Ok(())
}
