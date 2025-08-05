use super::constant::Constant;

use crate::compiler::instruction::Instruction;
use crate::parser::{ expression::Expression, statement::Statement};

pub struct Compiler {
    statements: Vec<Statement>,
}

impl Compiler {
    pub fn new(statements: Vec<Statement>) -> Compiler {
        Compiler { statements }
    }

    pub fn compile_statments(&self) -> Vec<Instruction> {
        for statement in &self.statements {
            self.compile_statment(statement);
        }

        todo!()
    }

    fn compile_statment(&self, statement: &Statement) -> Vec<Instruction> {
        match statement {
            Statement::VariableDeclaration {
                name,
                public,
                data_type,
                value,
            } => {
                let instructions = if let Some(value) = value {
                    self.compile_expression(value)
                } else {
                    vec![Instruction::LoadConst(Constant::Null)]
                };

                todo!()
            }

            Statement::Assigment { name, value } => {}

            Statement::AssigmentIndex { name, index, value } => {}

            Statement::AddValue { name, value } => {}

            Statement::RemoveValue { name, value } => {}

            Statement::ReturnStatement { value } => {}

            Statement::IfStatement {
                condition,
                body,
                else_body,
            } => {}

            Statement::WhileStatement { condition, body } => {}

            Statement::ForLoopStatement { name, list, body } => {}

            Statement::Expression { value } => {}

            Statement::Function {
                name,
                public,
                return_type,
                parameters,
                body,
            } => {}

            Statement::EarlyReturn { name, body } => {}

            Statement::Use {
                file_name,
                body,
                global,
            } => {}
        }


        todo!()
    }

    fn compile_expression(&self, expression: &Expression) -> Vec<Instruction> {
        match expression {
            Expression::NullLiteral => {}
            Expression::IntegerLiteral(integer) => {}
            Expression::FloatLiteral(float) => {}
            Expression::StringLiteral(string) => {}
            Expression::BooleanLiteral(boolean) => {}
            Expression::Identifier(name) => {}
            Expression::FunctionCall(name, arguments) => {}
            Expression::ListLiteral(elements) => {}
            Expression::TupleLiteral(elements) => {}

            Expression::BinaryOp {
                left,
                operator,
                right,
            } => {}

            Expression::FunctionLiteral {
                parameters,
                return_type,
                block,
            } => {}

            Expression::UnaryOp {
                expression,
                operator,
            } => {}

            Expression::FrontUnaryOp {
                expression,
                operator,
            } => {}

            Expression::IdentifierIndex { left, index } => {}
            Expression::TupleIndex { left, indeces } => {}
        }

        todo!()
    }
}
