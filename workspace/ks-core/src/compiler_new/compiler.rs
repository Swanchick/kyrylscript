use std::collections::HashMap;

use ks_global::utils::ks_result::KsResult;

use crate::compiler_new::constant::Constant;
use crate::parser::expression::Expression;
use crate::parser::statement::Statement;

use super::environment::Environment;
use super::instructions::Instruction;
use super::program::Program;
use super::types::FunctionPointer;

pub struct CompilerNew {
    functions: HashMap<String, FunctionPointer>,
    instuctions: Vec<Instruction>,
    environment: Environment,
}

impl CompilerNew {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            instuctions: Vec::new(),
            environment: Environment::new(),
        }
    }

    pub fn program(self) -> Program {
        Program::new(self.instuctions, self.functions)
    }

    pub fn compile(&mut self, mut statements: Vec<Statement>) -> KsResult<()> {
        while let Some(statement) = statements.pop() {
            self.compile_statement(statement)?;
        }

        Ok(())
    }

    fn variable_declaration(
        &mut self,
        name: String,
        public: bool,
        expression: Option<Expression>,
    ) -> KsResult<()> {
        let variable_id = self.environment.define_variable(&name)?;

        if let Some(expression) = expression {
            self.compile_expression(expression)?;
        }

        // REFACTOR needed in the parser, need to make a separate statement for the PubVariableDeclaration
        if public {
            self.instuctions.push(Instruction::PubStore(variable_id));
        } else {
            self.instuctions.push(Instruction::Store(variable_id));
        }

        Ok(())
    }

    fn compile_statement(&mut self, statement: Statement) -> KsResult<()> {
        match statement {
            Statement::VariableDeclaration {
                name,
                public,
                data_type: _,
                value,
            } => self.variable_declaration(name, public, value),
            _ => todo!(),
        }?;

        Ok(())
    }

    fn integer(&mut self, integer: i32) -> KsResult<()> {
        self.instuctions
            .push(Instruction::LoadConst(Constant::Integer(integer)));
        Ok(())
    }

    fn compile_expression(&mut self, expression: Expression) -> KsResult<()> {
        match expression {
            Expression::IntegerLiteral(integer) => self.integer(integer),
            _ => todo!(),
        }?;

        Ok(())
    }
}
