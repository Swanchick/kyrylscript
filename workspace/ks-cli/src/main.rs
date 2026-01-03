use std::env::args;

use ks_core::kyryl_script::KyrylScript;
use ks_global::utils::ks_result::KsResult;
use ks_std::ks_register_std;
use ks_vm::virtual_machine::VirtualMachine;

fn main() -> KsResult<()> {
    let args: Vec<String> = args().collect();
    let path = args.get(1);

    if let Some(path) = path {
        let mut kyryl_script = KyrylScript::new();
        let parser = kyryl_script.parser_mut();

        ks_register_std(parser);

        let compilation = kyryl_script.compile_from_file(path)?;

        let mut vm = VirtualMachine::from(compilation);
        vm.initialize()?;
    }

    Ok(())
}
