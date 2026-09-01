use std::cell::RefCell;
use std::rc::Rc;

use ks_vm_new::{
    BOOLEAN_TYPE, FLOAT_TYPE, INT_TYPE, KsCall, NativeHelper, STRING_TYPE, VMError, VMResult,
};

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

pub struct DigitalWrite {
    pub output: Rc<RefCell<String>>,
}

impl From<Rc<RefCell<String>>> for DigitalWrite {
    fn from(output: Rc<RefCell<String>>) -> Self {
        Self { output }
    }
}

impl KsCall for DigitalWrite {
    fn call<'a>(&mut self, arguments: usize, helper: NativeHelper<'a>) -> VMResult<()> {
        if arguments != 3 {
            return Ok(());
        }

        let gvs = helper.gvs;
        let runner = helper.runner;
        let port = runner.acc.last(gvs)?.clone();
        if port.value_type != STRING_TYPE {
            return Err(VMError::from("Variable is not a string"));
        }
        let port = gvs.collection_string(port.value as u32)?.to_string();

        runner.acc.pop_data()?;

        let pin = runner.acc.pop(gvs)?;
        if pin.value_type != INT_TYPE {
            return Err(VMError::from("Variable is not an int"));
        }
        let pin = format!("P{}{}", port, pin.value);

        let toggle = runner.acc.pop(gvs)?;
        if toggle.value_type != BOOLEAN_TYPE {
            return Err(VMError::from("Variable is not boolean"));
        }

        if toggle.as_boolean() {
            self.output
                .borrow_mut()
                .push_str(&format!("{} -> high;", pin));
        } else {
            self.output
                .borrow_mut()
                .push_str(&format!("{} -> low;", pin));
        }

        Ok(())
    }
}

pub struct Delay {
    pub output: Rc<RefCell<String>>,
}

impl From<Rc<RefCell<String>>> for Delay {
    fn from(output: Rc<RefCell<String>>) -> Self {
        Self { output }
    }
}

impl KsCall for Delay {
    fn call<'a>(&mut self, arguments: usize, helper: NativeHelper<'a>) -> VMResult<()> {
        if arguments != 1 {
            return Ok(());
        }

        let gvs = helper.gvs;
        let variable = helper.runner.acc.pop(gvs)?;

        if variable.value_type != INT_TYPE {
            return Ok(());
        }

        let ms = variable.value as u32;

        self.output
            .borrow_mut()
            .push_str(&format!("delay = {};", ms));
        Ok(())
    }
}
