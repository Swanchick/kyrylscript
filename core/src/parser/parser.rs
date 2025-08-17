use crate::global::ks_path::{KsPath, KS_MODULE_FILE};
use crate::lexer::lexer::Lexer;
use crate::lexer::token::Token;
use crate::lexer::token_pos::TokenPos;
use crate::native_registry::native_registry::NativeRegistry;
use crate::native_registry::native_types::NativeTypes;
use crate::global::data_type::DataType;

use super::operator::Operator;
use super::expression::Expression;
use super::parameter::Parameter;
use super::semantic_analyzer::SemanticAnalyzer;
use super::statement::Statement;
use super::context::Context;

use std::io;

pub struct Parser {
    tokens: Vec<Token>,
    token_pos: Vec<TokenPos>,
    current_token: usize,
    semantic_analyzer: SemanticAnalyzer,
    function_context: Context,
    path: KsPath,
    root: KsPath
}

impl Parser {
    pub fn new(
        tokens: Vec<Token>, 
        token_pos: Vec<TokenPos>, 
        path: KsPath, 
        root: KsPath
    ) -> Parser {
        let mut semantic_analyzer = SemanticAnalyzer::new();
        
        let registry = NativeRegistry::get();
        {
            let registry = registry.borrow();

            for (name, native) in registry.get_natives() {
                match native {
                    NativeTypes::NativeFunction(function) => {
                        semantic_analyzer.register_rust_function(name.clone(), function);
                    }
                }
            }
        }

        Parser {
            tokens,
            token_pos,
            current_token: 0,
            semantic_analyzer: semantic_analyzer,
            function_context: Context::None,
            path,
            root
        }
    }

    pub fn with_semantic_analyzer(
        tokens: Vec<Token>, 
        token_pos: Vec<TokenPos>, 
        mut semantic_analyzer: SemanticAnalyzer, 
        path: KsPath,
        root: KsPath
    ) -> Parser {
        let registry = NativeRegistry::get();
        {
            let registry = registry.borrow();

            for (name, native) in registry.get_natives() {
                match native {
                    NativeTypes::NativeFunction(function) => {
                        semantic_analyzer.register_rust_function(name.clone(), function);
                    }
                }
            }
        }
        
        Parser {
            tokens,
            token_pos,
            current_token: 0,
            semantic_analyzer: semantic_analyzer,
            function_context: Context::None,
            path,
            root
        }
    }

    pub fn start(&mut self) -> io::Result<Vec<Statement>> {
        let result = self.parse_block_statement();

        match result {
            Ok(statements) => {
                Ok(statements)
            },

            Err(e) => {
                let pos = self.peek_pos();

                let file = match pos.get_source() {
                    Some(path) => path,
                    None => "Main"
                };

                let error = format!("kyryl-script: At {}:{}: {}", file, pos.get_line() + 1, e.to_string());

                Err(io::Error::new(e.kind(), error))
            }
        }
    }

    pub fn get_semantic_analyzer(&self) -> &SemanticAnalyzer {
        &self.semantic_analyzer
    }

    pub fn parse_block_statement(&mut self) -> io::Result<Vec<Statement>> {
        let mut statements: Vec<Statement> = Vec::new();
        
        while !(self.match_token(&Token::RightBrace) || self.is_end()) {
            let statement = self.parse_statement()?;
            

            if let Some(statement) = statement {
                statements.push(statement);
            } else {
                break;
            }
        }

        Ok(statements)
    }

    fn parse_parameters(&mut self) -> io::Result<Vec<Parameter>> {
        if self.match_token(&Token::RightParenthesis) {
            return Ok(Vec::new());
        }
        
        let mut parameters: Vec<Parameter> = Vec::new();
        
        loop {
            let parameter = self.parse_parameter()?;
            parameters.push(parameter);

            if self.match_token(&Token::RightParenthesis) {
                break;
            } else {
                self.consume_token(Token::Comma)?;
            }
        }

        Ok(parameters)
    }

