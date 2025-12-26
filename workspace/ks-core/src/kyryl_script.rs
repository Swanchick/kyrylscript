use std::collections::HashMap;
use std::io;

use ks_vm::function::Function;

use crate::compiler::compiler::Compiler;
use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;

pub struct KyrylScript {}

impl KyrylScript {
    pub fn compile_from_file(&self, path: &str) -> io::Result<HashMap<String, Function>> {
        let mut lexer = Lexer::load(path)?;
        lexer.lexer()?;

        let tokens = lexer.get_tokens().clone();
        let token_pos = lexer.get_token_pos().clone();

        let mut parser = Parser::new(tokens, token_pos);
        let block = parser.start();

        if let Err(e) = block {
            println!("{}", e);

            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("KyrylScript Parser Layer: \n{}", e.to_string()),
            ));
        }

        let block = block?;

        let mut compiler = Compiler::new();
        compiler.start_compile(&block);
        compiler.display();

        Ok(compiler.to_functions())
    }
}
