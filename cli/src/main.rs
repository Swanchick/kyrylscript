use std::env::args;

use core::compiler::compiler::Compiler;
use core::lexer::lexer::Lexer;
use core::parser::parser::Parser;
use core::vm::virtual_machine::VirtualMachine;

use ks_std::ks_register_std;
use core::global::ks_path::KsPath;

fn main() {
    let args: Vec<String> = args().collect();
    let path = args.get(1);

    if let Some(path) = path {
        ks_register_std();

        let mut lexer = Lexer::load(&path).unwrap();
        lexer.lexer().unwrap();

        let mut parser = Parser::new(
            lexer.get_tokens().clone(), 
            lexer.get_token_pos().clone(), 
            KsPath::new(), 
            KsPath::new()
        );

        let statements = parser.start().unwrap();

        let mut compiler = Compiler::new();
        compiler.start_compile(&statements);
        compiler.display();

        println!("============================");

        let mut virtual_machine = VirtualMachine::from(compiler.functions());
        let result = virtual_machine.initialize();
        if let Err(result) = result {
            result.display();
        }
    }
}