    fn parse_parameter(&mut self) -> io::Result<Parameter> { 
        let name = self.consume_identifier()?;
        self.consume_token(Token::Colon)?;
        let data_type = self.parse_data_type()?;
        
        self.semantic_analyzer.save_variable(name.clone(), data_type.clone());

        let parameter = Parameter {
            name: name,
            data_type: data_type
        };

        Ok(parameter)
    }

    pub fn parse_statement(&mut self) -> io::Result<Option<Statement>> {
        let public = self.match_token(&Token::Pub);

        if let Context::Function { return_data: _ } = self.function_context {
            if public {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid context for public visibility!"));
            }
        }
        
        match self.advance() {
            Some(Token::Let) => return Ok(Some(self.parse_variable_declaration_statement(public)?)),
            Some(Token::Return) => return Ok(Some(self.parse_return_statement()?)),
            Some(Token::If) => return Ok(Some(self.parse_if_statement()?)),
            Some(Token::While) => return Ok(Some(self.parse_while_statement()?)),
            Some(Token::For) => return Ok(Some(self.parse_for_statement()?)),
            Some(Token::Function) => return Ok(Some(self.parse_function(public)?)),
            Some(Token::Use) => return Ok(Some(self.parse_use()?)),
            Some(Token::Identifier(name)) => {
                match self.advance() {
                    Some(Token::Equal) => return Ok(Some(self.parse_assignment_statement(name)?)),
                    Some(Token::PlusEqual) => return Ok(Some(self.parse_add_value_statment(name)?)),
                    Some(Token::MinusEqual) => return Ok(Some(self.parse_remove_value_statement(name)?)),
                    Some(Token::Question) => return Ok(Some(self.parse_early_return(name)?)),
                    Some(Token::LeftSquareBracket) => {
                        let mut indexes: Vec<Expression> = Vec::new();
                
                        loop {
                            let index = self.parse_expression()?;
                            self.consume_token(Token::RightSquareBracket)?;

                            indexes.push(index);

                            if !self.match_token(&Token::LeftSquareBracket) {
                                break;
                            }
                        }
                        
                        if self.match_token(&Token::Equal) {
                            let value  = self.parse_expression()?;
                            self.consume_token(Token::Semicolon)?;

                            return Ok(Some(Statement::AssignmentIndex { name: name, index: indexes, value: value }));
                        }
                    }

                    _ => self.back(),
                }
            },

            None => return Ok(None),
            _ => {},
        }
        
        self.back();
        Ok(Some(self.parse_expression_statement()?))
    }

    pub fn parse_function(&mut self, public: bool) -> io::Result<Statement> {
        let function_name = self.consume_identifier()?;

        self.consume_token(Token::LeftParenthesis)?;

        self.semantic_analyzer.enter_function_enviroment();

        let parameters = self.parse_parameters()?;

        let function_type = if self.match_token(&Token::Colon) {
            self.parse_data_type()?
        } else {
            DataType::void()
        };

        self.consume_token(Token::LeftBrace)?;
        
        let function_data_type = DataType::Function { 
            parameters: DataType::from_parameters(&parameters), 
            return_type: Box::new(function_type.clone())
        };

        self.function_context = Context::Function { return_data: function_data_type.clone() };
        let block = self.parse_block_statement()?;
        self.function_context = Context::None;

        self.semantic_analyzer.exit_function_enviroment()?;

        if public {
            self.semantic_analyzer.global_save_variable(
                function_name.clone(), 
                function_data_type
            );
        } else {
            self.semantic_analyzer.save_variable(
                function_name.clone(), 
                function_data_type
            );
        }

        Ok(
            Statement::Function { 
                name: function_name,
                public: public,
                return_type: function_type, 
                parameters: parameters, 
                body: block 
            }
        )
    }


    fn parse_use(&mut self) -> io::Result<Statement> {
        let mut current_path = self.path.parent();

        if self.match_token(&Token::Root) {
            self.consume_token(Token::ColonColon)?;
            current_path = self.root.clone();
        }
        let name = self.consume_identifier()?;
        current_path.push(name);

        self.consume_token(Token::Semicolon)?;

        let statement = if current_path.is_file() {
            self.parse_use_file(current_path, self.root.clone())
        } else if current_path.is_dir() {
            self.parse_use_dir(current_path, self.root.clone())
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Cannot use directory {}")) 
        }?;
        
        match self.advance() {
            Some(Token::ColonColon) => {
                if let Some(Token::LeftBrace) = self.advance() {

                } else {

                }
            }

            Some(Token::Semicolon) => {
                
            }
            
            None => {}
            _ => {}
        }

        todo!()
    }

