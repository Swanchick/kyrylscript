use std::collections::HashMap;
use std::ops::Add;

use super::constant::Constant;

use crate::compiler::instruction::{self, Instruction};
use crate::parser::expression;
use crate::parser::operator::Operator;
use crate::parser::{ expression::Expression, statement::Statement};

pub struct Compiler {
    statements: Vec<Statement>,
    functions: HashMap<String, Vec<Instruction>>
}

impl Compiler {
    pub fn new(statements: Vec<Statement>) -> Compiler {
        Compiler { 
            statements,
            functions: HashMap::new()
        }
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
                    self.start_compile_instruction(value)
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

    fn start_compile_instruction(&self, expression: &Expression) -> Vec<Instruction> {
        self.compile_expression(expression, Vec::new())
    }

    fn compile_expression(&self, expression: &Expression, mut instructions: Vec<Instruction>) -> Vec<Instruction> {
        match expression {
            Expression::NullLiteral => instructions.push(Instruction::LoadConst(Constant::Null)),
            Expression::IntegerLiteral(integer) => instructions.push(Instruction::LoadConst(Constant::Integer(*integer))),
            Expression::FloatLiteral(float) => instructions.push(Instruction::LoadConst(Constant::Float(*float))),
            Expression::StringLiteral(string) => instructions.push(Instruction::LoadConst(Constant::String(string.clone()))),
            Expression::BooleanLiteral(boolean) => instructions.push(Instruction::LoadConst(Constant::Boolean(*boolean))),
            Expression::Identifier(name) => instructions.push(Instruction::LoadVar(name.clone())),
            
            Expression::FunctionCall(name, arguments) => {
                for argument in arguments {
                    instructions = self.compile_expression(argument, instructions);
                }

                instructions.push(Instruction::Call { function: name.clone(), args: arguments.len() });
            },
            
            Expression::ListLiteral(elements) => {
                for element in elements {
                    instructions = self.compile_expression(element, instructions);
                }

                instructions.push(Instruction::LoadList);
            },

            Expression::TupleLiteral(elements) => {
                for element in elements {
                    instructions = self.compile_expression(element, instructions);
                }

                instructions.push(Instruction::LoadTuple);
            },

            Expression::BinaryOp {
                left,
                operator,
                right,
            } => {
                instructions = self.compile_binary_op(left, right, operator, instructions);
            },

            Expression::FunctionLiteral {
                parameters,
                return_type,
                block,
            } => todo!(),

            Expression::UnaryOp {
                expression,
                operator,
            } => {
                instructions = self.compile_unary_op(expression, operator, instructions);
            }

            Expression::FrontUnaryOp {
                expression,
                operator,
            } => todo!(),

            Expression::ListIndex { left, index } => todo!(),
            Expression::TupleIndex { left, indeces } => todo!()
        }

        instructions
    }

    fn compile_binary_op(
        &self, 
        left: &Box<Expression>, 
        right: &Box<Expression>,
        operator: &Operator,
        mut instructions: Vec<Instruction>
    ) -> Vec<Instruction> {
        instructions = self.compile_expression(&left, instructions);
        instructions = self.compile_expression(&right, instructions);

        match operator {
            Operator::Plus => instructions.push(Instruction::Add),
            Operator::Minus => instructions.push(Instruction::Minus),
            Operator::Multiply => instructions.push(Instruction::Mul),
            Operator::Divide => instructions.push(Instruction::Div),
            Operator::EqualEqual => instructions.push(Instruction::Eq),
            Operator::GreaterEqual => instructions.push(Instruction::GreaterEq),
            Operator::Greater => instructions.push(Instruction::Greater),
            Operator::LessEqual => instructions.push(Instruction::LessEq),
            Operator::Less => instructions.push(Instruction::Less),
            Operator::NotEqual => instructions.push(Instruction::NotEq),
            _ => unreachable!()
        }

        instructions
    }

    fn compile_unary_op(
        &self, 
        expression: &Box<Expression>, 
        operator: &Operator,
        mut instructions: Vec<Instruction>
    ) -> Vec<Instruction> {
        instructions = self.compile_expression(&expression, instructions);
        
        match operator {
            Operator::Not => instructions.push(Instruction::Not),
            Operator::Minus => instructions.push(Instruction::Minus),
            _ => unreachable!()
        }

        instructions
    }
}
