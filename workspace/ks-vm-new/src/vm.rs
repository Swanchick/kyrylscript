#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

use crate::{GVS, Instruction, KsCall, NativeCall, NativeRegistry, Program, Runner, VMResult};

pub struct VM {
    program: Program,
    pub runners: Vec<Runner>,
    pub gvs: GVS,
    pub native: NativeRegistry,
}

impl From<Vec<Instruction>> for VM {
    fn from(instructions: Vec<Instruction>) -> Self {
        Self {
            program: Program::new(instructions),
            runners: Vec::new(),
            gvs: GVS::new(),
            native: NativeRegistry::new(),
        }
    }
}

impl From<Program> for VM {
    fn from(program: Program) -> Self {
        Self {
            program,
            runners: Vec::new(),
            gvs: GVS::new(),
            native: NativeRegistry::new(),
        }
    }
}

impl VM {
    pub fn new(program: Program, runners: Vec<Runner>, gvs: GVS, native: NativeRegistry) -> Self {
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
        let instructions = self.program.instructions();
        let mut native_calls = Vec::new();

        for runner_id in 0..self.runners.len() {
            let runner = &mut self.runners[runner_id];
            let pc = runner.program_counter();

            if let Some(instruction) = instructions.get(pc as usize) {
                let instruction = instruction.clone();
                runner.run(runner_id, instruction, &mut self.gvs, &mut native_calls)?;
            }
        }

        while let Some(native_call) = native_calls.pop() {
            self.call_native(native_call)?;
        }

        Ok(())
    }

    pub fn add_native(&mut self, native: Box<dyn KsCall>) {
        self.native.functions.push(native);
    }

    pub fn init(&mut self) {
        self.create_thread();
    }
}
