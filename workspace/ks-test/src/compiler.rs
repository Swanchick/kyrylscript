use ks_global::utils::ks_result::KsResult;
use ks_vm::function::Function;

use crate::drivers::KsDriver;

#[test]
fn create_main_function() -> KsResult<()> {
    let output = vec![Function::method(Vec::new())];

    let driver = KsDriver::new("compiler/create_main_function.ks");
    let program = driver.compiler_new()?;

    assert_eq!(program, output);

    Ok(())
}
