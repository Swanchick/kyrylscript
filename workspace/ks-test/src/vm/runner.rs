use ks_global::utils::{ks_error::KsError, ks_result::KsResult};

use ks_vm_new::types::Pointer;
use ks_vm_new::{CallStack, Collection, Constant, Function, Instruction, Stack, Variable};

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
fn load_const_with_free_storage() -> KsResult<()> {
    let integer = 100;
    let variable = Variable::from(integer).with_owners(1);

    let storage = vec![None, Some(Variable::from(250).with_owners(1))];

    let gvs = KsDriver::gvs_storage(Some(storage), None, Some(vec![0]), None);

    let driver = KsDriver::runner_configured(
        None,
        gvs,
        Instruction::LoadConst(Constant::Integer(integer)),
    )?;

    assert_eq!(driver.runner.program_counter(), 1);
    assert_eq!(driver.runner.acc.get(0), Some(&0));
    assert_eq!(driver.gvs.storage[0], Some(variable));

    assert_eq!(
        driver.gvs.storage[0],
        Some(Variable::from(integer).with_owners(1)),
    );

    Ok(())
}

#[test]
fn load_var() -> KsResult<()> {
    let mut int = Variable::from(67);
    int.owners += 1;
    let storage_id = 0;

    let gvs = KsDriver::gvs_storage(Some(vec![Some(int)]), None, None, None);
    let runner =
        KsDriver::runner_default(None, Some(Stack::from(vec![storage_id])), false, None, None);

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
    let runner =
        KsDriver::runner_default(None, Some(Stack::from(vec![storage_id])), false, None, None);

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
    let runner = KsDriver::runner_default(None, None, false, None, None);
    let jump_offset = 32;

    let driver = KsDriver::runner_configured(runner, None, Instruction::Jump(jump_offset))?;

    assert_eq!(driver.runner.program_counter(), jump_offset as Pointer);
    assert_eq!(driver.runner.prevent_step, false);

    Ok(())
}

#[test]
fn jump_negative() -> KsResult<()> {
    let initial_pc = 64;
    let jump_offset = -5;

    let runner = KsDriver::runner_default(None, None, false, Some(initial_pc), None);
    let driver = KsDriver::runner_configured(runner, None, Instruction::Jump(jump_offset))?;

    assert_eq!(
        driver.runner.program_counter(),
        initial_pc.saturating_add_signed(jump_offset as isize) as Pointer
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
        None,
    );
    let gvs = KsDriver::gvs_storage(
        Some(vec![Some(variable_left), Some(variable_right)]),
        Some(vec![
            Collection::String(string_left),
            Collection::String(string_right),
        ]),
        None,
        None,
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
        None,
    );
    let gvs = KsDriver::gvs_storage(
        Some(vec![Some(variable_left), Some(variable_right)]),
        None,
        None,
        None,
    );

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
        None,
    );
    let gvs = KsDriver::gvs_storage(
        Some(vec![Some(variable_left), Some(variable_right)]),
        Some(vec![
            Collection::String(string_left),
            Collection::String(string_right),
        ]),
        None,
        None,
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
        None,
    );
    let gvs = KsDriver::gvs_storage(
        Some(vec![Some(variable_left), Some(variable_right)]),
        Some(vec![
            Collection::String(string_left),
            Collection::String(string_right),
        ]),
        None,
        None,
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

#[test]
fn not_true() -> KsResult<()> {
    let value = true;

    let mut variable_left = Variable::from(value);
    variable_left.owners = 2;
    let mut variable_result = Variable::from(!value);
    variable_result.owners = 1;

    let gvs = KsDriver::gvs_storage(Some(vec![Some(variable_left)]), None, None, None);
    let runner = KsDriver::runner_default(
        Some(Stack::from(vec![0])),
        Some(Stack::from(vec![0])),
        false,
        None,
        None,
    );

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::Not)?;

    assert_eq!(driver.runner.program_counter, 1);
    assert_eq!(driver.runner.acc.len(), 1);
    assert_eq!(driver.runner.acc.get(0).unwrap(), &1);

    let gvs_variable_value = driver.gvs.storage[0].clone().unwrap();
    let gvs_variable_result = driver.gvs.storage[1].clone().unwrap();

    assert_eq!(gvs_variable_value.owners, 1);
    assert_eq!(gvs_variable_result, variable_result);

    Ok(())
}

