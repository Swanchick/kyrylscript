use ks_global::utils::{ks_error::KsError, ks_result::KsResult};
use ks_vm_new::{Collection, Constant, Instruction, Stack, Variable};

use crate::drivers::KsDriver;
use crate::drivers::utils::operation;
use paste::paste;

#[test]
fn load_const_null() -> KsResult<()> {
    let mut variable = Variable::null();
    variable.owners = 1;

    let driver = KsDriver::runner(Instruction::LoadConst(Constant::Null))?;

    assert_eq!(driver.runner.program_counter(), 1);
    assert_eq!(driver.runner.acc.get(0), Some(&0));
    assert_eq!(driver.gvs.storage[0], Some(variable));

    Ok(())
}

#[test]
fn load_const_int() -> KsResult<()> {
    let int = 10i64;
    let mut variable = Variable::from(int);
    variable.owners = 1;

    let driver = KsDriver::runner(Instruction::LoadConst(Constant::Integer(int)))?;

    assert_eq!(driver.runner.program_counter(), 1);
    assert_eq!(driver.runner.acc.get(0), Some(&0));
    assert_eq!(driver.gvs.storage[0], Some(variable));

    Ok(())
}

#[test]
fn load_const_float() -> KsResult<()> {
    let float = 3.14;
    let mut variable = Variable::from(float);
    variable.owners = 1;

    let driver = KsDriver::runner(Instruction::LoadConst(Constant::Float(float)))?;

    assert_eq!(driver.runner.program_counter(), 1);
    assert_eq!(driver.runner.acc.get(0), Some(&0));
    assert_eq!(driver.gvs.storage[0], Some(variable));

    Ok(())
}

#[test]
fn load_const_string() -> KsResult<()> {
    let string = String::from("Hello World");
    let mut variable = Variable::string(0);
    variable.owners = 1;

    let driver = KsDriver::runner(Instruction::LoadConst(Constant::String(string.clone())))?;

    assert_eq!(driver.runner.program_counter(), 1);
    assert_eq!(driver.runner.acc.get(0), Some(&0));
    assert_eq!(driver.gvs.storage[0], Some(variable));
    assert_eq!(driver.gvs.collections[0], Collection::String(string));

    Ok(())
}

#[test]
fn load_const_boolean() -> KsResult<()> {
    let boolean = false;
    let mut variable = Variable::from(boolean);
    variable.owners = 1;

    let driver = KsDriver::runner(Instruction::LoadConst(Constant::Boolean(boolean)))?;

    assert_eq!(driver.runner.program_counter(), 1);
    assert_eq!(driver.runner.acc.get(0), Some(&0));
    assert_eq!(driver.gvs.storage[0], Some(variable));

    Ok(())
}

#[test]
fn load_var() -> KsResult<()> {
    let mut int = Variable::from(67);
    int.owners += 1;
    let storage_id = 0;

    let gvs = KsDriver::gvs_storage(Some(vec![Some(int)]), None);
    let runner = KsDriver::runner_default(None, Some(Stack::from(vec![storage_id])), false, None);

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::LoadVar(0))?;

    let variable = driver.gvs.storage[0].clone().unwrap();

    assert_eq!(variable.owners, 2);
    assert_eq!(driver.runner.program_counter(), 1);
    assert_eq!(driver.runner.acc.get(0), Some(&0));

    Ok(())
}

#[test]
fn load_var_invalid_storage_id() -> KsResult<()> {
    let storage_id = 5;
    let runner = KsDriver::runner_default(None, Some(Stack::from(vec![storage_id])), false, None);

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

operation!(add, Instruction::Add, +);

#[test]
fn add_string_string() -> KsResult<()> {
    let string_left = String::from("Hello,");
    let string_right = String::from(" world!");
    let string_result = format!("{}{}", string_left, string_right);

    let mut variable_left = Variable::string(0);
    variable_left.owners = 2;
    let mut variable_right = Variable::string(1);
    variable_right.owners = 2;
    let mut variable_result = Variable::string(2);
    variable_result.owners = 1;

    let runner = KsDriver::runner_default(
        Some(Stack::from(vec![0, 1])),
        Some(Stack::from(vec![0, 1])),
        false,
        None,
    );
    let gvs = KsDriver::gvs_storage(
        Some(vec![Some(variable_left), Some(variable_right)]),
        Some(vec![
            Collection::String(string_left),
            Collection::String(string_right),
        ]),
    );

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::Add)?;

    assert_eq!(driver.runner.program_counter, 1);
    assert_eq!(driver.runner.acc.len(), 1);
    assert_eq!(driver.runner.acc.get(0), Some(&2));

    let gvs_variable1_left = driver.gvs.storage[0].clone().unwrap();
    let gvs_variable1_right = driver.gvs.storage[1].clone().unwrap();
    let gvs_variable1_result = driver.gvs.storage[2].clone().unwrap();

    assert_eq!(gvs_variable1_left.owners, 1);
    assert_eq!(gvs_variable1_right.owners, 1);

    assert_eq!(driver.gvs.collections[2], Collection::String(string_result));
    assert_eq!(gvs_variable1_result, variable_result);
    Ok(())
}

