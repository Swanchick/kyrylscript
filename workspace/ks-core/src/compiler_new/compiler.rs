use std::collections::HashMap;

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
use super::types::{Pointer, VariableId};

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
        self.scope_enter();
        self.compile_statements(statements)?;
        self.scope_exit();

        Ok(())
    }

    fn compile_statements(&mut self, statements: Vec<Statement>) -> KsResult<()> {
        for statement in statements {
            self.compile_statement(statement)?;
        }

        Ok(())
    }

    fn current_pc(&self) -> Pointer {
        let saved_insctructions = self.instructions.len();
        let scope_instructions: usize = self.scopes.iter().map(|scope| scope.len()).sum();

        saved_insctructions + scope_instructions
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

    fn scope_append(&mut self, mut scope: Vec<Instruction>) -> KsResult<()> {
        let last_scope = self.scope_last_mut()?;
        last_scope.append(&mut scope);

        Ok(())
    }

    fn scope_exit(&mut self) {
        if let Some(mut insctructions) = self.scopes.pop() {
            self.instructions.append(&mut insctructions);
        }
    }

    fn wrap_scope_into_enter(&self, mut scope: Vec<Instruction>) -> Vec<Instruction> {
        let mut final_scope = Vec::<Instruction>::new();
        final_scope.push(Instruction::Enter);
        final_scope.append(&mut scope);
        final_scope.push(Instruction::Exit);
        final_scope
    }

    fn insert(&mut self, instruction: Instruction) -> KsResult<()> {
        let last_scope = self.scope_last_mut()?;
        last_scope.push(instruction);
        Ok(())
    }

    fn insert_store(&mut self, variable_id: VariableId, public: bool) -> KsResult<()> {
        if public {
            self.insert(Instruction::PubStore(variable_id))?;
        } else {
            self.insert(Instruction::Store(variable_id))?;
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

        self.insert_store(variable_id, public)?;

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
        let pointer = self.current_pc() + 1;
        self.environment.define_function(&name, pointer);
        let variable_id = self.environment.define_variable(name)?;

        let mut final_scope = Vec::<Instruction>::new();
        for parameter in parameters {
            let parameter_id = self.environment.define_variable(parameter.name)?;
            final_scope.push(Instruction::Store(parameter_id));
        }

        final_scope.reverse();

        self.scope_enter();
        self.compile_statements(body)?;

        let mut body_scope = self.scope_pop()?;
        final_scope.append(&mut body_scope);

        if !matches!(final_scope.last(), Some(Instruction::Return)) {
            final_scope.push(Instruction::Return);
        }

        self.insert(Instruction::Jump(final_scope.len() as i32))?;
        self.scope_append(final_scope)?;

        self.insert_constant(Constant::Function(pointer))?;
        self.insert_store(variable_id, public)?;

        Ok(())
    }

    fn return_statement(&mut self, expression: Option<Expression>) -> KsResult<()> {
        if let Some(expression) = expression {
            self.compile_expression(expression)?;
        }

        self.insert(Instruction::Return)?;

        Ok(())
    }

    fn assign_identifier_name(&mut self, name: String, is_first: bool) -> KsResult<()> {
        let variable_id = self.environment.variable_id(&name)?;

        if is_first {
            self.insert(Instruction::AssignVar(variable_id))?;
        } else {
            self.insert(Instruction::AssignModule(variable_id))?;
        }

        Ok(())
    }

    fn assign_identifier(&mut self, identifier: Vec<IdentifierTail>) -> KsResult<()> {
        let mut index = 0;
        for segment in identifier {
            match segment {
                IdentifierTail::Name(name) => self.assign_identifier_name(name, index == 0),
                IdentifierTail::Call(_) => {
                    Err(KsError::parse("Cannot call in assignment identifier"))
                }
                _ => todo!(),
            }?;

            index += 1;
        }

        Ok(())
    }

    fn assignment(
        &mut self,
        identifier: Vec<IdentifierTail>,
        expression: Expression,
    ) -> KsResult<()> {
        self.assign_identifier(identifier)?;
        self.compile_expression(expression)?;
        self.insert(Instruction::Assign)?;
        Ok(())
    }

    fn arithmetic_assignment(
        &mut self,
        identifier: Vec<IdentifierTail>,
        expression: Expression,
        operator: Operator,
    ) -> KsResult<()> {
        let assign_identifier = identifier.clone();
        self.assign_identifier(assign_identifier)?;
        self.identifier(identifier)?;
        self.compile_expression(expression)?;
        self.insert_operator(operator)?;
        self.insert(Instruction::Assign)?;

        Ok(())
    }

    fn if_statement(
        &mut self,
        expression: Expression,
        body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    ) -> KsResult<()> {
        self.compile_expression(expression)?;

        self.scope_enter();
        self.compile_statements(body)?;
        let body_scope = self.scope_pop()?;
        let mut body_scope = self.wrap_scope_into_enter(body_scope);

        let else_body_scope = if let Some(else_body) = else_body {
            self.scope_enter();
            self.compile_statements(else_body)?;
            let else_body_scope = self.scope_pop()?;
            let else_body_scope = self.wrap_scope_into_enter(else_body_scope);
            body_scope.push(Instruction::Jump(else_body_scope.len() as i32));

            else_body_scope
        } else {
            Vec::new()
        };

        self.insert(Instruction::JumpIfFalse(body_scope.len() as i32))?;
        self.scope_append(body_scope)?;
        self.scope_append(else_body_scope)?;

        Ok(())
    }

    fn while_statement(&mut self, expression: Expression, body: Vec<Statement>) -> KsResult<()> {
        self.scope_enter();
        self.compile_expression(expression)?;
        let expression_scope = self.scope_pop()?;

        self.scope_enter();
        self.compile_statements(body)?;
        let body_scope = self.scope_pop()?;
        let body_scope = self.wrap_scope_into_enter(body_scope);

        let expression_len = expression_scope.len() as i32;
        let body_len = body_scope.len() as i32;

        self.insert(Instruction::Jump(body_len))?;

        self.scope_append(body_scope)?;
        self.scope_append(expression_scope)?;
        self.insert(Instruction::JumpIfTrue(-body_len - expression_len))?;

        Ok(())
    }

    fn for_statement(
        &mut self,
        name: String,
        list: Expression,
        body: Vec<Statement>,
    ) -> KsResult<()> {
        let iter_list = self
            .environment
            .define_variable(format!("__iter_list_{}", name))?;
        let iter = self
            .environment
            .define_variable(format!("__iter_{}", name))?;
        let iter_variable_id = self.environment.define_variable(name)?;

        self.insert(Instruction::Enter)?;
        self.compile_expression(list)?;
        self.insert(Instruction::Store(iter_list))?;
        self.insert_constant(Constant::Integer(0))?;
        self.insert(Instruction::Store(iter))?;
        self.insert_constant(Constant::Null)?;
        self.insert(Instruction::Store(iter_variable_id))?;

        self.scope_enter();
        self.insert(Instruction::AssignVar(iter_variable_id))?;
        self.insert(Instruction::LoadVar(iter))?;
        self.insert(Instruction::LoadVar(iter_list))?;
        self.insert(Instruction::LoadFromList)?;
        self.insert(Instruction::Assign)?;

        self.compile_statements(body)?;
        let body_scope = self.scope_pop()?;
        let body_scope = self.wrap_scope_into_enter(body_scope);
        let body_len = body_scope.len() as i32;
        self.scope_append(body_scope)?;

        self.scope_enter();
        self.insert(Instruction::LoadVar(iter))?;
        self.insert(Instruction::Increment)?;
        self.insert(Instruction::LoadVar(iter_list))?;
        self.insert(Instruction::ListLen)?;
        self.insert(Instruction::GreaterEq)?;
        let after_scope = self.scope_pop()?;
        let after_len = after_scope.len() as i32;
        self.scope_append(after_scope)?;

        self.insert(Instruction::JumpIfFalse(-body_len - after_len))?;
        self.insert(Instruction::Exit)?;

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
            Statement::ReturnStatement { value } => self.return_statement(value),
            Statement::Assignment { segments, value } => self.assignment(segments, value),
            Statement::AddValue { segments, value } => {
                self.arithmetic_assignment(segments, value, Operator::Plus)
            }
            Statement::RemoveValue { segments, value } => {
                self.arithmetic_assignment(segments, value, Operator::Minus)
            }
            Statement::IfStatement {
                condition,
                body,
                else_body,
            } => self.if_statement(condition, body, else_body),
            Statement::WhileStatement { condition, body } => self.while_statement(condition, body),
            Statement::ForLoopStatement { name, list, body } => {
                self.for_statement(name, list, body)
            }
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

    fn identifier_call(&mut self, expressions: Vec<Expression>) -> KsResult<()> {
        let arguments = expressions.len();

        for expression in expressions {
            self.compile_expression(expression)?;
        }

        self.insert(Instruction::Call(arguments))?;

        Ok(())
    }

    fn identifier(&mut self, identifier: Vec<IdentifierTail>) -> KsResult<()> {
        let mut index = 0;

        for segment in identifier {
            match segment {
                IdentifierTail::Name(name) => self.identifier_name(name, index == 0),
                IdentifierTail::Call(expressions) => self.identifier_call(expressions),
                _ => todo!(),
            }?;

            index += 1;
        }

        Ok(())
    }

    fn insert_operator(&mut self, operator: Operator) -> KsResult<()> {
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

    fn collection(
        &mut self,
        collection: Instruction,
        expressions: Vec<Expression>,
    ) -> KsResult<()> {
        for expression in expressions {
            self.compile_expression(expression)?;
        }

        self.insert(collection)?;
        Ok(())
    }

    fn save_if_module(&mut self, name: String) -> KsResult<()> {
        // let last_module = self.last_module.take();
        // if let Some(module) = last_module {
        //     self.environment.define_module(name)?;
        // }

        Ok(())
    }

    fn module_literal(&mut self, module: HashMap<String, Expression>) -> KsResult<()> {
        self.environment.create_module()?;

        for (name, expression) in module {
            let last_temp_modules_len = self.environment.temporary_modules_len();

            self.compile_expression(expression)?;

            let is_field = self.environment.temporary_modules_len() == last_temp_modules_len;

            if is_field {
                self.environment.insert_field(name)?;
            } else {
                self.environment.insert_module(name)?;
            }
        }

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
        self.insert_operator(operator)?;
        Ok(())
    }

    fn unary_operator(&mut self, expression: Expression, operator: Operator) -> KsResult<()> {
        self.compile_expression(expression)?;
        self.insert_operator(operator)?;

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
            Expression::ListLiteral(expressions) => {
                self.collection(Instruction::LoadList(expressions.len()), expressions)
            }
            Expression::TupleLiteral(expressions) => {
                self.collection(Instruction::LoadTuple(expressions.len()), expressions)
            }
            Expression::Module(module) => self.module_literal(module),
            Expression::BinaryOp {
                left,
                operator,
                right,
            } => self.binary_operation(*left, operator, *right),
            Expression::UnaryOp {
                expression,
                operator,
            } => self.unary_operator(*expression, operator),
            Expression::FrontUnaryOp {
                expression,
                operator,
            } => self.unary_operator(*expression, operator),
            _ => todo!(),
        }?;

        Ok(())
    }
}
