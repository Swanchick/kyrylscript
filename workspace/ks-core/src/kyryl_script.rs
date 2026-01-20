use std::collections::HashMap;

use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;
use ks_vm::function::Function;

use crate::compiler::compiler::Compiler;
use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;

pub struct KyrylScript {
    parser: Parser,
}

impl KyrylScript {
    pub fn new() -> KyrylScript {
        KyrylScript {
            parser: Parser::new(),
        }
    }

    pub fn parser_mut(&mut self) -> &mut Parser {
        &mut self.parser
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
}
