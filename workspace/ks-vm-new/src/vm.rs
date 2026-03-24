use ks_global::utils::ks_result::KsResult;

use super::Program;
use super::runner::Runner;
use super::types::Stack;
use super::vgs::VGS;

pub struct VM {
    program: Program,
    runners: Vec<Runner>,
    vgs: VGS,
    stacks: Vec<Stack>,
}

impl From<Program> for VM {
    fn from(program: Program) -> Self {
        Self {
            program,
            runners: Vec::new(),
            vgs: VGS {},
            stacks: Vec::new(),
        }
    }
}

impl VM {
    fn create_thread(&mut self) {
        let runner = Runner::new();
        self.runners.push(runner);
    }

    fn runner_loop(&mut self) -> KsResult<()> {
        let instructions = self.program.instructions();

        for runner in &mut self.runners {
            let pc = runner.program_counter();

            if let Some(instruction) = instructions.get(pc) {
                let instruction = instruction.clone();
                runner.step(instruction, &mut self.vgs)?;
            }
        }

        Ok(())
    }

    pub fn init(&mut self) {
        self.create_thread();
    }
}
