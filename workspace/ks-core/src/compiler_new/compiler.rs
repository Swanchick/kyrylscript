use std::collections::{BTreeMap, HashMap};

use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;
use ks_vm_new::{Constant, Instruction, Program};

use crate::parser::expression::Expression;
use crate::parser::identifier_tail::IdentifierTail;
use crate::parser::operator::Operator;
use crate::parser::parameter::Parameter;
use crate::parser::statement::Statement;

use super::collection::Collection;
use super::environment::Environment;
use super::slot::Slot;
use super::types::{CollectionId, Pointer, VariableId};

pub struct CompilerNew {
    scopes: Vec<Vec<Instruction>>,
    instructions: Vec<Instruction>,
    environment: Environment,
    function_depth: usize,
}

impl CompilerNew {
    pub fn new() -> Self {
        Self {
            scopes: Vec::new(),
            instructions: Vec::new(),
            environment: Environment::new(),
            function_depth: 0,
        }
    }

    pub fn program(self) -> Program {
        Program::new(self.instructions, self.environment.functions())
    }

    pub fn compile(&mut self, statements: Vec<Statement>) -> KsResult<()> {
        self.scope_enter();
        self.environment.enter()?;

        self.compile_statements(statements)?;

        self.environment.exit()?;
        self.scope_exit();

        Ok(())
    }

    fn compile_statements(&mut self, statements: Vec<Statement>) -> KsResult<()> {
        for statement in statements {
            self.compile_statement(statement)?;
        }

        Ok(())
    }

    fn free(&mut self) -> KsResult<()> {
        let variable_scope = self.environment.exit()?;
        if variable_scope.len() != 0 {
            self.insert(Instruction::Free(variable_scope.len()))?;
        }

        Ok(())
    }

    fn compile_statements_free(&mut self, statements: Vec<Statement>) -> KsResult<()> {
        self.environment.enter()?;

        self.compile_statements(statements)?;

        self.free()?;

        Ok(())
    }

