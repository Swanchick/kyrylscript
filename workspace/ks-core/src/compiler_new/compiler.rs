use std::collections::HashMap;

use ks_global::utils::ks_result::KsResult;

use crate::parser::expression::Expression;
use crate::parser::identifier_tail::IdentifierTail;
use crate::parser::statement::Statement;

use super::constant::Constant;
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

    pub fn compile(&mut self, statements: Vec<Statement>) -> KsResult<()> {
        for statement in statements {
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
        let variable_id = self.environment.define_variable(name)?;

        if let Some(expression) = expression {
            self.compile_expression(expression)
        } else {
            self.insert_constant(Constant::Null)
        }?;

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

    fn insert_constant(&mut self, constant: Constant) -> KsResult<()> {
        self.instuctions.push(Instruction::LoadConst(constant));
        Ok(())
    }

    fn identifier_name(&mut self, name: String) -> KsResult<()> {
        let variable_id = self.environment.variable_id(&name)?;
        self.instuctions.push(Instruction::LoadVar(variable_id));

        Ok(())
    }

    fn identifier(&mut self, identifier: Vec<IdentifierTail>) -> KsResult<()> {
        for segment in identifier {
            match segment {
                IdentifierTail::Name(name) => self.identifier_name(name),
                _ => todo!(),
            }?;
        }

        Ok(())
    }

    fn compile_expression(&mut self, expression: Expression) -> KsResult<()> {
        match expression {
            Expression::NullLiteral => self.insert_constant(Constant::Null),
            Expression::BooleanLiteral(boolean) => self.insert_constant(Constant::Boolean(boolean)),
            Expression::IntegerLiteral(integer) => self.insert_constant(Constant::Integer(integer)),
            Expression::FloatLiteral(float) => self.insert_constant(Constant::Float(float)),
            Expression::StringLiteral(string) => self.insert_constant(Constant::String(string)),
            Expression::Identifier(identifier) => self.identifier(identifier),
            _ => todo!(),
        }?;

        Ok(())
    }
}
