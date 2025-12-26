use std::env::args;
use std::io;

use ks_core::kyryl_script::KyrylScript;
use ks_std::ks_register_std;
use ks_vm::virtual_machine::VirtualMachine;

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();
    let path = args.get(1);

    if let Some(path) = path {
        let kyryl_script = KyrylScript {};
        let compilation = kyryl_script.compile_from_file(path)?;

        ks_register_std();

        let mut vm = VirtualMachine::from(compilation);
        let result = vm.initialize();
        if let Err(err) = result {
            err.display();
        }
    }

    Ok(())
}
