use std::{env::args, println};

use ks_core::kyryl_script::KyrylScript;
use ks_global::utils::{ks_error::KsError, ks_result::KsResult};
use ks_std::ks_register_std;
use ks_vm_new::{Program, VM};

fn main() -> KsResult<()> {
    let args: Vec<String> = args().collect();
    let path = args.get(1);

    if let Some(path) = path {
        let mut kyryl_script = KyrylScript::new();
        ks_register_std(&mut kyryl_script);

        let program = kyryl_script.compile_from_file_new(path)?;
        let program =
            Program::deserialize(program).or_else(|e| Err(KsError::runtime(&e.message)))?;
        println!("{:X?}", program);
        let mut vm = VM::from(program);
        vm.init();
        while !vm.is_empty() {
            vm.step().or_else(|e| Err(KsError::runtime(&e.message)))?;
        }
    }

    Ok(())
}
