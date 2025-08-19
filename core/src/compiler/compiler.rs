use std::collections::HashMap;

use super::instruction::Instruction;
use super::constant::Constant;

use crate::global::constants::{
    Instructions, 
    ANONYNOUS_FUNCTION_ENCAPSULATION, 
    FUNCTION_ENCAPSULATION, 
    ITERATOR_LIST_NAME, 
    ITERATOR_NAME, 
    MAIN_FUNCTION
};

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

    pub fn display(&self) {
        for (function_name, function) in &self.functions {
            print!("{} ", function_name);
            println!("{:?}:", function.get_args());

            for (i, instruction) in function.get_instructions().iter().enumerate() {
                println!("    {}: {:?}", i, instruction);
            }

            println!("");
        }
    }

    pub fn get_instructions(&self, name: &str) -> Option<&Function>{
        self.functions.get(name)
    }

    pub fn start_compile(&mut self, statements: &Vec<Statement>) {
        let mut instructions: Instructions = Vec::new();

        instructions = self.compile_statments(statements, instructions);

        self.functions.insert(String::from(MAIN_FUNCTION), Function::method(instructions));
    }

    fn has_return(&self, instructions: &Instructions) -> bool {
        if let Some(instruction) = instructions.last() {
            matches!(instruction, Instruction::Return)
        } else {
            false
        }
    }

    pub fn compile_statments(&mut self, statements: &Vec<Statement>, mut instructions: Instructions) -> Instructions {
        for statement in statements {
            let mut statement_instructions = self.compile_statment(statement);

            let has_return = self.has_return(&statement_instructions);

            instructions.append(&mut statement_instructions);

            if has_return {
                break;
            }
        }

        instructions
    }

    fn compile_statment(&mut self, statement: &Statement) -> Instructions {
        let mut instructions: Instructions = Vec::new();
        
        match statement {
            Statement::VariableDeclaration {
                name,
                public,
                data_type: _,
                value,
            } => {
                let name = name.clone();

                let mut value_instructions = if let Some(value) = value {
                    self.start_compile_expression(value)
                } else {
                    vec![Instruction::LoadConst(Constant::Null)]
                };

                instructions.append(&mut value_instructions);

                let store = if *public {
                    Instruction::PubStore(name)
                } else {
                    Instruction::Store(name)
                };

                instructions.push(store);
            }

            Statement::Assignment { name, value } => {
                instructions = self.compile_expression(value, instructions);

                instructions.push(Instruction::Assign(name.clone()));
            },

            Statement::AssignmentIndex { name, index, value } => {
                instructions.push(Instruction::LoadVar(name.clone()));
                
                for (i, index_value) in index.iter().enumerate() {
                    let is_last = i == index.len() - 1;
                    
                    if is_last {
                        instructions = self.compile_expression(value, instructions);
                        instructions = self.compile_expression(index_value, instructions);

                        instructions.push(Instruction::AssignListIndex);

                        break;
                    }

                    instructions = self.compile_expression(index_value, instructions);
                    instructions.push(Instruction::LoadFromList);
                }
            },

            Statement::AddValue { name, value } => {
                instructions.push(Instruction::LoadVar(name.clone()));
                instructions = self.compile_expression(value, instructions);

                instructions.push(Instruction::Add);

                instructions.push(Instruction::Store(name.clone()));
            }, 

            Statement::RemoveValue { name, value } => {
                instructions.push(Instruction::LoadVar(name.clone()));
                instructions = self.compile_expression(value, instructions);

                instructions.push(Instruction::Minus);
                instructions.push(Instruction::Store(name.clone()));
            },

            Statement::ReturnStatement { value } => {
                if let Some(value) = value {
                    instructions = self.compile_expression(value, instructions);
                } else {
                    instructions.push(Instruction::LoadConst(Constant::Null));
                }

                instructions.push(Instruction::Return);
            },

            Statement::IfStatement {
                condition,
                body,
                else_body,
            } => {
                instructions.push(Instruction::Enter);

                let mut body = self.compile_statments(body, Vec::new());
                let body_len = body.len() as i32;

                instructions = self.compile_expression(condition, instructions);
                
                let mut jump_distance = body_len + 1;
                

                let mut else_body = if let Some(else_body) = else_body {
                    jump_distance += 1;
                    
                    self.compile_statments(else_body, Vec::new())
                } else {
                    Vec::new()
                };

                instructions.push(Instruction::JumpIfFalse(jump_distance));
                instructions.append(&mut body);
                instructions.push(Instruction::Jump(else_body.len() as i32 + 1));
                instructions.append(&mut else_body);
                instructions.push(Instruction::Exit);
            },

            Statement::WhileStatement { 
                condition, 
                body 
            } => {
                let mut body = self.compile_statments(body, Vec::new());
                let body_len = body.len() as i32;
                
                let mut condition = self.compile_expression(condition, Vec::new());
                let condition_len = condition.len() as i32;
                
                instructions.append(&mut condition);
                instructions.push(Instruction::JumpIfFalse(body_len + 3));
                instructions.push(Instruction::Enter);

                instructions.append(&mut body);
                instructions.push(Instruction::Exit);
                instructions.push(Instruction::Jump(-body_len - condition_len - 3));
            },

            Statement::ForLoopStatement { 
                name, 
                list, 
                body 
            } => {
                let name = name.clone();
                let iter_name = format!("{}{}", ITERATOR_NAME, name);
                let iter_list_name = format!("{}{}", ITERATOR_LIST_NAME, name);
                
                instructions.push(Instruction::Enter);

                instructions.push(Instruction::LoadConst(Constant::Integer(0)));
                instructions.push(Instruction::Store(iter_name.clone()));

                instructions = self.compile_expression(list, instructions);
                instructions.push(Instruction::Store(iter_list_name.clone()));

                let mut condition_body: Instructions = Vec::new();

                condition_body.push(Instruction::LoadVar(iter_name.clone()));
                condition_body.push(Instruction::LoadVar(iter_list_name.clone()));
                condition_body.push(Instruction::ListLen);
                condition_body.push(Instruction::Less);
                
                let mut for_body: Instructions = Vec::new();
                
                for_body.push(Instruction::Enter);
                
                for_body.push(Instruction::LoadVar(iter_list_name.clone()));
                for_body.push(Instruction::LoadVar(iter_name.clone()));
                for_body.push(Instruction::LoadFromList);
                for_body.push(Instruction::Store(name));
                
                for_body = self.compile_statments(body, for_body);

                for_body.push(Instruction::Exit);

                for_body.push(Instruction::LoadVar(iter_name.clone()));
                for_body.push(Instruction::LoadConst(Constant::Integer(1)));
                for_body.push(Instruction::Add);
                for_body.push(Instruction::Store(iter_name.clone()));
                
                let for_body_len = for_body.len() as i32;
                let condition_body_len = condition_body.len() as i32;
                for_body.push(Instruction::Jump(-condition_body_len - for_body_len - 1));

                let for_body_len = for_body.len() as i32;
                condition_body.push(Instruction::JumpIfFalse(for_body_len + 1));


                instructions.append(&mut condition_body);
                instructions.append(&mut for_body);
                instructions.push(Instruction::Exit);
            },

            Statement::Expression { value } => {
                instructions = self.compile_expression(value, instructions);
                instructions.push(Instruction::End);
            },

            Statement::Function {
                name,
                public,
                return_type: _,
                parameters,
                body,
            } => {
                let name = name.clone();

                let final_function_name = format!("{}{}", FUNCTION_ENCAPSULATION, name);
                let mut function_instructions: Instructions = Vec::new();
                function_instructions = self.compile_statments(body, function_instructions);

                let args: Vec<String> = 
                    parameters
                        .iter()
                        .map(|parameter| parameter.name.clone())
                        .collect();

                self.functions.insert(
                    final_function_name.clone(), 
                    Function::new(function_instructions, args)
                );

                instructions.push(Instruction::LoadConst(Constant::Function(final_function_name)));
                
                let store = if *public {
                    Instruction::PubStore(name)
                } else {
                    Instruction::Store(name)
                };

                instructions.push(store);
            }

            Statement::EarlyReturn { 
                name, 
                body 
            } => {
                let name = name.clone();

                instructions.push(Instruction::LoadVar(name));
                instructions.push(Instruction::LoadConst(Constant::Null));
                instructions.push(Instruction::Eq);

                let mut prepare_early_body: Instructions = Vec::new();
                prepare_early_body.push(Instruction::Enter);

                let mut early_body: Instructions = if let Some(body) = body {
                    prepare_early_body = self.compile_statments(body, prepare_early_body);

                    if self.has_return(&prepare_early_body) {
                        prepare_early_body
                    } else {
                        prepare_early_body.push(Instruction::LoadConst(Constant::Null));
                        prepare_early_body.push(Instruction::Return);
                        prepare_early_body
                    }
                } else {
                    prepare_early_body.push(Instruction::LoadConst(Constant::Null));
                    prepare_early_body.push(Instruction::Return);
                    prepare_early_body
                };

                early_body.push(Instruction::Exit);

                instructions.push(Instruction::JumpIfFalse(early_body.len() as i32 + 1));
                instructions.append(&mut early_body);

            },

            Statement::Use {
                file_name: _,
                body: _,
                global: _,
            } => todo!()
        }


        instructions
    }

    fn start_compile_expression(&mut self, expression: &Expression) -> Instructions {
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
                instructions.push(Instruction::LoadVar(name.clone()));
                for argument in arguments {
                    instructions = self.compile_expression(argument, instructions);
                }

                instructions.push(Instruction::Call { args: arguments.len() });
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
                function_instructions = self.compile_statments(block, function_instructions);

                let args: Vec<String> = 
                    parameters
                        .iter()
                        .map(|parameter| parameter.name.clone())
                        .collect();

                self.functions.insert(
                    function_name.clone(), 
                    Function::new(function_instructions, args)
                );

                instructions.push(Instruction::LoadConst(
                    Constant::Function(function_name)
                ));
            },

            Expression::UnaryOp {
                expression,
                operator,
            } => instructions = self.compile_unary_op(expression, operator, instructions),

            Expression::FrontUnaryOp {
                expression,
                operator,
            } => instructions = self.compile_front_unary_op(expression, operator, instructions),

            Expression::ListIndex { left, index } => {
                instructions = self.compile_expression(&left, instructions);
                instructions = self.compile_expression(&index, instructions);

                instructions.push(Instruction::LoadFromList);
            },

            Expression::TupleIndex { left, indeces } => {
                instructions = self.compile_expression(&left, instructions);
                
                for index in indeces {
                    instructions.push(Instruction::LoadFromTuple(*index as usize));
                }
            }
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

    fn compile_front_unary_op(
        &mut self,
        expression: &Box<Expression>,
        operator: &Operator,
        mut instructions: Instructions,
    ) -> Instructions {
        instructions = self.compile_expression(&expression, instructions);

        match operator {
            Operator::PlusPlus => {
                instructions.push(Instruction::LoadConst(Constant::Integer(1)));
                instructions.push(Instruction::Add);
            },
            Operator::MinusMinus => {
                instructions.push(Instruction::LoadConst(Constant::Integer(1)));
                instructions.push(Instruction::Minus);
            },
            Operator::Not => {
                instructions.push(Instruction::Clone);
            },
            _ => unreachable!()
        }

        instructions
    }
}
