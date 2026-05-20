use ks_core::compiler_new::compiler::CompilerNew;
use ks_core::lexer::lexer::Lexer;
use ks_core::parser::parser::Parser;
use ks_core::parser::statement::Statement;
use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;
use ks_std::ks_register_std;
use ks_vm_new::{Collection, GVS, Instruction, Runner, Stack, Variable};

use super::runner_driver::RunnerDriver;

pub struct KsDriver {
    path: String,
}

impl KsDriver {
    pub fn new(path: &str) -> KsDriver {
        let path = format!("tests/{}", path);

        KsDriver { path }
    }

    pub fn lexer(&self) -> KsResult<Lexer> {
        let mut lexer = Lexer::load(&self.path)?;
        lexer.lexer()?;
        Ok(lexer)
    }

    pub fn parser(&self) -> KsResult<Vec<Statement>> {
        let lexer = self.lexer()?;
        let mut parser = Parser::new();
        parser.set_tokens(lexer.get_tokens().to_vec(), lexer.get_token_pos().to_vec());
        ks_register_std(&mut parser);
        let statements = parser.start()?;

        Ok(statements)
    }

    pub fn parser_with_parser(&self) -> KsResult<Parser> {
        let lexer = self.lexer()?;
        let mut parser = Parser::new();
        parser.set_tokens(lexer.get_tokens().to_vec(), lexer.get_token_pos().to_vec());
        ks_register_std(&mut parser);

        Ok(parser)
    }

    pub fn compiler_new(&self) -> KsResult<CompilerNew> {
        let statements = self.parser()?;
        let mut compiler = CompilerNew::new();
        compiler.compile(statements)?;

        Ok(compiler)
    }

    pub fn runner(instruction: Instruction) -> KsResult<RunnerDriver> {
        let mut gvs = GVS::new();
        let mut runner = Runner::new();

        runner.run(instruction, &mut gvs)?;

        Ok(RunnerDriver::new(runner, gvs))
    }

    pub fn gvs_storage(
        storage: Option<Vec<Option<Variable>>>,
        collections: Option<Vec<Collection>>,
        free_storage: Option<Vec<usize>>,
        free_collection: Option<Vec<usize>>,
    ) -> Option<GVS> {
        let storage = if let Some(storage) = storage {
            storage
        } else {
            Vec::new()
        };

        let collections = if let Some(collections) = collections {
            collections
        } else {
            Vec::new()
        };

        let free_storage = if let Some(free_storage) = free_storage {
            free_storage
        } else {
            Vec::new()
        };

        let free_collection = if let Some(free_collection) = free_collection {
            free_collection
        } else {
            Vec::new()
        };

        Some(GVS {
            storage,
            collections,
            free_storage,
            free_collection,
        })
    }

    pub fn runner_default(
        acc: Option<Stack>,
        stack: Option<Stack>,
        prevent_step: bool,
        program_counter: Option<usize>,
    ) -> Option<Runner> {
        let acc = if let Some(acc) = acc {
            acc
        } else {
            Stack::new()
        };
        let stack = if let Some(stack) = stack {
            stack
        } else {
            Stack::new()
        };
        let program_counter = if let Some(program_counter) = program_counter {
            program_counter
        } else {
            0
        };

        Some(Runner {
            program_counter,
            acc,
            stack,
            call_stack: Vec::new(),
            prevent_step,
        })
    }

    pub fn runner_configured(
        runner: Option<Runner>,
        gvs: Option<GVS>,
        instruction: Instruction,
    ) -> KsResult<RunnerDriver> {
        let mut gvs = if let Some(gvs) = gvs { gvs } else { GVS::new() };
        let mut runner = if let Some(runner) = runner {
            runner
        } else {
            Runner::new()
        };

        runner.run(instruction, &mut gvs)?;
        Ok(RunnerDriver::new(runner, gvs))
    }

    pub fn operation_test(
        left: Variable,
        right: Variable,
        result: Variable,
        instruction: Instruction,
    ) -> KsResult<()> {
        let runner = KsDriver::runner_default(
            Some(Stack::from(vec![0, 1])),
            Some(Stack::from(vec![0, 1])),
            false,
            None,
        );
        let gvs = KsDriver::gvs_storage(Some(vec![Some(left), Some(right)]), None, None, None);

        let driver = KsDriver::runner_configured(runner, gvs, instruction)?;

        if driver.runner.program_counter != 1 {
            return Err(KsError::runtime("Wrong program_counter"));
        }

        if driver.runner.acc.len() != 1 {
            return Err(KsError::runtime("Wrong acc size"));
        }

        if driver.runner.acc.get(0).unwrap() != &2 {
            return Err(KsError::runtime("Acc doesn't have the variable"));
        }

        let gvs_variable1_left = driver.gvs.storage[0].clone().unwrap();
        let gvs_variable1_right = driver.gvs.storage[1].clone().unwrap();
        let gvs_variable1_result = driver.gvs.storage[2].clone().unwrap();

        if gvs_variable1_left.owners != 1 {
            return Err(KsError::runtime("Left varaible has wrong amount of owners"));
        }

        if gvs_variable1_right.owners != 1 {
            return Err(KsError::runtime(
                "Right varaible has wrong amount of owners",
            ));
        }

        if gvs_variable1_result != result {
            return Err(KsError::runtime(&format!(
                "Wrong result {:?}",
                gvs_variable1_result
            )));
        }

        Ok(())
    }
}