    fn parse_use_file(&self, path: KsPath, root: KsPath) -> io::Result<Statement> {
        if let Some(source_path) = path.to_string() {
            let mut lexer = Lexer::load(source_path)?;
            lexer.lexer()?;

            let mut parser = Parser::with_semantic_analyzer(
                lexer.get_tokens().clone(), 
                lexer.get_token_pos().clone(), 
                SemanticAnalyzer::with_global(self.semantic_analyzer.get_global()), 
                path.clone(), 
                root
            );

            let body = parser.parse_block_statement()?;
            Ok(Statement::Use { 
                file_name: source_path.to_string(), 
                body: body,
                global: parser.get_semantic_analyzer().get_global()
            })
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Cannot find file!"))
        }
    }

    fn parse_use_dir(&self, mut path: KsPath, root: KsPath) -> io::Result<Statement> {
        path.push(KS_MODULE_FILE.to_string());

        if let Some(source_path) = path.to_string() {
            let mut lexer = Lexer::load(source_path)?;
            lexer.lexer()?;

            let mut parser = Parser::with_semantic_analyzer(
                lexer.get_tokens().clone(), 
                lexer.get_token_pos().clone(), 
                SemanticAnalyzer::with_global(self.semantic_analyzer.get_global()), 
                path.clone(), 
                root
            );

            let body = parser.parse_block_statement()?;

            Ok(Statement::Use { 
                file_name: source_path.to_string(), 
                body: body, 
                global: parser.get_semantic_analyzer().get_global() 
            })
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Cannot find directory module!"))
        }
    }

    fn parse_early_return(&mut self, name: String) -> io::Result<Statement> {
        let body: Option<Vec<Statement>> = if self.match_token(&Token::LeftBrace) {
            Some(self.parse_block_statement()?)
        } else {
            None
        };

        self.consume_token(Token::Semicolon)?;

        Ok(Statement::EarlyReturn { name: name, body: body })
    }

