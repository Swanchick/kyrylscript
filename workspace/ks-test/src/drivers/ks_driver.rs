use std::collections::HashMap;

use ks_core::compiler::compiler::Compiler;
use ks_core::compiler_new::compiler::CompilerNew;
use ks_core::lexer::lexer::Lexer;
use ks_core::parser::parser::Parser;
use ks_core::parser::statement::Statement;
use ks_global::utils::ks_result::KsResult;
use ks_std::ks_register_std;
use ks_vm::function::Function;
use ks_vm_new::{GVS, Instruction, Runner};

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

    pub fn compiler(&self) -> KsResult<HashMap<String, Function>> {
        let statements = self.parser()?;
        let mut compiler = Compiler::new();
        compiler.start_compile(&statements);

        Ok(compiler.to_functions())
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
}
