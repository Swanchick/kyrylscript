#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::{
    GVS, KsCall, NativeCall, NativeRegistry, Runner, VMHelper, VMResult,
    runner::runner_status::RunnerStatus,
};

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

    fn call_native(&mut self, native_call: Option<NativeCall>) -> VMResult<()> {
        if let Some(native_call) = native_call {
            self.native.call(
                native_call.native_id as usize,
                native_call.arguments,
                &mut self.runners[native_call.runner_id],
                &mut self.gvs,
            )?;
        }

        Ok(())
    }

    pub fn step(&mut self) -> VMResult<()> {
        let instructions = self.program.clone();
        let mut empty_runner_ids = Vec::new();

        for runner_id in 0..self.runners.len() {
            let runner = &mut self.runners[runner_id];
            let mut native_call = None;
            let vm_helper = VMHelper {
                instructions: &instructions,
                gvs: &mut self.gvs,
                native_call: &mut native_call,
                runner_id,
            };

            let status = runner.run(vm_helper)?;
            if let RunnerStatus::OutOfProgram = status {
                empty_runner_ids.push(runner_id);
            }
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