    fn parse_for_statement(&mut self) -> io::Result<Statement> {
        let name = self.consume_identifier()?;

        self.consume_token(Token::In)?;
        let expression = self.parse_expression()?;
        let data_type = self.semantic_analyzer.get_data_type(&expression)?;

        self.semantic_analyzer.enter_function_enviroment();

        match data_type {
            DataType::List(child_data_type) => self.semantic_analyzer.save_variable(name.clone(), *child_data_type),
            DataType::String => self.semantic_analyzer.save_variable(name.clone(), DataType::String),
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "For loop statement mismatch type!"))
        }

        self.consume_token(Token::LeftBrace)?;
        let body = self.parse_block_statement()?;

        self.semantic_analyzer.exit_function_enviroment()?;
        
        Ok(Statement::ForLoopStatement { name: name, list: expression, body: body })
    }

    fn parse_expression_statement(&mut self) -> io::Result<Statement> {
        let expression = self.parse_expression()?;
        self.semantic_analyzer.get_data_type(&expression)?;

        self.consume_token(Token::Semicolon)?;

        Ok(Statement::Expression { value: expression })
    }

    fn parse_add_value_statment(&mut self, name: String) -> io::Result<Statement> {
        let expression = self.parse_expression()?;
        self.consume_token(Token::Semicolon)?;

        Ok(Statement::AddValue { name: name, value: expression })
    }

    fn parse_remove_value_statement(&mut self, name: String) -> io::Result<Statement> {
        let expression = self.parse_expression()?;
        self.consume_token(Token::Semicolon)?;

        Ok(Statement::RemoveValue { name: name, value: expression })
    }

    fn parse_variable_declaration_statement(&mut self, public: bool) -> io::Result<Statement> {
        let name = self.consume_identifier()?;

        let data_type = if self.match_token(&Token::Colon) {
            Some(self.parse_data_type()?)
        } else {
            None
        };
        
        self.consume_token(Token::Equal)?;
        let expression = self.parse_expression()?;

        let dt = self.semantic_analyzer.get_data_type(&expression)?;
        
        if let Some(data_type_to_check) = &data_type {
            if dt != data_type_to_check.clone() && !DataType::is_void(&dt) {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Different data types in expression and actual data type."));
            } 
        }

        if public {
            self.semantic_analyzer.global_save_variable(name.clone(), dt.clone());
        } else {
            self.semantic_analyzer.save_variable(name.clone(), dt.clone());
        }

        self.consume_token(Token::Semicolon)?;

        Ok(
            Statement::VariableDeclaration {
                name,
                public,
                data_type,
                value: Some(expression)
            }
        )
    }

    fn parse_return_statement(&mut self) -> io::Result<Statement> { 
        if let Context::Function{ return_data} = self.function_context.clone() {
            let expression = self.parse_expression()?;
            let data_type = self.semantic_analyzer.get_data_type(&expression)?;

            if let DataType::Function { parameters: _, return_type } = return_data {
                if *return_type != data_type {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Mismatch return and function return types!"));
                }

                self.consume_token(Token::Semicolon)?;
                return Ok(Statement::ReturnStatement { value: Some(expression) });
            }
        } 

        Err(io::Error::new(io::ErrorKind::InvalidData, "No function context for return!"))
    }

    fn parse_assignment_statement(&mut self, name: String) -> io::Result<Statement> {
        let expression = self.parse_expression()?;
        let data_type = self.semantic_analyzer.get_data_type(&expression)?;
        let data_type_to_check = self.semantic_analyzer.get_variable(&name)?;

        match data_type_to_check {
            DataType::Void(Some (null_type)) => {
                if *null_type != data_type && !DataType::is_void(&data_type) {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Assignment value mismatch!"))
                }
            }

            _ => {
                if data_type_to_check != data_type && !DataType::is_void(&data_type) {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Assignment value mismatch!"))
                }
            }
        }

        self.consume_token(Token::Semicolon)?;

        Ok(Statement::Assignment { name: name, value: expression })
    }

    fn parse_function_call_parameters(&mut self) -> io::Result<Vec<Expression>> {
        let mut parameters: Vec<Expression> = Vec::new();
        
        loop {
            let expression = self.parse_expression()?;
            parameters.push(expression);

            if !self.match_token(&Token::Comma) {
                break;
            }
        }

        Ok(parameters)
    }

    fn parse_if_statement(&mut self) -> io::Result<Statement> {
        let condition = self.parse_expression()?;

        let statment_data_type = self.semantic_analyzer.get_data_type(&condition)?;
        if statment_data_type != DataType::Bool {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "If statment condition mismatch data_type, expected bool!"));
        }

        self.consume_token(Token::LeftBrace)?;

        self.semantic_analyzer.enter_function_enviroment();
        let if_body = self.parse_block_statement()?;
        self.semantic_analyzer.exit_function_enviroment()?;

        let else_block= if self.match_token(&Token::Else) {
            self.consume_token(Token::LeftBrace)?;
            
            self.semantic_analyzer.enter_function_enviroment();
            let result = self.parse_block_statement()?;
            self.semantic_analyzer.exit_function_enviroment()?;

            Some(result)
        } else {
            None
        };

        Ok(
            Statement::IfStatement {
                condition: condition,
                body: if_body,
                else_body: else_block
            }
        )
    }

    fn parse_while_statement(&mut self) -> io::Result<Statement> {
        let condition = self.parse_expression()?;
        
        let condition_data_type = self.semantic_analyzer.get_data_type(&condition)?;
        if condition_data_type != DataType::Bool {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "While statment condition mismatch data_type, expected bool!"));
        }

        self.consume_token(Token::LeftBrace)?;

        self.semantic_analyzer.enter_function_enviroment();
        let block = self.parse_block_statement()?;
        self.semantic_analyzer.exit_function_enviroment()?;

        Ok(Statement::WhileStatement {
                condition: condition,
                body: block
        })
    }

    pub fn parse_expression(&mut self) -> io::Result<Expression> {
        self.parse_logic_or()
    }

    fn parse_logic_or(&mut self) -> io::Result<Expression> {
        let mut expression = self.parse_logic_and()?;
        
        while self.match_token(&Token::Or) {
            let right = self.parse_logic_and()?;

            expression = Expression::BinaryOp { left: Box::new(expression), operator: Operator::Or, right: Box::new(right) }
        }
        
        Ok(expression)
    }

    fn parse_logic_and(&mut self) -> io::Result<Expression> {
        let mut expression = self.parse_comparison()?;

        while self.match_token(&Token::And) {
            let right = self.parse_comparison()?;

            expression = Expression::BinaryOp { left: Box::new(expression), operator: Operator::And, right: Box::new(right) }
        }

        Ok(expression)
    }

    fn parse_comparison(&mut self) -> io::Result<Expression> {
        let mut expression = self.parse_addition()?;

        while self.match_token(&Token::EqualEqual)      ||
                self.match_token(&Token::NotEqual)    ||
                self.match_token(&Token::GreaterEqual)  ||
                self.match_token(&Token::LessEqual)     ||
                self.match_token(&Token::GreaterThan)   ||
                self.match_token(&Token::LessThan)
        {    
            let operator = match self.previous() {
                Token::EqualEqual => Operator::EqualEqual,
                Token::NotEqual => Operator::NotEqual,
                Token::GreaterEqual => Operator::GreaterEqual,
                Token::GreaterThan => Operator::Greater,
                Token::LessEqual => Operator::LessEqual,
                Token::LessThan => Operator::Less,
                _ => unreachable!()
            };

            let right = self.parse_addition()?;

            expression = Expression::BinaryOp { left: Box::new(expression), operator: operator, right: Box::new(right) }
        }

        Ok(expression)
    }

    fn parse_addition(&mut self) -> io::Result<Expression> {
        let mut expression = self.parse_multiplication()?;
        
        while self.match_token(&Token::Plus) || self.match_token(&Token::Minus) {
            let operator = match self.previous() {
                Token::Plus => Operator::Plus,
                Token::Minus => Operator::Minus,
                _ => unreachable!()
            };

            let right = self.parse_multiplication()?;

            expression = Expression::BinaryOp { left: Box::new(expression), operator: operator, right: Box::new(right) };
        }

        Ok(expression)
    }

    fn parse_multiplication(&mut self) -> io::Result<Expression> {
        let mut expression = self.parse_power()?;

        while self.match_token(&Token::Multiply) || self.match_token(&Token::Divide) {
            let operator = match self.previous() {
                Token::Multiply => Operator::Multiply,
                Token::Divide => Operator::Divide,
                _ => unreachable!()
            };

            let right = self.parse_power()?;

            expression = Expression::BinaryOp { left: Box::new(expression), operator: operator, right: Box::new(right) };
        }

        Ok(expression)
    }

    fn parse_power(&mut self) -> io::Result<Expression> {
        let mut expression = self.parse_unary()?;

        while self.match_token(&Token::Power) {
            let right = self.parse_unary()?;

            expression = Expression::BinaryOp { left: Box::new(expression), operator: Operator::Power, right: Box::new(right) }
        }

        Ok(expression)
    }

    fn parse_unary(&mut self) -> io::Result<Expression> {
        if self.match_token(&Token::Minus) || self.match_token(&Token::Not) {
            let operator = match self.previous() {
                Token::Minus => Operator::Minus,
                Token::Not => Operator::Not,
                _ => unreachable!()
            };
            
            let expression = self.parse_front_unary()?;
            
            Ok(
                Expression::UnaryOp {
                    expression: Box::new(expression),
                    operator: operator
                }
            )
        } else {
            self.parse_front_unary()
        }
    }

    fn parse_front_unary(&mut self) -> io::Result<Expression> {
        let left = self.parse_identifier_index()?;
        
        if self.match_token(&Token::PlusPlus) 
            || self.match_token(&Token::MinusMinus)
            || self.match_token(&Token::Not) {

            let operator = match self.previous() {
                Token::PlusPlus => Operator::PlusPlus,
                Token::MinusMinus => Operator::MinusMinus,
                Token::Not => Operator::Clone,
                _ => unreachable!()
            };
            
            Ok(
                Expression::FrontUnaryOp {
                    expression: Box::new(left),
                    operator: operator
                }
            )
        } else {
            Ok(left)
        }
    }

    fn parse_identifier_index(&mut self) -> io::Result<Expression> {
        let left = self.parse_tuple_index()?;

        if self.match_token(&Token::LeftSquareBracket) {
            let mut index: Option<Expression> = None;
            
            loop {
                let value = self.parse_expression()?;
                
                if let Some(i) = index {
                    index = Some(Expression::ListIndex { left: Box::new(i), index: Box::new(value) }); 
                } else {
                    index = Some(Expression::ListIndex { left: Box::new(left.clone()), index: Box::new(value)});
                }
                
                self.consume_token(Token::RightSquareBracket)?;
                
                if !self.match_token(&Token::LeftSquareBracket) {
                    break;
                }
            }

            Ok(index.unwrap())
        } else {
            Ok(left)
        }
    }

    fn parse_tuple_index(&mut self) -> io::Result<Expression> {
        let left = self.parse_primary()?;

        if self.match_token(&Token::Dot) {
            let mut indeces: Vec<i32> = Vec::new();

            loop {
                if let Token::IntegerLiteral(index) = self.peek() {
                    indeces.push(*index);
                    self.advance();
                }

                if !self.match_token(&Token::Dot) {
                    break;
                }
            }

            Ok(Expression::TupleIndex { left: Box::new(left), indeces: indeces })

        } else {
            Ok(left)
        }
    }

    fn parse_primary(&mut self) -> io::Result<Expression> {
        match self.advance() {
            Some(Token::True) => Ok(Expression::BooleanLiteral(true)),
            Some(Token::False) => Ok(Expression::BooleanLiteral(true)),
            Some(Token::Function) => self.parse_expression_function(),
            Some(Token::Null) => Ok(Expression::NullLiteral),
            Some(Token::IntegerLiteral(value)) => Ok(Expression::IntegerLiteral(value)),
            Some(Token::FloatLiteral(value)) => Ok(Expression::FloatLiteral(value)),
            Some(Token::StringLiteral(value)) => Ok(Expression::StringLiteral(value)),

            Some(Token::LeftParenthesis) => {
                let expression = self.parse_expression()?;
                
                match self.advance() {
                    Some(Token::RightParenthesis) => {
                        Ok(expression)
                    },

                    Some(Token::Comma) => {
                        let mut expressions: Vec<Expression> = vec![expression];

                        loop {
                            let expression = self.parse_expression()?;

                            expressions.push(expression);

                            if !self.match_token(&Token::Comma) {
                                break;
                            }
                        }

                        self.consume_token(Token::RightParenthesis)?;

                        Ok(Expression::TupleLiteral(expressions))
                    }

                    _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Expected closed expression with right parenthesis."))
                }
            },
            Some(Token::LeftSquareBracket) => {
                let mut expressions: Vec<Expression> = Vec::new();

                loop {
                    let expression = self.parse_expression()?;
                    expressions.push(expression);

                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                }

                self.consume_token(Token::RightSquareBracket)?;

                Ok(Expression::ListLiteral(expressions))
            },
            Some(Token::Identifier(name)) => {
                let name = name.to_owned();
                if self.match_token(&Token::LeftParenthesis) {
                    if self.match_token(&Token::RightParenthesis) {
                        return Ok(Expression::FunctionCall(name, Vec::new()));
                    }

                    let parameters = self.parse_function_call_parameters()?;
                    self.consume_token(Token::RightParenthesis)?;

                    Ok(Expression::FunctionCall(name, parameters))
                } else {
                    Ok(Expression::Identifier(name))
                }
            },
            None => Err(io::Error::new(io::ErrorKind::InvalidData, "Expected expression got nothing!")),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, format!("Expected expression got {}", self.peek())))
        }
    }

    fn parse_expression_function(&mut self) -> io::Result<Expression> {
        self.consume_token(Token::LeftParenthesis)?;

        self.semantic_analyzer.enter_function_enviroment();

        let parameters = self.parse_parameters()?;

        let return_type = if self.match_token(&Token::Colon) {
            let data_type = self.parse_data_type()?;

            data_type
        } else {
            DataType::void()
        };

        self.consume_token(Token::LeftBrace)?;

        let function_data_type = DataType::Function { 
            parameters: DataType::from_parameters(&parameters), 
            return_type: Box::new(return_type.clone())
        };
        
        self.function_context = Context::Function{ return_data: function_data_type.clone() };
        let block = self.parse_block_statement()?;
        self.function_context = Context::None;

        self.semantic_analyzer.exit_function_enviroment()?;

        Ok(Expression::FunctionLiteral {
            parameters,
            return_type,
            block
        })
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_end() {
            return false;
        }
        
        self.peek() == token
    }

    fn parse_data_type(&mut self) -> io::Result<DataType> {        
        match self.advance() {
            Some(Token::Int) => Ok(DataType::Int),
            Some(Token::Float) => Ok(DataType::Float),
            Some(Token::String) => Ok(DataType::String),
            Some(Token::Bool) => Ok(DataType::Bool),
            Some(Token::Function) => {
                self.consume_token(Token::LeftParenthesis)?;
                
                let mut parameters: Vec<DataType> = Vec::new();

                if !self.match_token(&Token::RightParenthesis) {
                    loop {
                        let data_type = self.parse_data_type()?;
                        parameters.push(data_type);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }

                    self.consume_token(Token::RightParenthesis)?;
                }

                let return_type = if self.match_token(&Token::Colon) {
                    self.parse_data_type()?
                } else {
                    DataType::void()
                };

                Ok(DataType::Function { parameters: parameters, return_type: Box::new(return_type) })
            }
            Some(Token::LeftSquareBracket) => {
                let data_type = self.parse_data_type()?;
                self.consume_token(Token::RightSquareBracket)?;

                Ok(DataType::List(Box::new(data_type)))
            },
            Some(Token::LeftParenthesis) => {
                let mut data_types: Vec<DataType> = Vec::new();

                loop {
                    let data_type = self.parse_data_type()?;

                    data_types.push(data_type);

                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                }

                self.consume_token(Token::RightParenthesis)?;

                Ok(DataType::Tuple(data_types))
            }
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Cannot parse the data type!"))
        }
    }

    fn consume_token(&mut self, token: Token) -> io::Result<Token> {
        if self.check(&token) {
            

            Ok(self.advance().unwrap())
        } else {

            if self.is_end() {
                Err(io::Error::new(io::ErrorKind::InvalidData, format!("Expected token: {:?} got nothing!", token)))
            } else {
                Err(io::Error::new(io::ErrorKind::InvalidData, format!("Expected token: {:?} got {:?}!", token, self.peek())))
            }
        }
    }

    fn consume_identifier(&mut self) -> io::Result<String> {
        let token= self.advance();

        if let Some(Token::Identifier(name)) = token {            
            Ok(name.to_string())
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Expected token identefier!"))
        }
    }

    fn advance(&mut self) -> Option<Token> {
        if self.is_end() {
            return None;
        }

        if self.current_token < self.tokens.len() {
            let token = self.peek().clone();
            self.current_token += 1;
            Some(token)
        } else {
            None
        }
    }

    fn back(&mut self) {
        self.current_token -= 1;
    }

    fn is_end(&self) -> bool {
        self.current_token >= self.tokens.len()
    }

    fn match_token(&mut self, token: &Token) -> bool {
        if self.is_end() {
            return false;
        }

        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current_token - 1]
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current_token]
    }

    fn peek_pos(&self) -> &TokenPos { // Big boss
        if self.is_end() {
            &self.token_pos[self.token_pos.len() - 1]
        } else {
            &self.token_pos[self.current_token]
        }
    }
}
