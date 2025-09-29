use std::io;

use crate::compiler::compiler::Compiler;
use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;

use crate::vm::virtual_machine::VirtualMachine;

pub struct KyrylScript {
}

impl KyrylScript {
    pub fn new() -> KyrylScript {
        KyrylScript { }
    }

    pub fn run_from_file(&self, path: &str) -> io::Result<()> {
        let mut lexer = Lexer::load(path)?;
        lexer.lexer()?;

        let tokens = lexer.get_tokens().clone();
        let token_pos = lexer.get_token_pos().clone();

        let mut parser = Parser::new(tokens, token_pos);
        let block = parser.start();

        if let Err(e) = block {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("KyrylScript Parser Layer: \n{}", e.to_string())
            ));
        }

        let block = block?;

        let mut compiler = Compiler::new();
        compiler.start_compile(&block);

        let mut vm = VirtualMachine::from(compiler.functions());
        let result = vm.initialize();

        if let Err(e) = result {
            e.display();
        }

        Ok(())
    }
}
