use std::collections::HashMap;

use ks_core::compiler::compiler::Compiler;
use ks_core::lexer::lexer::Lexer;
use ks_core::parser::parser::Parser;
use ks_core::parser::statement::Statement;
use ks_global::utils::ks_result::KsResult;
use ks_vm::function::Function;
use ks_vm::virtual_machine::VirtualMachine;

pub struct KsDriver {
    path: String,
}

impl KsDriver {
    pub fn new(path: &str) -> KsDriver {
        KsDriver {
            path: path.to_string(),
        }
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
        let statements = parser.start()?;

        Ok(statements)
    }

    pub fn compiler(&self) -> KsResult<HashMap<String, Function>> {
        let statements = self.parser()?;
        let mut compiler = Compiler::new();
        compiler.start_compile(&statements);

        Ok(compiler.to_functions())
    }
}
