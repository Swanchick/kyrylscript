use std::cell::RefCell;
use std::rc::Rc;

use ks_core::kyryl_script::KyrylScript;
use ks_core::parser::data_type::DataType;
use ks_global::utils::ks_result::KsResult;

use crate::drivers::KsDriver;
use crate::e2e::native::MockPrintLn;

mod native;

fn run(path: &str) -> KsResult<String> {
    let mut kyrylscript = KyrylScript::new();
    kyrylscript.parser_mut().register_variable(
        "print",
        DataType::RustFunction {
            return_type: Box::new(DataType::void()),
        },
        true,
    );
    kyrylscript.compiler_mut().register_native("print", 0);

    let bytes = KsDriver::compiler(kyrylscript, path)?;
    let output = Rc::new(RefCell::new(String::new()));

    KsDriver::vm(bytes, vec![Box::new(MockPrintLn::from(output.clone()))])?;

    Ok(output.borrow().clone())
}

#[test]
fn if_statement() -> KsResult<()> {
    let output = run("e2e/if_statement.ks")?;
    assert_eq!(output, "023");

    Ok(())
}

#[test]
fn while_statement() -> KsResult<()> {
    let output = run("e2e/while_statement.ks")?;
    assert_eq!(output, "7\\ 7\\ 7\\ 7\\ 7\\ ");

    Ok(())
}

#[test]
fn function_call() -> KsResult<()> {
    let output = run("e2e/function_call.ks")?;
    assert_eq!(output, "30");
    Ok(())
}
