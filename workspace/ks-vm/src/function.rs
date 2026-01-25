use crate::constants::Instructions;

#[derive(Debug, Clone)]
pub struct Function {
    instructions: Instructions,
    args: Vec<String>,
}

impl Function {
    pub fn method(instructions: Instructions) -> Function {
        Function {
            instructions,
            args: Vec::new(),
        }
    }

    pub fn new(instructions: Instructions, args: Vec<String>) -> Function {
        Function { instructions, args }
    }

    pub fn get_args(&self) -> &[String] {
        &self.args
    }

    pub fn get_instructions(&self) -> &Instructions {
        &self.instructions
    }
}
