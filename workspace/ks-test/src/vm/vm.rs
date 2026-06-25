use std::cell::RefCell;
use std::rc::Rc;

use ks_global::utils::ks_result::KsResult;

use ks_vm_new::{Constant, Instruction};
use ks_vm_new::{KsCall, NativeHelper, NativeRegistry, STRING_TYPE};

use crate::drivers::KsDriver;

struct TestPrint {
    output: Rc<RefCell<String>>,
}

impl<'a> KsCall<'a> for TestPrint {
    fn call(&mut self, arguments: usize, helper: NativeHelper<'a>) -> KsResult<()> {
        let gvs = helper.gvs;

        for _ in 0..arguments {
            let variable = helper.runner.acc.pop(gvs)?;
            if variable.value_type != STRING_TYPE {
                continue;
            }

            let string = gvs.collection_string(variable.value)?;

            let mut output = self.output.borrow_mut();
            output.push_str(string);
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

    assert_eq!(vm.runners[0].program_counter, 1);

    let output_string = output.borrow().clone();
    assert_eq!(output_string, String::from("Hello, world!"));

    Ok(())
}
