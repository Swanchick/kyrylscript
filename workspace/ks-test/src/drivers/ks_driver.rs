use std::collections::HashMap;

use ks_core::compiler::compiler::Compiler;
use ks_core::compiler_new::compiler::CompilerNew;
use ks_core::compiler_new::program::Program;
use ks_core::lexer::lexer::Lexer;
use ks_core::parser::parser::Parser;
use ks_core::parser::statement::Statement;
use ks_global::utils::ks_result::KsResult;
use ks_std::ks_register_std;
use ks_vm::function::Function;
use ks_vm::variable::Variable;
use ks_vm::virtual_machine::VirtualMachine;

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

    pub fn compiler(&self) -> KsResult<HashMap<String, Function>> {
        let statements = self.parser()?;
        let mut compiler = Compiler::new();
        compiler.start_compile(&statements);

        Ok(compiler.to_functions())
    }

    pub fn compiler_new(&self) -> KsResult<Program> {
        let statements = self.parser()?;
        let mut compiler = CompilerNew::new();
        compiler.compile(statements)?;
        compiler.display();

        Ok(compiler.program())
    }

    pub fn run(&self) -> KsResult<()> {
        let comiler_output = self.compiler()?;
        let mut vm = VirtualMachine::from(comiler_output);

        vm.initialize()?;

        Ok(())
    }

    pub fn call_null(&self, function_name: &str) -> KsResult<()> {
        let comiler_output = self.compiler()?;
        let mut vm = VirtualMachine::from(comiler_output);
        vm.initialize()?;
        vm.call_null(function_name)?;
        Ok(())
    }

    pub fn call(&self, function_name: &str) -> KsResult<Variable> {
        let comiler_output = self.compiler()?;
        let mut vm = VirtualMachine::from(comiler_output);
        vm.initialize()?;

        let result = vm.call(function_name)?;

        Ok(result)
    }
}