#[test]
fn not_false() -> KsResult<()> {
    let value = false;

    let mut variable_left = Variable::from(value);
    variable_left.owners = 2;
    let mut variable_result = Variable::from(!value);
    variable_result.owners = 1;

    let gvs = KsDriver::gvs_storage(Some(vec![Some(variable_left)]), None, None, None);
    let runner = KsDriver::runner_default(
        Some(Stack::from(vec![0])),
        Some(Stack::from(vec![0])),
        false,
        None,
        None,
    );

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::Not)?;

    assert_eq!(driver.runner.program_counter, 1);
    assert_eq!(driver.runner.acc.len(), 1);
    assert_eq!(driver.runner.acc.get(0).unwrap(), &1);

    let gvs_variable_value = driver.gvs.storage[0].clone().unwrap();
    let gvs_variable_result = driver.gvs.storage[1].clone().unwrap();

    assert_eq!(gvs_variable_value.owners, 1);
    assert_eq!(gvs_variable_result, variable_result);

    Ok(())
}

#[test]
fn increment() -> KsResult<()> {
    let value = 10;

    let mut variable_left = Variable::from(value);
    variable_left.owners = 2;
    let mut variable_result = Variable::from(value + 1);
    variable_result.owners = 2;

    let gvs = KsDriver::gvs_storage(Some(vec![Some(variable_left)]), None, None, None);
    let runner = KsDriver::runner_default(
        Some(Stack::from(vec![0])),
        Some(Stack::from(vec![0])),
        false,
        None,
        None,
    );

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::Increment)?;

    assert_eq!(driver.runner.program_counter, 1);
    assert_eq!(driver.runner.acc.len(), 1);
    assert_eq!(driver.runner.acc.get(0).unwrap(), &0);

    let gvs_variable_result = driver.gvs.storage[0].clone().unwrap();

    assert_eq!(gvs_variable_result, variable_result);

    Ok(())
}

#[test]
fn decrement() -> KsResult<()> {
    let value = 10;

    let mut variable_left = Variable::from(value);
    variable_left.owners = 2;
    let mut variable_result = Variable::from(value - 1);
    variable_result.owners = 2;

    let gvs = KsDriver::gvs_storage(Some(vec![Some(variable_left)]), None, None, None);
    let runner = KsDriver::runner_default(
        Some(Stack::from(vec![0])),
        Some(Stack::from(vec![0])),
        false,
        None,
        None,
    );

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::Decrement)?;

    assert_eq!(driver.runner.program_counter, 1);
    assert_eq!(driver.runner.acc.len(), 1);
    assert_eq!(driver.runner.acc.get(0).unwrap(), &0);

    let gvs_variable_result = driver.gvs.storage[0].clone().unwrap();

    assert_eq!(gvs_variable_result, variable_result);

    Ok(())
}

#[test]
fn clone_primitive() -> KsResult<()> {
    let value = 10;

    let mut variable = Variable::from(value);
    variable.owners = 2;

    let gvs = KsDriver::gvs_storage(Some(vec![Some(variable.clone())]), None, None, None);
    let runner = KsDriver::runner_default(
        Some(Stack::from(vec![0])),
        Some(Stack::from(vec![0])),
        false,
        None,
        None,
    );

    variable.owners = 1;

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::Clone)?;

    assert_eq!(driver.runner.program_counter, 1);
    assert_eq!(driver.runner.acc.len(), 1);
    assert_eq!(driver.runner.acc.get(0).unwrap(), &1);

    let gvs_variable_original = driver.gvs.storage[0].clone().unwrap();
    let gvs_variable_result = driver.gvs.storage[1].clone().unwrap();

    assert_eq!(gvs_variable_original, variable);
    assert_eq!(gvs_variable_result, variable);

    Ok(())
}

