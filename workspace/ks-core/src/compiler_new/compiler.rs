use super::instructions::Instruction;
use super::types::FunctionPointer;
use crate::compiler_new::program::Program;
use crate::parser::expression::Expression;
use crate::parser::statement::Statement;
use std::collections::HashMap;

pub struct CompilerNew {
    statements: Vec<Statement>,
    functions: HashMap<String, FunctionPointer>,
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

    pub fn program(self) -> Program {
        Program::new(self.instuctions, self.functions)
    }

    pub fn compile(&mut self) {
        while let Some(statement) = self.statements.pop() {
            self.compile_statement(statement);
        }
    }

    fn variable_declaration(&mut self) {}

    fn compile_statement(&mut self, statement: Statement) {
        match statement {
            Statement::VariableDeclaration {
                name,
                public,
                data_type,
                value,
            } => self.variable_declaration(),
            _ => todo!(),
        }
    }

    fn compile_expression(&mut self, expression: Expression) {}
}
