use ks_core::lexer::lexer::Lexer;
use ks_core::parser::parser::Parser;
use ks_core::parser::statement::Statement;
use ks_core::{
    compiler_new::{compiler::CompilerNew, instructions::Instruction, program::Program},
    kyryl_script::KyrylScript,
};

use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;
use ks_std::ks_register_std;
use ks_vm_new::{
    Assign, CallStack, Collection, GVS, KsCall, NativeRegistry, Runner, Stack, VM, VMError,
    VMHelper, VMResult, Variable,
};

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
        ks_register_std(&mut KyrylScript::new());
        let statements = parser.start()?;

        Ok(statements)
    }

    pub fn parser_with_parser(&self) -> KsResult<Parser> {
        let lexer = self.lexer()?;
        let mut parser = Parser::new();
        parser.set_tokens(lexer.get_tokens().to_vec(), lexer.get_token_pos().to_vec());
        ks_register_std(&mut KyrylScript::new());

        Ok(parser)
    }

    pub fn compiler_new(&self) -> KsResult<CompilerNew> {
        let mut kyryl_script = KyrylScript::new();
        ks_register_std(&mut kyryl_script);
        let mut compiler = kyryl_script.take_compiler();

        let statements = self.parser()?;

        compiler.compile(statements)?;

        Ok(compiler)
    }

    pub fn compiler(mut kyryl_script: KyrylScript, path: &str) -> KsResult<Box<[u8]>> {
        let statements = kyryl_script.statements(&format!("tests/{}", path))?;
        let mut compiler = kyryl_script.take_compiler();
        compiler.compile(statements)?;
        let program = compiler.program();
        let bytes = program.as_bytes();
        Ok(bytes)
    }

    pub fn vm(bytes: Box<[u8]>, mut natives: Vec<Box<dyn KsCall>>) -> KsResult<()> {
        let mut vm = VM::from(bytes);
        while let Some(native) = natives.pop() {
            vm.add_native(native);
        }

        vm.init();

        while !vm.is_empty() {
            vm.step().or_else(|e| Err(KsError::runtime(&e.message)))?;
        }

        Ok(())
    }

    pub fn compiler_new_environment(&self, mut kyryl_script: KyrylScript) -> KsResult<CompilerNew> {
        let statements = kyryl_script.statements(&self.path)?;
        let mut compiler = kyryl_script.take_compiler();

        compiler.compile(statements)?;

        Ok(compiler)
    }

    pub fn runner(instructions: Vec<u8>) -> VMResult<RunnerDriver> {
        let mut gvs = GVS::new();
        let mut runner = Runner::new();

        let vm_helper = VMHelper {
            instruction: instructions[0],
            instructions: &instructions,
            gvs: &mut gvs,
            native_stack: &mut Vec::new(),
            runner_id: 0,
        };

        runner.run(vm_helper)?;

        Ok(RunnerDriver::new(runner, gvs))
    }

    pub fn vm_configured(
        runner: Option<Runner>,
        gvs: Option<GVS>,
        native: Option<NativeRegistry>,
        instructions: Vec<Instruction>,
    ) -> VMResult<VM> {
        let instructions_len = instructions.len();

        let gvs = if let Some(gvs) = gvs { gvs } else { GVS::new() };
        let runner = if let Some(runner) = runner {
            runner
        } else {
            Runner::new()
        };
        let native = if let Some(native) = native {
            native
        } else {
            NativeRegistry::new()
        };

        let mut vm = VM::new(
            Program::serialize(instructions).as_bytes(),
            vec![runner],
            gvs,
            native,
        );

        for _ in 0..instructions_len {
            vm.step()?;
        }

        Ok(vm)
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
        pc: Option<usize>,
        call_stack: Option<Vec<CallStack>>,
        assign: Option<Assign>,
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

        let pc = if let Some(pc) = pc { pc } else { 0 };

        let call_stack = if let Some(call_stack) = call_stack {
            call_stack
        } else {
            Vec::new()
        };

        let assign = if let Some(assign) = assign {
            assign
        } else {
            Assign::None
        };

        Some(Runner {
            pc,
            acc,
            stack,
            call_stack,
            assign,
        })
    }

    pub fn runner_configured(
        runner: Option<Runner>,
        gvs: Option<GVS>,
        instruction: Vec<u8>,
    ) -> VMResult<RunnerDriver> {
        let mut gvs = if let Some(gvs) = gvs { gvs } else { GVS::new() };
        let mut runner = if let Some(runner) = runner {
            runner
        } else {
            Runner::new()
        };

        let vm_helper = VMHelper {
            instruction: instruction[runner.pc],
            instructions: &instruction,
            gvs: &mut gvs,
            native_stack: &mut Vec::new(),
            runner_id: 0,
        };

        runner.run(vm_helper)?;
        Ok(RunnerDriver::new(runner, gvs))
    }

    pub fn operation_test(
        left: Variable,
        right: Variable,
        result: Variable,
        instruction: Vec<u8>,
    ) -> VMResult<()> {
        let runner = KsDriver::runner_default(
            Some(Stack::from(vec![0, 1])),
            Some(Stack::from(vec![0, 1])),
            None,
            None,
            None,
        );
        let gvs = KsDriver::gvs_storage(Some(vec![Some(left), Some(right)]), None, None, None);

        let driver = KsDriver::runner_configured(runner, gvs, instruction)?;

        if driver.runner.pc != 1 {
            return Err(VMError::from("Wrong pc"));
        }

        if driver.runner.acc.len() != 1 {
            return Err(VMError::from("Wrong acc size"));
        }

        if driver.runner.acc.get(0).unwrap() != &2 {
            return Err(VMError::from("Acc doesn't have the variable"));
        }

        let gvs_variable1_left = driver.gvs.storage[0].clone().unwrap();
        let gvs_variable1_right = driver.gvs.storage[1].clone().unwrap();
        let gvs_variable1_result = driver.gvs.storage[2].clone().unwrap();

        if gvs_variable1_left.owners != 1 {
            return Err(VMError::from("Left varaible has wrong amount of owners"));
        }

        if gvs_variable1_right.owners != 1 {
            return Err(VMError::from("Right varaible has wrong amount of owners"));
        }

        if gvs_variable1_result != result {
            return Err(VMError::from(format!(
                "Wrong result {:?}",
                gvs_variable1_result
            )));
        }

        Ok(())
    }
}