    fn current_pc(&self) -> Pointer {
        let saved_insctructions = self.instructions.len();
        let in_scopes: usize = self.scopes.iter().map(|scope| scope.len()).sum();

        saved_insctructions + in_scopes
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

    fn scope_last(&self) -> KsResult<&[Instruction]> {
        if let Some(scope) = self.scopes.last() {
            Ok(scope)
        } else {
            Err(KsError::parse("Cannot get last mutable scope"))
        }
    }

    fn scope_last_instruction(&self) -> KsResult<&Instruction> {
        let scope = self.scope_last()?;
        if let Some(instruction) = scope.last() {
            Ok(instruction)
        } else {
            Err(KsError::parse("Cannot get last instruction"))
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

    fn insert(&mut self, instruction: Instruction) -> KsResult<()> {
        let last_scope = self.scope_last_mut()?;
        last_scope.push(instruction);
        Ok(())
    }

    fn insert_store(&mut self, public: bool) -> KsResult<()> {
        if public {
            self.insert(Instruction::PubStore)?;
        } else {
            self.insert(Instruction::Store)?;
        }

        Ok(())
    }

    fn variable_declaration(
        &mut self,
        name: String,
        public: bool,
        expression: Option<Expression>,
    ) -> KsResult<()> {
        if let Some(expression) = expression {
            self.compile_expression(expression)
        } else {
            self.insert_constant(Constant::Null)
        }?;

        self.environment.define_variable(name)?;
        self.insert_store(public)?;

        Ok(())
    }

    fn expression_statement(&mut self, expression: Expression) -> KsResult<()> {
        self.compile_expression(expression)?;
        self.insert(Instruction::ClearAcc)?;

        self.environment.clear_temp_collection();

        Ok(())
    }

    fn function(
        &mut self,
        parameters: Vec<Parameter>,
        body: Vec<Statement>,
        captured: Vec<String>,
    ) -> KsResult<Pointer> {
        self.function_depth += 1;

        self.environment.enter_function()?;

        self.scope_enter();

        for parameter in parameters {
            self.environment.define_variable(parameter.name)?;
            self.insert(Instruction::Store)?;
        }

        for index in 0..captured.len() {
            self.insert(Instruction::LoadCapture(index as u64))?;

            self.environment.define_variable(captured[index].clone())?;
            self.insert(Instruction::Store)?;
        }

        self.scope_enter();

        self.compile_statements(body)?;

        let body_scope = self.scope_pop()?;
        self.scope_append(body_scope)?;

        let variables = self.environment.exit_function()?;

        let last_instruction = self.scope_last_instruction();
        if !matches!(last_instruction, Ok(&Instruction::Return)) {
            if variables != 0 {
                self.insert(Instruction::Free(variables))?;
            }

            self.insert(Instruction::Return)?;
        }

        let final_scope = self.scope_pop()?;

        self.insert(Instruction::Jump(final_scope.len() as i32))?;
        let pointer = self.current_pc() + self.function_depth - 1;

        self.scope_append(final_scope)?;

        self.insert_constant(Constant::Integer(pointer as i64))?;
        let captured_len = captured.len();
        for capture in captured {
            let variable_id = self.environment.variable_id(&capture)?;
            self.insert(Instruction::LoadVar(variable_id))?;
        }
        self.insert(Instruction::LoadFunction(captured_len))?;

        self.function_depth -= 1;

        Ok(pointer)
    }

    fn function_declaration(
        &mut self,
        name: String,
        public: bool,
        parameters: Vec<Parameter>,
        body: Vec<Statement>,
        captured: Vec<String>,
    ) -> KsResult<()> {
        let pointer = self.function(parameters, body, captured)?;

        self.environment.define_function(&name, pointer);
        self.environment.define_variable(name)?;
        self.insert_store(public)?;

        Ok(())
    }

    fn return_statement(&mut self, expression: Option<Expression>) -> KsResult<()> {
        if let Some(expression) = expression {
            self.compile_expression(expression)?;
        }

        let variables = self.environment.current()?;

        if variables != 0 {
            self.insert(Instruction::Free(variables as usize))?;
        }

        self.insert(Instruction::Return)?;

        Ok(())
    }

    fn assignment(
        &mut self,
        identifier: Vec<IdentifierTail>,
        expression: Expression,
    ) -> KsResult<()> {
        self.identifier(identifier, true)?;
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

        self.identifier(assign_identifier, true)?;
        self.identifier(identifier, false)?;

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
        self.compile_statements_free(body)?;
        let mut body_scope = self.scope_pop()?;

        let else_body_scope = if let Some(else_body) = else_body {
            self.scope_enter();
            self.compile_statements_free(else_body)?;
            let else_body_scope = self.scope_pop()?;

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
        self.compile_statements_free(body)?;
        let body_scope = self.scope_pop()?;

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
        self.environment.enter()?;

        let iter_list = self
            .environment
            .define_variable(format!("__iter_list_{}", name))?;
        let iter = self
            .environment
            .define_variable(format!("__iter_{}", name))?;
        let iter_variable_id = self.environment.define_variable(name)?;

        self.compile_expression(list)?;
        self.insert(Instruction::Store)?;
        self.insert_constant(Constant::Integer(0))?;
        self.insert(Instruction::Store)?;
        self.insert_constant(Constant::Null)?;
        self.insert(Instruction::Store)?;

        self.scope_enter();
        self.insert(Instruction::AssignVariable(iter_variable_id))?;
        self.insert(Instruction::LoadVar(iter))?;
        self.insert(Instruction::LoadVar(iter_list))?;
        self.insert(Instruction::LoadFromCollection)?;
        self.insert(Instruction::Assign)?;

        self.environment.enter()?;

        self.compile_statements(body)?;
        self.free()?;

        let body_scope = self.scope_pop()?;
        let body_len = body_scope.len() as i32;
        self.scope_append(body_scope)?;

        self.scope_enter();
        self.insert(Instruction::LoadVar(iter))?;
        self.insert(Instruction::Increment)?;
        self.insert(Instruction::LoadVar(iter_list))?;
        self.insert(Instruction::CollectionLen)?;
        self.insert(Instruction::GreaterEq)?;
        let after_scope = self.scope_pop()?;
        let after_len = after_scope.len() as i32;
        self.scope_append(after_scope)?;

        self.insert(Instruction::JumpIfFalse(-body_len - after_len))?;

        self.free()?;

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
                captured,
            } => self.function_declaration(name, public, parameters, body, captured),
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

    fn insert_collection(&mut self, assign: bool) -> KsResult<()> {
        if assign {
            self.insert(Instruction::AssignCollection)?;
        } else {
            self.insert(Instruction::LoadFromCollection)?;
        }

        Ok(())
    }

    fn identifier_name(
        &mut self,
        name: String,
        last_collection_id: &mut Option<CollectionId>,
        assign: bool,
    ) -> KsResult<()> {
        if let Some(collection_id) = last_collection_id {
            let collection = self.environment.collection(*collection_id)?;
            if let Collection::Module { children, indeces } = collection {
                if let Some(variable_id) = indeces.get(&name) {
                    if let Some(collection_id) = children.get(*variable_id as usize) {
                        *last_collection_id = collection_id.clone();
                    }

                    self.insert_constant(Constant::Integer(*variable_id as i64))?;
                    self.insert_collection(assign)?;
                }
            }
        } else {
            let slot = self.environment.slot(&name)?;
            let variable_id = *slot.variable_id();

            if let Slot::Collection {
                variable_id: _,
                collection_id,
            } = slot
            {
                *last_collection_id = Some(*collection_id);
            }

            if assign {
                self.insert(Instruction::AssignVariable(variable_id))?;
            } else {
                self.insert(Instruction::LoadVar(variable_id))?;
            }
        }

        Ok(())
    }

    fn identifier_call(&mut self, mut expressions: Vec<Expression>, assign: bool) -> KsResult<()> {
        if assign {
            return Err(KsError::parse("Cannot call in assign statement"));
        }
        while let Some(expression) = expressions.pop() {
            self.compile_expression(expression)?;
        }

        self.insert(Instruction::Call)?;

        Ok(())
    }

    fn identifier_index(
        &mut self,
        expression: Expression,
        last_collection_id: &mut Option<CollectionId>,
        assign: bool,
    ) -> KsResult<()> {
        if let Some(collection_id) = last_collection_id {
            let collection = self.environment.collection(*collection_id)?;
            if let Collection::List { child } = collection {
                *last_collection_id = child.clone();
                self.compile_expression(expression)?;
                self.insert_collection(assign)?;
            }

            Ok(())
        } else {
            Err(KsError::parse(
                "Invalid IdentifierTail, cant compile list index",
            ))
        }
    }

    fn identifier_tuple_index(
        &mut self,
        index: i32,
        last_collection_id: &mut Option<CollectionId>,
        assign: bool,
    ) -> KsResult<()> {
        if let Some(collection_id) = last_collection_id {
            let collection = self.environment.collection(*collection_id)?;
            if let Collection::Tuple { children } = collection {
                if let Some(collection_id) = children.get(index as usize) {
                    *last_collection_id = collection_id.clone();
                }

                self.insert_constant(Constant::Integer(index as i64))?;
                self.insert_collection(assign)?;
            }

            Ok(())
        } else {
            Err(KsError::parse(
                "Invalid IdentifierTail, cant compile tuple index",
            ))
        }
    }

    fn identifier(&mut self, identifier: Vec<IdentifierTail>, assign: bool) -> KsResult<()> {
        let mut last_collection_id: Option<CollectionId> = None;

        for segment in identifier {
            match segment {
                IdentifierTail::Name(name) => {
                    self.identifier_name(name, &mut last_collection_id, assign)
                }
                IdentifierTail::Call(expressions) => self.identifier_call(expressions, assign),
                IdentifierTail::Index(expression) => {
                    self.identifier_index(expression, &mut last_collection_id, assign)
                }
                IdentifierTail::TupleIndex(index) => {
                    self.identifier_tuple_index(index, &mut last_collection_id, assign)
                }
            }?;
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

    fn list_literal(&mut self, mut expressions: Vec<Expression>) -> KsResult<()> {
        let collection_size = expressions.len();
        let mut child: Option<CollectionId> = None;

        if collection_size != 0 {
            let expression = expressions.remove(0);
            self.compile_expression(expression)?;
            child = self.environment.temp_collection();
        }

        let collection = Collection::List { child };
        let collection_id = self.environment.register_collection(collection);
        self.environment.set_temp_collection(collection_id);

        for expression in expressions {
            self.compile_expression(expression)?;
        }

        self.insert(Instruction::LoadCollection(collection_size))?;
        Ok(())
    }

    fn tuple_literal(&mut self, expressions: Vec<Expression>) -> KsResult<()> {
        let collection_size = expressions.len();
        let mut children = Vec::<Option<CollectionId>>::new();

        for expression in expressions {
            self.compile_expression(expression)?;

            let temp_collection = self.environment.temp_collection();
            children.push(temp_collection);
        }

        let collection = Collection::Tuple { children };
        let collection_id = self.environment.register_collection(collection);
        self.environment.set_temp_collection(collection_id);

        self.insert(Instruction::LoadCollection(collection_size))?;
        Ok(())
    }

    fn module_literal(&mut self, module: BTreeMap<String, Expression>) -> KsResult<()> {
        let collection_size = module.len();
        let mut children = Vec::<Option<CollectionId>>::new();
        let mut indeces = HashMap::<String, VariableId>::new();

        for (name, expression) in module {
            self.compile_expression(expression)?;

            indeces.insert(name, children.len() as u64);
            let temp_collection = self.environment.temp_collection();
            children.push(temp_collection);
        }

        let collection = Collection::Module { children, indeces };
        let collection_id = self.environment.register_collection(collection);
        self.environment.set_temp_collection(collection_id);

        self.insert(Instruction::LoadCollection(collection_size))?;

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

    fn function_literal(
        &mut self,
        parameters: Vec<Parameter>,
        body: Vec<Statement>,
        captured: Vec<String>,
    ) -> KsResult<()> {
        self.function(parameters, body, captured)?;

        Ok(())
    }

    fn compile_expression(&mut self, expression: Expression) -> KsResult<()> {
        match expression {
            Expression::NullLiteral => self.insert_constant(Constant::Null),
            Expression::BooleanLiteral(boolean) => self.insert_constant(Constant::Boolean(boolean)),
            Expression::IntegerLiteral(integer) => {
                self.insert_constant(Constant::Integer(integer as i64))
            }
            Expression::FloatLiteral(float) => self.insert_constant(Constant::Float(float)),
            Expression::StringLiteral(string) => self.insert_constant(Constant::String(string)),
            Expression::Identifier(identifier) => self.identifier(identifier, false),
            Expression::ListLiteral(expressions) => self.list_literal(expressions),
            Expression::TupleLiteral(expressions) => self.tuple_literal(expressions),
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
            Expression::FunctionLiteral {
                parameters,
                return_type: _,
                block: body,
                captured,
            } => self.function_literal(parameters, body, captured),
        }?;

        Ok(())
    }
}
