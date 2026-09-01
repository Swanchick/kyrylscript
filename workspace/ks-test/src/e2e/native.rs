use std::cell::RefCell;
use std::rc::Rc;

use ks_vm_new::{FLOAT_TYPE, INT_TYPE, KsCall, NativeHelper, STRING_TYPE, VMResult};

pub struct MockPrintLn {
    pub output: Rc<RefCell<String>>,
}

impl From<Rc<RefCell<String>>> for MockPrintLn {
    fn from(output: Rc<RefCell<String>>) -> Self {
        Self { output }
    }
}

impl KsCall for MockPrintLn {
    fn call(&mut self, arguments: usize, helper: NativeHelper) -> VMResult<()> {
        let gvs = helper.gvs;

        let mut output = self.output.borrow_mut();

        for _ in 0..arguments {
            let argument = helper.runner.acc.last(gvs)?.clone();

            match argument.value_type {
                INT_TYPE => output.push_str(&(argument.value as i64).to_string()),
                FLOAT_TYPE => output.push_str(&(f64::from_bits(argument.value)).to_string()),
                STRING_TYPE => output.push_str(gvs.collection_string(argument.value as u32)?),
                _ => {}
            }

            helper.runner.acc.pop_data()?;
        }

        Ok(())
    }
}
