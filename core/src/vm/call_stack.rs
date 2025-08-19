use crate::global::constants::Instructions;

pub struct CallStack {
    name: String,
    instructions: Instructions,
    scopes: usize
}

impl CallStack {
    pub fn new(name: &str, instructions: Instructions) -> CallStack {
        CallStack { 
            name: name.to_string(), 
            instructions, 
            scopes: 0 
        }
    }
}