#[test]
fn clone_collection_string() -> KsResult<()> {
    let value = String::from("Hello World");

    let mut variable = Variable::string(0);
    variable.owners = 2;

    let mut variable_new = Variable::string(1);
    variable_new.owners = 1;

    let gvs = KsDriver::gvs_storage(
        Some(vec![Some(variable.clone())]),
        Some(vec![Collection::String(value.clone())]),
        None,
        None,
    );
    let runner = KsDriver::runner_default(
        Some(Stack::from(vec![0])),
        Some(Stack::from(vec![0])),
        false,
        None,
        None,
    );

    variable.owners = 1;

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::Clone)?;

    assert_eq!(driver.runner.program_counter, 1);
    assert_eq!(driver.runner.acc.len(), 1);
    assert_eq!(driver.runner.acc.get(0).unwrap(), &1);

    let gvs_variable_original = driver.gvs.storage[0].clone().unwrap();
    let gvs_variable_result = driver.gvs.storage[1].clone().unwrap();

    let string_original = driver.gvs.collections.get(0).unwrap();
    let string_new = driver.gvs.collections.get(1).unwrap();

    assert_eq!(string_original, &Collection::String(value.clone()));
    assert_eq!(string_new, &Collection::String(value));

    assert_eq!(gvs_variable_original.owners, 1);

    assert_eq!(gvs_variable_original, variable);
    assert_eq!(gvs_variable_result, variable_new);

    Ok(())
}

#[test]
fn clone_collection() -> KsResult<()> {
    let value = vec![0, 1, 2, 3];
    let mut variable = Variable::collection(0).with_owners(2);
    let storage = vec![
        Some(Variable::from(123).with_owners(1)),
        Some(Variable::from(67).with_owners(1)),
        Some(Variable::from(32).with_owners(1)),
        Some(Variable::from(94).with_owners(1)),
        Some(variable.clone()),
    ];

    let new_value = vec![5, 6, 7, 8];
    let new_variable = Variable::collection(1).with_owners(1);
    let new_storage = vec![
        Some(Variable::from(123).with_owners(1)),
        Some(Variable::from(67).with_owners(1)),
        Some(Variable::from(32).with_owners(1)),
        Some(Variable::from(94).with_owners(1)),
        Some(variable.clone().with_owners(1)),
        Some(Variable::from(123).with_owners(1)),
        Some(Variable::from(67).with_owners(1)),
        Some(Variable::from(32).with_owners(1)),
        Some(Variable::from(94).with_owners(1)),
        Some(new_variable.clone()),
    ];

    let mut variable_new = Variable::collection(1);
    variable_new.owners = 1;

    let gvs = KsDriver::gvs_storage(
        Some(storage),
        Some(vec![Collection::Stack(value.clone())]),
        None,
        None,
    );

    let runner = KsDriver::runner_default(
        Some(Stack::from(vec![4])),
        Some(Stack::from(vec![4])),
        false,
        None,
        None,
    );

    variable.owners = 1;

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::Clone)?;

    assert_eq!(driver.runner.program_counter, 1);
    assert_eq!(driver.runner.acc.len(), 1);
    assert_eq!(driver.runner.acc.get(0).unwrap(), &9);

    let gvs_variable_original = driver.gvs.storage[4].clone().unwrap();
    let gvs_variable_result = driver.gvs.storage[9].clone().unwrap();

    assert_eq!(driver.gvs.storage, new_storage);

    let stack_original = driver.gvs.collections.get(0).unwrap();
    let stack_new = driver.gvs.collections.get(1).unwrap();

    assert_eq!(stack_original, &Collection::Stack(value.clone()));
    assert_eq!(stack_new, &Collection::Stack(new_value));

    assert_eq!(gvs_variable_original.owners, 1);

    assert_eq!(gvs_variable_original, variable);
    assert_eq!(gvs_variable_result, variable_new);

    Ok(())
}

