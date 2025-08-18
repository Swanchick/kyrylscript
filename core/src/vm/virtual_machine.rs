use super::environment::Environment;

pub struct VirtualMachine {
    environment: Environment
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine { 
            environment: Environment::new() 
        }
    }
}
