use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;

use crate::parser::expression::Expression;
use crate::parser::identifier_tail::IdentifierTail;
use crate::parser::operator::Operator;
use crate::parser::parameter::Parameter;
use crate::parser::statement::Statement;

use super::constant::Constant;
use super::environment::Environment;
use super::instructions::Instruction;
use super::program::Program;
use super::types::Pointer;

pub struct CompilerNew {
    scopes: Vec<Vec<Instruction>>,
    instructions: Vec<Instruction>,
    environment: Environment,
}

impl CompilerNew {
    pub fn new() -> Self {
        Self {
            scopes: Vec::new(),
            instructions: Vec::new(),
            environment: Environment::new(),
        }
    }

    pub fn program(self) -> Program {
        Program::new(self.instructions, self.environment.functions())
    }

    pub fn compile(&mut self, statements: Vec<Statement>) -> KsResult<()> {
        for statement in statements {
            self.scope_enter();
            self.compile_statement(statement)?;
            self.scope_exit();
        }

        Ok(())
    }

    fn current_pc(&mut self) -> Pointer {
        self.instructions.len()
    }

    fn scope_enter(&mut self) {
        self.scopes.push(Vec::new());
    }

    fn scope_pop(&mut self) -> KsResult<Vec<Instruction>> {
        if let Some(scope) = self.scopes.pop() {
            Ok(scope)
        } else {
            Err(KsError::parse("Cannot get ownership of last scope"))
        }
    }

    fn scope_last_mut(&mut self) -> KsResult<&mut Vec<Instruction>> {
        if let Some(scope) = self.scopes.last_mut() {
            Ok(scope)
        } else {
            Err(KsError::parse("Cannot get last mutable scope"))
        }
    }

    fn scope_exit(&mut self) {
        if let Some(mut insctructions) = self.scopes.pop() {
            self.instructions.append(&mut insctructions);
        }
    }

    fn insert(&mut self, instruction: Instruction) -> KsResult<()> {
        let last_scope = self.scope_last_mut()?;
        last_scope.push(instruction);
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
            self.insert(Instruction::PubStore(variable_id))?;
        } else {
            self.insert(Instruction::Store(variable_id))?;
        }

        Ok(())
    }

    fn expression_statement(&mut self, expression: Expression) -> KsResult<()> {
        self.compile_expression(expression)?;
        self.insert(Instruction::End)?;

        Ok(())
    }

    fn function_declaration(
        &mut self,
        name: String,
        public: bool,
        parameters: Vec<Parameter>,
        body: Vec<Statement>,
    ) -> KsResult<()> {
        let current_pc = self.current_pc();

        // self.environment.define_variable(name)?;

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
            Statement::Expression { value } => self.expression_statement(value),
            Statement::Function {
                name,
                public,
                return_type: _,
                parameters,
                body,
            } => self.function_declaration(name, public, parameters, body),
            _ => todo!(),
        }?;

        Ok(())
    }

    fn insert_constant(&mut self, constant: Constant) -> KsResult<()> {
        self.insert(Instruction::LoadConst(constant))?;
        Ok(())
    }

    fn identifier_name(&mut self, name: String, is_first: bool) -> KsResult<()> {
        let variable_id = self.environment.variable_id(&name)?;

        let instruction = if is_first {
            Instruction::LoadVar(variable_id)
        } else {
            Instruction::LoadFromModule(variable_id)
        };

        self.insert(instruction)?;

        Ok(())
    }

    fn identifier(&mut self, identifier: Vec<IdentifierTail>) -> KsResult<()> {
        let mut index = 0;

        for segment in identifier {
            match segment {
                IdentifierTail::Name(name) => self.identifier_name(name, index == 0),
                _ => todo!(),
            }?;

            index += 1;
        }

        Ok(())
    }

    fn operator(&mut self, operator: Operator) -> KsResult<()> {
        let instruction = match operator {
            Operator::Plus => Instruction::Add,
            Operator::Minus => Instruction::Minus,
            Operator::Multiply => Instruction::Mul,
            Operator::Divide => Instruction::Div,
            Operator::EqualEqual => Instruction::Eq,
            Operator::GreaterEqual => Instruction::GreaterEq,
            Operator::Greater => Instruction::Greater,
            Operator::LessEqual => Instruction::LessEq,
            Operator::Less => Instruction::Less,
            Operator::NotEqual => Instruction::NotEq,
            Operator::And => Instruction::And,
            Operator::Or => Instruction::Or,
            Operator::Not => Instruction::Not,
            Operator::PlusPlus => Instruction::Increment,
            Operator::MinusMinus => Instruction::Decrement,
            Operator::Clone => Instruction::Clone,
            Operator::Power => Instruction::Power,
        };

        self.insert(instruction)?;

        Ok(())
    }

    fn binary_operation(
        &mut self,
        left: Expression,
        operator: Operator,
        right: Expression,
    ) -> KsResult<()> {
        self.compile_expression(left)?;
        self.compile_expression(right)?;
        self.operator(operator)?;
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
            Expression::BinaryOp {
                left,
                operator,
                right,
            } => self.binary_operation(*left, operator, *right),
            _ => todo!(),
        }?;

        Ok(())
    }
}