#[test]
fn load_collection() -> KsResult<()> {
    let collection = Collection::Stack(vec![0, 1, 2, 3]);
    let mut storage = vec![
        Some(Variable::from(1).with_owners(1)),
        Some(Variable::from(2).with_owners(1)),
        Some(Variable::from(3).with_owners(1)),
        Some(Variable::from(4).with_owners(1)),
    ];

    let storage_len = storage.len();

    for variable in &mut storage {
        if let Some(variable) = variable {
            variable.owners = 1;
        }
    }

    let mut variable_collection = Variable::collection(0);
    variable_collection.owners = 1;

    let gvs = KsDriver::gvs_storage(Some(storage), None, None, None);

    let acc = Stack::from(vec![3, 2, 1, 0]);

    let runner = KsDriver::runner_default(Some(acc), None, false, None, None);
    let driver =
        KsDriver::runner_configured(runner, gvs, Instruction::LoadCollection(storage_len))?;

    assert_eq!(driver.runner.acc.len(), 1);
    assert_eq!(driver.gvs.storage.len(), 5);

    let gvs_variable_collection = driver.gvs.storage[4].clone().unwrap();

    assert_eq!(gvs_variable_collection, variable_collection);

    let gvs_collection = &driver.gvs.collections[0];
    assert_eq!(gvs_collection, &collection);

    Ok(())
}

#[test]
fn store() -> KsResult<()> {
    let storage = vec![Some(Variable::from(10).with_owners(1))];
    let gvs = KsDriver::gvs_storage(Some(storage), None, None, None);

    let acc = Stack::from(vec![0]);
    let runner = KsDriver::runner_default(Some(acc), None, false, None, None);

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::Store)?;

    assert_eq!(driver.runner.program_counter, 1);
    assert_eq!(driver.runner.acc.len(), 0);

    assert_eq!(driver.runner.stack.len(), 1);
    assert_eq!(driver.runner.stack.get(0).unwrap(), &0);

    Ok(())
}

#[test]
fn free_primitive() -> KsResult<()> {
    let storage = vec![
        Some(Variable::from(10).with_owners(1)),
        Some(Variable::from(20).with_owners(1)),
        Some(Variable::from(30).with_owners(1)),
    ];

    let gvs = KsDriver::gvs_storage(Some(storage), None, None, None);

    let stack = Stack::from(vec![0, 1, 2]);
    let runner = KsDriver::runner_default(None, Some(stack), false, None, None);

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::Free(3))?;

    assert_eq!(driver.runner.program_counter, 1);

    assert_eq!(driver.runner.stack.len(), 0);

    assert_eq!(driver.gvs.storage.len(), 3);
    assert_eq!(driver.gvs.storage, vec![None, None, None]);

    assert_eq!(driver.gvs.free_storage.len(), 3);
    assert_eq!(driver.gvs.free_storage, vec![2, 1, 0]);

    Ok(())
}

#[test]
fn free_string() -> KsResult<()> {
    let storage = vec![Some(Variable::string(0).with_owners(1))];
    let collections = vec![Collection::String(String::from("Hello World"))];

    let gvs = KsDriver::gvs_storage(Some(storage), Some(collections), None, None);

    let stack = Stack::from(vec![0]);
    let runner = KsDriver::runner_default(None, Some(stack), false, None, None);

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::Free(1))?;

    assert_eq!(driver.runner.program_counter, 1);

    assert_eq!(driver.runner.stack.len(), 0);

    assert_eq!(driver.gvs.storage.len(), 1);
    assert_eq!(driver.gvs.storage, vec![None]);

    assert_eq!(driver.gvs.free_storage.len(), 1);
    assert_eq!(driver.gvs.free_storage, vec![0]);

    assert_eq!(driver.gvs.collections, vec![Collection::Free]);
    assert_eq!(driver.gvs.free_collection.len(), 1);
    assert_eq!(driver.gvs.free_collection, vec![0]);

    Ok(())
}

