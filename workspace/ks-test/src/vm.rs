use ks_global::utils::ks_result::KsResult;
use ks_vm::variable::value::Value;

use crate::drivers::KsDriver;

#[test]
fn call_stack_optimization() {
    let driver = KsDriver::new("vm/call_stack_optimization.ks");
    let result = driver.run();
    assert!(result.is_ok());
}

#[test]
fn native_call() {
    let driver = KsDriver::new("vm/native_call.ks");
    let result = driver.call_null("super_productive");
    assert!(result.is_ok());
}

#[test]
fn native_call_with_return() -> KsResult<()> {
    let driver = KsDriver::new("vm/native_call_with_return.ks");
    let result = driver.call("return_number_10")?;

    assert!(matches!(result.value(), Value::Integer(10)));

    Ok(())
}
