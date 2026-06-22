use std::collections::HashMap;

use ks_global::utils::ks_result::KsResult;

use crate::Instruction;

use super::Program;
use super::environment::GVS;
use super::runner::Runner;

pub struct VM {
    program: Program,
    pub runners: Vec<Runner>,
    pub gvs: GVS,
}

impl From<Vec<Instruction>> for VM {
    fn from(instructions: Vec<Instruction>) -> Self {
        Self {
            program: Program::new(instructions, HashMap::new()),
            runners: Vec::new(),
            gvs: GVS::new(),
        }
    }
}

impl From<Program> for VM {
    fn from(program: Program) -> Self {
        Self {
            program,
            runners: Vec::new(),
            gvs: GVS::new(),
        }
    }
}

impl VM {
    fn create_thread(&mut self) {
        let runner = Runner::new();
        self.runners.push(runner);
    }

    pub fn step(&mut self) -> KsResult<()> {
        let instructions = self.program.instructions();

        for index in 0..self.runners.len() {
            let runner = &mut self.runners[index];
            let pc = runner.program_counter();

            if let Some(instruction) = instructions.get(pc as usize) {
                let instruction = instruction.clone();
                runner.run(instruction, &mut self.gvs)?;
            }
        }

        Ok(())
    }

    pub fn init(&mut self) {
        self.create_thread();
    }
}