#[test]
fn free_collection() -> KsResult<()> {
    let storage = vec![
        Some(Variable::from(10).with_owners(1)),
        Some(Variable::from(20).with_owners(1)),
        Some(Variable::from(30).with_owners(1)),
        Some(Variable::collection(0).with_owners(1)),
    ];

    let collections = vec![Collection::Stack(vec![0, 1, 2])];

    let gvs = KsDriver::gvs_storage(Some(storage), Some(collections), None, None);

    let stack = Stack::from(vec![3]);
    let runner = KsDriver::runner_default(None, Some(stack), false, None, None);

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::Free(1))?;

    assert_eq!(driver.runner.program_counter, 1);

    assert_eq!(driver.runner.stack.len(), 0);

    assert_eq!(driver.gvs.storage.len(), 4);
    assert_eq!(driver.gvs.storage, vec![None, None, None, None]);

    assert_eq!(driver.gvs.free_storage.len(), 4);
    assert_eq!(driver.gvs.free_storage, vec![0, 1, 2, 3]);

    assert_eq!(driver.gvs.collections, vec![Collection::Free]);
    assert_eq!(driver.gvs.free_collection.len(), 1);
    assert_eq!(driver.gvs.free_collection, vec![0]);

    Ok(())
}

#[test]
fn free_collection_matrix() -> KsResult<()> {
    let storage = vec![
        Some(Variable::from(10).with_owners(1)),
        Some(Variable::from(20).with_owners(1)),
        Some(Variable::from(30).with_owners(1)),
        Some(Variable::collection(0).with_owners(1)),
        Some(Variable::from(10).with_owners(1)),
        Some(Variable::from(20).with_owners(1)),
        Some(Variable::from(30).with_owners(1)),
        Some(Variable::collection(1).with_owners(1)),
        Some(Variable::from(10).with_owners(1)),
        Some(Variable::from(20).with_owners(1)),
        Some(Variable::from(30).with_owners(1)),
        Some(Variable::collection(2).with_owners(1)),
        Some(Variable::collection(3).with_owners(1)),
    ];

    let collections = vec![
        Collection::Stack(vec![0, 1, 2]),
        Collection::Stack(vec![4, 5, 6]),
        Collection::Stack(vec![8, 9, 10]),
        Collection::Stack(vec![3, 7, 11]),
    ];

    let gvs = KsDriver::gvs_storage(Some(storage), Some(collections), None, None);

    let stack = Stack::from(vec![12]);
    let runner = KsDriver::runner_default(None, Some(stack), false, None, None);

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::Free(1))?;

    assert_eq!(driver.runner.program_counter, 1);

    assert_eq!(driver.runner.stack.len(), 0);

    assert_eq!(driver.gvs.storage.len(), 13);
    assert_eq!(
        driver.gvs.storage,
        vec![
            None, None, None, None, None, None, None, None, None, None, None, None, None
        ]
    );

    assert_eq!(driver.gvs.free_storage.len(), 13);
    assert_eq!(
        driver.gvs.free_storage,
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
    );

    assert_eq!(
        driver.gvs.collections,
        vec![
            Collection::Free,
            Collection::Free,
            Collection::Free,
            Collection::Free
        ]
    );
    assert_eq!(driver.gvs.free_collection.len(), 4);
    assert_eq!(driver.gvs.free_collection, vec![0, 1, 2, 3]);

    Ok(())
}

#[test]
fn clear_acc() -> KsResult<()> {
    let storage = vec![
        Some(Variable::from(10).with_owners(1)),
        Some(Variable::from(20).with_owners(1)),
        Some(Variable::from(30).with_owners(1)),
        Some(Variable::collection(0).with_owners(1)),
    ];

    let collections = vec![Collection::Stack(vec![0, 1, 2])];

    let gvs = KsDriver::gvs_storage(Some(storage), Some(collections), None, None);

    let acc = Stack::from(vec![3]);
    let runner = KsDriver::runner_default(Some(acc), None, false, None, None);

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::ClearAcc)?;

    assert_eq!(driver.runner.program_counter, 1);

    assert_eq!(driver.runner.acc.len(), 0);

    assert_eq!(driver.gvs.storage.len(), 4);
    assert_eq!(driver.gvs.storage, vec![None, None, None, None]);

    assert_eq!(driver.gvs.free_storage.len(), 4);
    assert_eq!(driver.gvs.free_storage, vec![0, 1, 2, 3]);

    assert_eq!(driver.gvs.collections, vec![Collection::Free]);
    assert_eq!(driver.gvs.free_collection.len(), 1);
    assert_eq!(driver.gvs.free_collection, vec![0]);

    Ok(())
}

