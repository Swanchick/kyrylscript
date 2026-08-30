use std::collections::HashMap;

use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;
use ks_vm::function::Function;

use crate::compiler::compiler::Compiler;
use crate::compiler_new::compiler::CompilerNew;
use crate::compiler_new::program::Program;
use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::parser::statement::Statement;

pub struct KyrylScript {
    parser: Parser,
    compiler: CompilerNew,
}

impl KyrylScript {
    pub fn new() -> KyrylScript {
        KyrylScript {
            parser: Parser::new(),
            compiler: CompilerNew::new(),
        }
    }

    pub fn parser_mut(&mut self) -> &mut Parser {
        &mut self.parser
    }

    pub fn compiler_mut(&mut self) -> &mut CompilerNew {
        &mut self.compiler
    }

    pub fn take_compiler(self) -> CompilerNew {
        self.compiler
    }

    pub fn statements(&mut self, path: &str) -> KsResult<Vec<Statement>> {
        let mut lexer = Lexer::load(path)?;
        lexer.lexer()?;

        let tokens = lexer.get_tokens().to_vec();
        let token_pos = lexer.get_token_pos().to_vec();

        self.parser.set_tokens(tokens, token_pos);
        self.parser.start()
    }

    pub fn compile_from_file(&mut self, path: &str) -> KsResult<HashMap<String, Function>> {
        let mut lexer = Lexer::load(path)?;
        lexer.lexer()?;

        let tokens = lexer.get_tokens().to_vec();
        let token_pos = lexer.get_token_pos().to_vec();

        self.parser.set_tokens(tokens, token_pos);
        let block = self.parser.start();
        if let Err(e) = block {
            e.display();

            return Err(KsError::runtime(&format!(
                "KyrylScript Parser Layer: \n{}",
                e.message(),
            )));
        }

        let block = block?;

        let mut compiler = Compiler::new();
        compiler.start_compile(&block);
        compiler.display();

        Ok(compiler.to_functions())
    }

    pub fn compile_from_file_new(mut self, path: &str) -> KsResult<Program> {
        let mut lexer = Lexer::load(path)?;
        lexer.lexer()?;

        let tokens = lexer.get_tokens().to_vec();
        let token_pos = lexer.get_token_pos().to_vec();

        self.parser.set_tokens(tokens, token_pos);
        let block = self.parser.start();
        if let Err(e) = block {
            e.display();

            return Err(KsError::runtime(&format!(
                "KyrylScript Parser Layer: \n{}",
                e.message(),
            )));
        }

        let statements = block?;
        let result = self.compiler.compile(statements);
        if let Err(e) = result {
            e.display();
            return Err(KsError::runtime(&format!(
                "KyrylScript Compiler Layer: \n{}",
                e.message(),
            )));
        }

        let program = self.compiler.program();
        Ok(program)
    }
}
