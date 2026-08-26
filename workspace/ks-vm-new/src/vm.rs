#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

use crate::{GVS, KsCall, NativeCall, NativeRegistry, Runner, VMHelper, VMResult};

pub struct VM {
    program: Box<[u8]>,
    pub runners: Vec<Runner>,
    pub gvs: GVS,
    pub native: NativeRegistry,
}

impl From<Box<[u8]>> for VM {
    fn from(program: Box<[u8]>) -> Self {
        Self {
            program,
            runners: Vec::new(),
            gvs: GVS::new(),
            native: NativeRegistry::new(),
        }
    }
}

impl VM {
    pub fn new(program: Box<[u8]>, runners: Vec<Runner>, gvs: GVS, native: NativeRegistry) -> Self {
        Self {
            program,
            runners,
            gvs,
            native,
        }
    }

    fn create_thread(&mut self) {
        let runner = Runner::new();
        self.runners.push(runner);
    }

    fn call_native(&mut self, native_call: NativeCall) -> VMResult<()> {
        self.native.call(
            native_call.native_id,
            native_call.arguments,
            &mut self.runners[native_call.runner_id],
            &mut self.gvs,
        )?;

        Ok(())
    }

    pub fn step(&mut self) -> VMResult<()> {
        let instructions = &self.program;
        let mut native_stack = Vec::new();
        let mut empty_runner_ids = Vec::new();

        for runner_id in 0..self.runners.len() {
            let runner = &mut self.runners[runner_id];
            let pc = runner.pc;

            if let Some(instruction) = instructions.get(pc) {
                let instruction = *instruction;
                let vm_helper = VMHelper {
                    instruction,
                    instructions,
                    gvs: &mut self.gvs,
                    native_stack: &mut native_stack,
                    runner_id,
                };

                runner.run(vm_helper)?;
            } else {
                empty_runner_ids.push(runner_id);
            }
        }

        while let Some(native_call) = native_stack.pop() {
            self.call_native(native_call)?;
        }

        while let Some(runner_id) = empty_runner_ids.pop() {
            self.runners.remove(runner_id);
        }

        Ok(())
    }

    pub fn reset(&mut self, program: Box<[u8]>) {
        self.runners.clear();
        self.gvs = GVS::new();
        self.program = program;
    }

    pub fn is_empty(&self) -> bool {
        self.runners.is_empty()
    }

    pub fn add_native(&mut self, native: Box<dyn KsCall>) {
        self.native.functions.push(native);
    }

    pub fn init(&mut self) {
        self.create_thread();
    }
}
