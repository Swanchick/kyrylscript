use std::cell::RefCell;
use std::rc::Rc;

use ks_core::kyryl_script::KyrylScript;
use ks_core::parser::data_type::DataType;
use ks_global::utils::ks_result::KsResult;

use crate::drivers::KsDriver;
use crate::e2e::native::MockPrintLn;

mod native;

#[test]
fn if_statement() -> KsResult<()> {
    let mut kyrylscript = KyrylScript::new();
    kyrylscript.parser_mut().register_variable(
        "println",
        DataType::RustFunction {
            return_type: Box::new(DataType::void()),
        },
        true,
    );
    kyrylscript.compiler_mut().register_native("println", 0);

    let bytes = KsDriver::compiler(kyrylscript, "e2e/if_statement.ks")?;
    let output = Rc::new(RefCell::new(String::new()));

    KsDriver::vm(bytes, vec![Box::new(MockPrintLn::from(output.clone()))])?;

    assert_eq!(output.borrow().clone(), "worldBU BU BU BA!");

    Ok(())
}