#[test]
fn jump_if_false_if_actually_false() -> KsResult<()> {
    let condition = Variable::from(false).with_owners(1);

    let gvs = KsDriver::gvs_storage(Some(vec![Some(condition)]), None, None, None);
    let acc = Stack::from(vec![0]);

    let runner = KsDriver::runner_default(Some(acc), None, false, None, None);
    let jump_offset = 32;

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::JumpIfFalse(jump_offset))?;

    assert_eq!(driver.runner.program_counter(), jump_offset as usize);
    assert_eq!(driver.runner.prevent_step, false);

    assert_eq!(driver.runner.acc.len(), 0);

    Ok(())
}

#[test]
fn jump_if_false_if_actually_true() -> KsResult<()> {
    let condition = Variable::from(true).with_owners(1);

    let gvs = KsDriver::gvs_storage(Some(vec![Some(condition)]), None, None, None);
    let acc = Stack::from(vec![0]);

    let runner = KsDriver::runner_default(Some(acc), None, false, None, None);
    let jump_offset = 32;

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::JumpIfFalse(jump_offset))?;

    assert_eq!(driver.runner.program_counter(), 1);
    assert_eq!(driver.runner.prevent_step, false);

    assert_eq!(driver.runner.acc.len(), 0);

    Ok(())
}

#[test]
fn jump_if_true_if_actually_false() -> KsResult<()> {
    let condition = Variable::from(false).with_owners(1);

    let gvs = KsDriver::gvs_storage(Some(vec![Some(condition)]), None, None, None);
    let acc = Stack::from(vec![0]);

    let runner = KsDriver::runner_default(Some(acc), None, false, None, None);
    let jump_offset = 32;

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::JumpIfTrue(jump_offset))?;

    assert_eq!(driver.runner.program_counter(), 1);
    assert_eq!(driver.runner.prevent_step, false);

    assert_eq!(driver.runner.acc.len(), 0);

    Ok(())
}

#[test]
fn jump_if_true_if_actually_true() -> KsResult<()> {
    let condition = Variable::from(true).with_owners(1);

    let gvs = KsDriver::gvs_storage(Some(vec![Some(condition)]), None, None, None);
    let acc = Stack::from(vec![0]);

    let runner = KsDriver::runner_default(Some(acc), None, false, None, None);
    let jump_offset = 32;

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::JumpIfTrue(jump_offset))?;

    assert_eq!(driver.runner.program_counter(), jump_offset as Pointer);
    assert_eq!(driver.runner.prevent_step, false);

    assert_eq!(driver.runner.acc.len(), 0);

    Ok(())
}

#[test]
fn call() -> KsResult<()> {
    let storage = vec![Some(Variable::from(Function::from(20u32)).with_owners(1))];

    let gvs = KsDriver::gvs_storage(Some(storage), None, None, None);

    let acc = vec![0];
    let runner = KsDriver::runner_default(Some(Stack::from(acc)), None, false, None, None);

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::Call)?;

    assert_eq!(driver.runner.program_counter, 20);
    assert_eq!(driver.runner.call_stack.len(), 1);
    assert_eq!(driver.runner.call_stack[0].return_pointer, 0);
    assert_eq!(driver.runner.call_stack[0].collection_id, 0);

    assert_eq!(driver.runner.acc.len(), 0);

    Ok(())
}

#[test]
fn return_instruction() -> KsResult<()> {
    let initial_pc = 20usize;

    let call_stack = CallStack::new(0, 0);

    let runner =
        KsDriver::runner_default(None, None, false, Some(initial_pc), Some(vec![call_stack]));

    let driver = KsDriver::runner_configured(runner, None, Instruction::Return)?;

    assert_eq!(driver.runner.call_stack.len(), 0);
    assert_eq!(driver.runner.program_counter, 1);

    Ok(())
}

