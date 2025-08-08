use std::collections::HashMap;

use super::instruction::Instruction;
use super::constant::Constant;
use super::globals::{Instructions, FUNCTION_ENCAPSULATION, ANONYNOUS_FUNCTION_ENCAPSULATION};

use crate::compiler::function::Function;
use crate::parser::operator::Operator;
use crate::parser::expression::Expression;
use crate::parser::statement::Statement;

pub struct Compiler {
    functions: HashMap<String, Function>,
    last_anonymous_function: usize
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler { 
            functions: HashMap::new(),
            last_anonymous_function: 0
        }
    }

    pub fn get_instructions(&self, name: &str) -> Option<&Function>{
        self.functions.get(name)
    }

    pub fn start_compile(&mut self, statements: Vec<Statement>) {
        let mut instructions: Instructions = Vec::new();

        instructions = self.compile_statments(statements, instructions);

        self.functions.insert(String::from("main"), Function::method(instructions));
    }

    pub fn compile_statments(&mut self, statements: Vec<Statement>, mut instructions: Instructions) -> Instructions {
        for statement in &statements {
            let mut statement_instructions = self.compile_statment(statement);
            
            instructions.append(&mut statement_instructions);
        }

        instructions
    }

    fn compile_statment(&mut self, statement: &Statement) -> Instructions {
        let mut instructions: Instructions = Vec::new();
        
        match statement {
            Statement::VariableDeclaration {
                name,
                public,
                data_type,
                value,
            } => {
                let mut value_instructions = if let Some(value) = value {
                    self.start_compile_instruction(value)
                } else {
                    vec![Instruction::LoadConst(Constant::Null)]
                };

                instructions.append(&mut value_instructions);
                instructions.push(Instruction::Store(name.clone()));
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
            } => {
                let final_function_name = format!("{}{}", FUNCTION_ENCAPSULATION, name);
                let mut function_instructions: Instructions = Vec::new();
                function_instructions = self.compile_statments(body.clone(), function_instructions);

                let args: Vec<String> = 
                    parameters
                        .iter()
                        .map(|parameter| parameter.name.clone())
                        .collect();

                self.functions.insert(
                    final_function_name, 
                    Function::new(function_instructions, args)
                );
            }

            Statement::EarlyReturn { name, body } => {}

            Statement::Use {
                file_name,
                body,
                global,
            } => {}
        }


        instructions
    }

    fn start_compile_instruction(&mut self, expression: &Expression) -> Instructions {
        self.compile_expression(expression, Vec::new())
    }

    fn compile_expression(&mut self, expression: &Expression, mut instructions: Instructions) -> Instructions {
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
                return_type: _,
                block,
            } => {
                let function_name = format!(
                    "{}{}", 
                    ANONYNOUS_FUNCTION_ENCAPSULATION, 
                    self.last_anonymous_function
                );

                self.last_anonymous_function += 1;
                let mut function_instructions: Instructions = Vec::new();
                function_instructions = self.compile_statments(block.clone(), function_instructions);

                let args: Vec<String> = 
                    parameters
                        .iter()
                        .map(|parameter| parameter.name.clone())
                        .collect();

                self.functions.insert(
                    function_name, 
                    Function::new(function_instructions, args)
                );
            },

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

            Expression::ListIndex { left, index } => {
                instructions = self.compile_expression(&left, instructions);

            },
            Expression::TupleIndex { left, indeces } => todo!()
        }

        instructions
    }

    fn compile_binary_op(
        &mut self, 
        left: &Box<Expression>, 
        right: &Box<Expression>,
        operator: &Operator,
        mut instructions: Instructions
    ) -> Instructions {
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
        &mut self, 
        expression: &Box<Expression>, 
        operator: &Operator,
        mut instructions: Instructions
    ) -> Instructions {
        instructions = self.compile_expression(&expression, instructions);
        
        match operator {
            Operator::Not => instructions.push(Instruction::Not),
            Operator::Minus => instructions.push(Instruction::Minus),
            _ => unreachable!()
        }

        instructions
    }
}
