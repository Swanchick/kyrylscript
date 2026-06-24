use std::collections::HashMap;

use ks_global::utils::ks_result::KsResult;

use crate::{Instruction, NativeRegistry, Program, Runner};

use super::environment::GVS;

pub struct VM<'a> {
    program: Program,
    pub runners: Vec<Runner>,
    pub gvs: GVS,
    pub native: NativeRegistry<'a>,
}

impl<'a> From<Vec<Instruction>> for VM<'a> {
    fn from(instructions: Vec<Instruction>) -> Self {
        Self {
            program: Program::new(instructions, HashMap::new()),
            runners: Vec::new(),
            gvs: GVS::new(),
            native: NativeRegistry::new(),
        }
    }
}

impl<'a> From<Program> for VM<'a> {
    fn from(program: Program) -> Self {
        Self {
            program,
            runners: Vec::new(),
            gvs: GVS::new(),
            native: NativeRegistry::new(),
        }
    }
}

impl<'a> VM<'a> {
    pub fn new(
        program: Program,
        runners: Vec<Runner>,
        gvs: GVS,
        native: NativeRegistry<'a>,
    ) -> Self {
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

    pub fn step(&mut self) -> KsResult<()> {
        let instructions = self.program.instructions();

        for index in 0..self.runners.len() {
            let runner = &mut self.runners[index];
            let pc = runner.program_counter();
            let mut native_calls = Vec::new();

            if let Some(instruction) = instructions.get(pc as usize) {
                let instruction = instruction.clone();
                runner.run(instruction, &mut self.gvs, &mut native_calls)?;
            }
        }

        Ok(())
    }

    pub fn init(&mut self) {
        self.create_thread();
    }
}
