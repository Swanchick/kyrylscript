use ks_global::utils::ks_result::KsResult;
use ks_vm_new::{Constant, Instruction, VM, Variable};

#[test]
fn load_const_null() -> KsResult<()> {
    let instructions = vec![Instruction::LoadConst(Constant::Null)];
    let mut vm = VM::from(instructions);
    vm.init();
    vm.step()?;

    let runner = &vm.runners[0];
    let vgs = vm.vgs;

    assert_eq!(runner.acc[0], 0);
    assert_eq!(vgs.storage[0], Some(Variable::null()));

    Ok(())
}

#[test]
fn load_const_int() -> KsResult<()> {
    let int = 10i64;
    let instructions = vec![Instruction::LoadConst(Constant::Integer(int))];
    let mut vm = VM::from(instructions);
    vm.init();
    vm.step()?;

    let runner = &vm.runners[0];
    let vgs = vm.vgs;

    assert_eq!(runner.acc[0], 0);
    assert_eq!(vgs.storage[0], Some(Variable::from(int)));

    Ok(())
}

#[test]
fn load_const_float() -> KsResult<()> {
    let float = 3.14;
    let instructions = vec![Instruction::LoadConst(Constant::Float(float))];
    let mut vm = VM::from(instructions);
    vm.init();
    vm.step()?;

    let runner = &vm.runners[0];
    let vgs = vm.vgs;

    assert_eq!(runner.acc[0], 0);
    assert_eq!(vgs.storage[0], Some(Variable::from(float)));

    Ok(())
}

#[test]
fn load_const_string() -> KsResult<()> {
    let string = String::from("Hello World");
    let instructions = vec![Instruction::LoadConst(Constant::String(string.clone()))];
    let mut vm = VM::from(instructions);
    vm.init();
    vm.step()?;

    let runner = &vm.runners[0];
    let vgs = vm.vgs;

    // assert_eq!(runner.acc[0], 0);
    // assert_eq!(vgs.storage[0], Some(Variable::from(float)));

    todo!()
}

#[test]
fn load_const_boolean() -> KsResult<()> {
    let boolean = false;
    let instructions = vec![Instruction::LoadConst(Constant::Boolean(boolean))];
    let mut vm = VM::from(instructions);
    vm.init();
    vm.step()?;

    let runner = &vm.runners[0];
    let vgs = vm.vgs;

    assert_eq!(runner.acc[0], 0);
    assert_eq!(vgs.storage[0], Some(Variable::from(boolean)));

    Ok(())
}
