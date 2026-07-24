use std::{env::args, println};

use ks_core::kyryl_script::KyrylScript;
use ks_global::utils::ks_result::KsResult;
use ks_std::ks_register_std;

fn main() -> KsResult<()> {
    let args: Vec<String> = args().collect();
    let path = args.get(1);

    if let Some(path) = path {
        let mut kyryl_script = KyrylScript::new();
        ks_register_std(&mut kyryl_script);

        let program = kyryl_script.compile_from_file_new(path)?;

        println!("{:X?}", program);

        // let mut vm = VirtualMachine::from(compilation);
        // vm.initialize()?;
    }

    Ok(())
}
