use crate::parser::statement::Statement;

use super::instructions::Instruction;

use super::function::Function;
use std::collections::HashMap;

pub struct CompilerNew {
    statements: Vec<Statement>,
    functions: HashMap<String, Function>,
    instuctions: Vec<Instruction>,
}

impl CompilerNew {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self {
            statements,
            functions: HashMap::new(),
            instuctions: Vec::new(),
        }
    }

    pub fn compile(&self, state) -> Vec<Instruction> {
        todo!()
    }
}
