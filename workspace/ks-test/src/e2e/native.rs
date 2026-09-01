use std::cell::RefCell;
use std::rc::Rc;

use ks_vm_new::{KsCall, NativeHelper, STRING_TYPE, VMResult, types::CollectionId};

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

        let mut storage_ids = helper.runner.acc.size_pop(arguments as u32);
        storage_ids.reverse();
        for storage_id in storage_ids {
            println!("STORAGE_ID: {}", storage_id);

            let variable = gvs.variable(storage_id)?;
            if variable.value_type != STRING_TYPE {
                gvs.storage_remove_owner(storage_id)?;
                continue;
            }

            let string = gvs.collection_string(variable.value as CollectionId)?;

            let mut output = self.output.borrow_mut();
            output.push_str(string);

            gvs.storage_remove_owner(storage_id)?;
        }

        Ok(())
    }
}