operation!(minus, Instruction::Minus, -);
operation!(mul, Instruction::Mul, *);

#[test]
fn div_int_int() -> KsResult<()> {
    let int_left = 10;
    let int_right = 20;

    let mut variable_left = Variable::from(int_left);
    variable_left.owners = 2;
    let mut variable_right = Variable::from(int_right);
    variable_right.owners = 2;
    let mut variable_result = Variable::from(int_left as f64 / int_right as f64);
    variable_result.owners = 1;

    KsDriver::operation_test(
        variable_left,
        variable_right,
        variable_result,
        Instruction::Div,
    )?;

    Ok(())
}

#[test]
fn div_int_float() -> KsResult<()> {
    let int_left = 10;
    let float_right = 3.14;

    let mut variable_left = Variable::from(int_left);
    variable_left.owners = 2;
    let mut variable_right = Variable::from(float_right);
    variable_right.owners = 2;
    let mut variable_result = Variable::from((int_left as f64) / float_right);
    variable_result.owners = 1;

    KsDriver::operation_test(
        variable_left,
        variable_right,
        variable_result,
        Instruction::Div,
    )?;

    Ok(())
}

#[test]
fn div_float_int() -> KsResult<()> {
    let float_left = 3.14;
    let int_right = 10;

    let mut variable_left = Variable::from(float_left);
    variable_left.owners = 2;
    let mut variable_right = Variable::from(int_right);
    variable_right.owners = 2;
    let mut variable_result = Variable::from(float_left / (int_right as f64));
    variable_result.owners = 1;

    KsDriver::operation_test(
        variable_left,
        variable_right,
        variable_result,
        Instruction::Div,
    )?;

    Ok(())
}

#[test]
fn div_float_float() -> KsResult<()> {
    let float_left = 3.14;
    let float_right = 1.23;

    let mut variable_left = Variable::from(float_left);
    variable_left.owners = 2;
    let mut variable_right = Variable::from(float_right);
    variable_right.owners = 2;
    let mut variable_result = Variable::from(float_left / float_right);
    variable_result.owners = 1;

    KsDriver::operation_test(
        variable_left,
        variable_right,
        variable_result,
        Instruction::Div,
    )?;

    Ok(())
}

#[test]
fn div_zero_division_error() -> KsResult<()> {
    let float_left = 3.14;
    let float_right = 0.0;

    let mut variable_left = Variable::from(float_left);
    variable_left.owners = 2;
    let mut variable_right = Variable::from(float_right);
    variable_right.owners = 2;

    let runner = KsDriver::runner_default(
        Some(Stack::from(vec![0, 1])),
        Some(Stack::from(vec![0, 1])),
        false,
        None,
    );
    let gvs = KsDriver::gvs_storage(Some(vec![Some(variable_left), Some(variable_right)]), None);

    let err = KsDriver::runner_configured(runner, gvs, Instruction::Div).unwrap_err();

    assert_eq!(err, KsError::runtime("Zero division error"));

    Ok(())
}

operation!(eq, Instruction::Eq, ==);

