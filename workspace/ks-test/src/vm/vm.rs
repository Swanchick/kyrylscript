use std::cell::RefCell;
use std::rc::Rc;

use ks_global::utils::ks_result::KsResult;

use ks_vm_new::{Constant, Instruction};
use ks_vm_new::{KsCall, NativeHelper, NativeRegistry, STRING_TYPE};

use crate::drivers::KsDriver;

struct TestPrint {
    output: Rc<RefCell<String>>,
}

impl KsCall for TestPrint {
    fn call(&mut self, arguments: usize, helper: NativeHelper) -> KsResult<()> {
        let gvs = helper.gvs;

        let mut storage_ids = helper.runner.acc.size_pop(arguments);
        storage_ids.reverse();
        for storage_id in storage_ids {
            let variable = gvs.variable(storage_id)?;
            if variable.value_type != STRING_TYPE {
                gvs.storage_remove_owner(storage_id)?;
                continue;
            }

            let string = gvs.collection_string(variable.value)?;

            let mut output = self.output.borrow_mut();
            output.push_str(string);

            gvs.storage_remove_owner(storage_id)?;
        }

        Ok(())
    }
}

#[test]
fn call_native() -> KsResult<()> {
    let mut native = NativeRegistry::new();
    let output = Rc::new(RefCell::new(String::new()));
    native.functions.push(Box::new(TestPrint {
        output: output.clone(),
    }));

    let instructions = vec![
        Instruction::LoadConst(Constant::String(String::from("Hello, "))),
        Instruction::LoadConst(Constant::String(String::from("world!"))),
        Instruction::CallNative(0, 2),
    ];

    let vm = KsDriver::vm_configured(None, None, Some(native), instructions)?;

    // assert_eq!(vm.runners[0].program_counter, 1);

    let output_string = output.borrow().clone();
    assert_eq!(output_string, String::from("Hello, world!"));

    Ok(())
}
