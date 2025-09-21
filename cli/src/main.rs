use core::kyryl_script::KyrylScript;
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

        let kyryl_script = KyrylScript::new();
        let _ = kyryl_script.run_from_file(path);
    }
}