#[test]
fn eq_string_string() -> KsResult<()> {
    let string_left = String::from("Hello,");
    let string_right = String::from(" world!");

    let mut variable_left = Variable::string(0);
    variable_left.owners = 2;
    let mut variable_right = Variable::string(1);
    variable_right.owners = 2;
    let mut variable_result = Variable::from(string_left == string_right);
    variable_result.owners = 1;

    let runner = KsDriver::runner_default(
        Some(Stack::from(vec![0, 1])),
        Some(Stack::from(vec![0, 1])),
        false,
        None,
    );
    let gvs = KsDriver::gvs_storage(
        Some(vec![Some(variable_left), Some(variable_right)]),
        Some(vec![
            Collection::String(string_left),
            Collection::String(string_right),
        ]),
    );

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::Eq)?;

    assert_eq!(driver.runner.program_counter, 1);
    assert_eq!(driver.runner.acc.len(), 1);
    assert_eq!(driver.runner.acc.get(0), Some(&2));

    let gvs_variable1_left = driver.gvs.storage[0].clone().unwrap();
    let gvs_variable1_right = driver.gvs.storage[1].clone().unwrap();
    let gvs_variable1_result = driver.gvs.storage[2].clone().unwrap();

    assert_eq!(gvs_variable1_left.owners, 1);
    assert_eq!(gvs_variable1_right.owners, 1);
    assert_eq!(gvs_variable1_result, variable_result);

    Ok(())
}

operation!(greater_eq, Instruction::GreaterEq, >=);
operation!(greater, Instruction::Greater, >);
operation!(less_eq, Instruction::LessEq, <=);
operation!(less, Instruction::Less, <);

operation!(not_eq, Instruction::NotEq, !=);

#[test]
fn not_eq_string_string() -> KsResult<()> {
    let string_left = String::from("Hello,");
    let string_right = String::from(" world!");

    let mut variable_left = Variable::string(0);
    variable_left.owners = 2;
    let mut variable_right = Variable::string(1);
    variable_right.owners = 2;
    let mut variable_result = Variable::from(string_left != string_right);
    variable_result.owners = 1;

    let runner = KsDriver::runner_default(
        Some(Stack::from(vec![0, 1])),
        Some(Stack::from(vec![0, 1])),
        false,
        None,
    );
    let gvs = KsDriver::gvs_storage(
        Some(vec![Some(variable_left), Some(variable_right)]),
        Some(vec![
            Collection::String(string_left),
            Collection::String(string_right),
        ]),
    );

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::NotEq)?;

    assert_eq!(driver.runner.program_counter, 1);
    assert_eq!(driver.runner.acc.len(), 1);
    assert_eq!(driver.runner.acc.get(0), Some(&2));

    let gvs_variable1_left = driver.gvs.storage[0].clone().unwrap();
    let gvs_variable1_right = driver.gvs.storage[1].clone().unwrap();
    let gvs_variable1_result = driver.gvs.storage[2].clone().unwrap();

    assert_eq!(gvs_variable1_left.owners, 1);
    assert_eq!(gvs_variable1_right.owners, 1);
    assert_eq!(gvs_variable1_result, variable_result);

    Ok(())
}

#[test]
fn and_true() -> KsResult<()> {
    let left = true;
    let right = true;

    let mut variable_left = Variable::from(left);
    variable_left.owners = 2;
    let mut variable_right = Variable::from(right);
    variable_right.owners = 2;
    let mut variable_result = Variable::from(left && right);
    variable_result.owners = 1;

    KsDriver::operation_test(
        variable_left,
        variable_right,
        variable_result,
        Instruction::And,
    )?;

    Ok(())
}

#[test]
fn and_false() -> KsResult<()> {
    let left = true;
    let right = false;

    let mut variable_left = Variable::from(left);
    variable_left.owners = 2;
    let mut variable_right = Variable::from(right);
    variable_right.owners = 2;
    let mut variable_result = Variable::from(left && right);
    variable_result.owners = 1;

    KsDriver::operation_test(
        variable_left,
        variable_right,
        variable_result,
        Instruction::And,
    )?;

    Ok(())
}

#[test]
fn or_true() -> KsResult<()> {
    let left = false;
    let right = true;

    let mut variable_left = Variable::from(left);
    variable_left.owners = 2;
    let mut variable_right = Variable::from(right);
    variable_right.owners = 2;
    let mut variable_result = Variable::from(left || right);
    variable_result.owners = 1;

    KsDriver::operation_test(
        variable_left,
        variable_right,
        variable_result,
        Instruction::Or,
    )?;

    Ok(())
}

#[test]
fn or_false() -> KsResult<()> {
    let left = false;
    let right = false;

    let mut variable_left = Variable::from(left);
    variable_left.owners = 2;
    let mut variable_right = Variable::from(right);
    variable_right.owners = 2;
    let mut variable_result = Variable::from(left || right);
    variable_result.owners = 1;

    KsDriver::operation_test(
        variable_left,
        variable_right,
        variable_result,
        Instruction::Or,
    )?;

    Ok(())
}