#[test]
fn load_function_empty() -> KsResult<()> {
    let function = Variable::from(Function::from(20u32)).with_owners(1);

    let storage = vec![Some(function.clone())];
    let gvs = KsDriver::gvs_storage(Some(storage), None, None, None);

    let acc = vec![0];
    let runner = KsDriver::runner_default(Some(Stack::from(acc)), None, false, None, None);

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::LoadFunction(0))?;

    assert_eq!(driver.runner.program_counter, 1);
    assert_eq!(driver.gvs.storage[0], Some(function));

    assert_eq!(driver.runner.acc.len(), 1);
    assert_eq!(driver.runner.acc.get(0), Some(&0));

    Ok(())
}

#[test]
fn load_function_capture() -> KsResult<()> {
    let function = Variable::function(Function::new(20, 0)).with_owners(1);

    let storage = vec![
        Some(Variable::from(20).with_owners(1)),
        Some(Variable::from(1).with_owners(1)),
        Some(Variable::from(2).with_owners(1)),
    ];
    let gvs = KsDriver::gvs_storage(Some(storage), None, None, None);

    let acc = vec![0, 2, 1];
    let runner = KsDriver::runner_default(Some(Stack::from(acc)), None, false, None, None);

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::LoadFunction(2))?;

    assert_eq!(driver.runner.program_counter, 1);
    assert_eq!(driver.gvs.storage[0], Some(function));

    assert_eq!(driver.runner.acc.len(), 1);
    assert_eq!(driver.runner.acc.get(0), Some(&0));

    assert_eq!(driver.gvs.collections.len(), 1);
    assert_eq!(driver.gvs.collections[0], Collection::Stack(vec![1, 2]));

    Ok(())
}

#[test]
fn free_function_with_capture() -> KsResult<()> {
    let storage = vec![
        Some(Variable::from(10).with_owners(1)),
        Some(Variable::from(20).with_owners(1)),
        Some(Variable::from(30).with_owners(1)),
        Some(Variable::from(Function::new(10, 0)).with_owners(1)),
    ];

    let collections = vec![Collection::Stack(vec![0, 1, 2])];

    let gvs = KsDriver::gvs_storage(Some(storage), Some(collections), None, None);

    let stack = Stack::from(vec![3]);
    let runner = KsDriver::runner_default(None, Some(stack), false, None, None);

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::Free(1))?;

    assert_eq!(driver.runner.program_counter, 1);

    assert_eq!(driver.runner.stack.len(), 0);

    assert_eq!(driver.gvs.storage.len(), 4);
    assert_eq!(driver.gvs.storage, vec![None, None, None, None]);

    assert_eq!(driver.gvs.free_storage.len(), 4);
    assert_eq!(driver.gvs.free_storage, vec![0, 1, 2, 3]);

    assert_eq!(driver.gvs.collections, vec![Collection::Free]);
    assert_eq!(driver.gvs.free_collection.len(), 1);
    assert_eq!(driver.gvs.free_collection, vec![0]);

    Ok(())
}

#[test]
fn call_stack_should_own_collection() -> KsResult<()> {
    let storage = vec![
        Some(Variable::from(10).with_owners(1)),
        Some(Variable::from(20).with_owners(1)),
        Some(Variable::from(30).with_owners(1)),
        Some(Variable::from(Function::new(20u32, 0u32)).with_owners(1)),
    ];

    let collection = vec![Collection::Stack(vec![0, 1, 2])];

    let gvs = KsDriver::gvs_storage(Some(storage), Some(collection), None, None);

    let acc = vec![3];
    let runner = KsDriver::runner_default(Some(Stack::from(acc)), None, false, None, None);

    let driver = KsDriver::runner_configured(runner, gvs, Instruction::Call)?;

    assert_eq!(driver.runner.program_counter, 20);
    assert_eq!(driver.runner.call_stack.len(), 1);
    assert_eq!(driver.runner.call_stack[0].return_pointer, 0);
    assert_eq!(driver.runner.call_stack[0].collection_id, 0);

    assert_eq!(driver.runner.acc.len(), 0);

    assert_eq!(driver.gvs.collections.len(), 1);
    assert_eq!(driver.gvs.collections[0], Collection::Stack(vec![0, 1, 2]));

    Ok(())
}
