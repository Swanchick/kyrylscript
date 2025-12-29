use super::data_type::DataType;
use crate::lexer::token::Token;
use crate::lexer::token_pos::TokenPos;

use super::context::Context;
use super::expression::Expression;
use super::identifier_tail::IdentifierTail;
use super::operator::Operator;
use super::parameter::Parameter;
use super::semantic_analyzer::SemanticAnalyzer;
use super::statement::Statement;

use std::collections::HashMap;
use std::io;

pub struct Parser {
    tokens: Vec<Token>,
    token_pos: Vec<TokenPos>,
    semantic_analyzer: SemanticAnalyzer,
    current_token: usize,
    function_context: Context,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, token_pos: Vec<TokenPos>) -> Parser {
        Parser {
            tokens,
            token_pos,
            semantic_analyzer: SemanticAnalyzer::new(),
            current_token: 0,
            function_context: Context::None,
        }
    }

    pub fn with_semantic_analyzer(
        tokens: Vec<Token>,
        token_pos: Vec<TokenPos>,
        semantic_analyzer: SemanticAnalyzer,
    ) -> Parser {
        Parser {
            tokens,
            token_pos,
            semantic_analyzer: semantic_analyzer,
            current_token: 0,
            function_context: Context::None,
        }
    }

    pub fn start(&mut self) -> io::Result<Vec<Statement>> {
        let result = self.parse_block_statement();

        match result {
            Ok(statements) => Ok(statements),

            Err(e) => {
                let pos = self.peek_pos();

                let file = match pos.get_source() {
                    Some(path) => path,
                    None => "Main",
                };

                let error = format!(
                    "kyryl-script: At {}:{}: {}",
                    file,
                    pos.get_line() + 1,
                    e.to_string()
                );

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

        self.semantic_analyzer
            .save_variable(name.clone(), data_type.clone());

        let parameter = Parameter {
            name: name,
            data_type: data_type,
        };

        Ok(parameter)
    }

    pub fn parse_statement(&mut self) -> io::Result<Option<Statement>> {
        let public = self.match_token(&Token::Pub);

        if let Context::Function { return_data: _ } = self.function_context {
            if public {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid context for public visibility!",
                ));
            }
        }

        match self.advance() {
            Some(Token::Let) => Ok(Some(self.parse_variable_declaration_statement(public)?)),
            Some(Token::Function) => Ok(Some(self.parse_function(public)?)),
            Some(Token::Return) => Ok(Some(self.parse_return_statement()?)),
            Some(Token::If) => Ok(Some(self.parse_if_statement()?)),
            Some(Token::While) => Ok(Some(self.parse_while_statement()?)),
            Some(Token::For) => Ok(Some(self.parse_for_statement()?)),
            None => Ok(None),
            _ => {
                self.back();
                let backup_token = self.current_token;
                let segments = self.parse_identifier_tail()?;

                if segments.is_empty() {
                    self.current_token = backup_token;
                    let value = self.parse_expression()?;
                    self.consume_token(Token::Semicolon)?;
                    return Ok(Some(Statement::Expression { value }));
                }

                self.parse_assignment(backup_token, segments)
            }
        }
    }

    fn parse_assignment(
        &mut self,
        backup_token: usize,
        segments: Vec<IdentifierTail>,
    ) -> io::Result<Option<Statement>> {
        match self.advance() {
            Some(Token::Equal) => {
                let statement = self.parse_assignment_statement(&segments)?;
                Ok(Some(statement))
            }
            Some(Token::PlusEqual) => {
                let statement = self.parse_add_value_statment(&segments)?;
                Ok(Some(statement))
            }
            Some(Token::MinusEqual) => todo!(),
            Some(Token::Question) => todo!(),
            _ => {
                self.current_token = backup_token;
                let value = self.parse_expression()?;
                self.consume_token(Token::Semicolon)?;
                Ok(Some(Statement::Expression { value }))
            }
        }
    }

    fn parse_identifier_tail(&mut self) -> io::Result<Vec<IdentifierTail>> {
        let mut segments: Vec<IdentifierTail> = Vec::new();
        let mut segment_data_type: Option<DataType> = None;

        loop {
            match self.advance() {
                Some(Token::Identifier(name)) => {
                    if segments.is_empty() {
                        let current_data_type = self.semantic_analyzer.get_variable(&name)?;
                        segment_data_type = Some(current_data_type);
                        segments.push(IdentifierTail::Name(name));
                        continue;
                    }
                }
                Some(Token::Dot) => {
                    let name = self.consume_identifier()?;

                    if let Some(DataType::Module(module_data_type)) = &segment_data_type {
                        if let Some(current_data_type) = module_data_type.get(&name) {
                            segment_data_type = Some(current_data_type.clone());
                            segments.push(IdentifierTail::Name(name));
                        } else {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("No field in module with name {}!", name),
                            ));
                        }
                    } else {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "This is not a module!",
                        ));
                    }
                }
                Some(Token::LeftSquareBracket) => {
                    let index = self.parse_expression()?;
                    let index_data_type = self.semantic_analyzer.get_data_type(&index)?;

                    if !(matches!(segment_data_type, Some(DataType::List(_)))
                        || matches!(segment_data_type, Some(DataType::String)))
                    {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Cannot access element from non list value!",
                        ));
                    }

                    if index_data_type != DataType::Int {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Expected type integer in indexing the value in list!",
                        ));
                    }

                    self.consume_token(Token::RightSquareBracket)?;
                    segments.push(IdentifierTail::Index(index));
                }
                Some(Token::Arrow) => {
                    if !matches!(segment_data_type, Some(DataType::Tuple(_))) {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Cannot access element from non tuple value!",
                        ));
                    }

                    if let Some(Token::IntegerLiteral(int)) = self.advance() {
                        segments.push(IdentifierTail::TupleIndex(int));
                    } else {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Invalid tuple indexing!",
                        ));
                    }
                }
                Some(Token::LeftParenthesis) => {
                    if self.match_token(&Token::RightParenthesis) {
                        segments.push(IdentifierTail::Call(Vec::new()));
                        continue;
                    }

                    let arguments = self.parse_function_call_parameters()?;

                    match &segment_data_type {
                        Some(DataType::Function {
                            parameters,
                            return_type: _,
                        }) => {
                            self.check_function_arguments(&arguments, parameters)?;
                        }
                        Some(DataType::RustFunction { return_type: _ }) => {}
                        data_type => {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("Expected a function to call, got {:?}", data_type),
                            ));
                        }
                    }

                    self.consume_token(Token::RightParenthesis)?;
                    segments.push(IdentifierTail::Call(arguments));
                }
                _ => {
                    self.back();
                    return Ok(segments);
                }
            }
        }
    }

    pub fn parse_function(&mut self, public: bool) -> io::Result<Statement> {
        let function_name = self.consume_identifier()?;

        self.consume_token(Token::LeftParenthesis)?;

        let parameters = self.parse_parameters()?;

        let function_type = if self.match_token(&Token::Colon) {
            self.parse_data_type()?
        } else {
            DataType::void()
        };

        self.consume_token(Token::LeftBrace)?;

        let function_data_type = DataType::Function {
            parameters: DataType::from_parameters(&parameters),
            return_type: Box::new(function_type.clone()),
        };

        self.function_context = Context::Function {
            return_data: function_data_type.clone(),
        };

        if public {
            self.semantic_analyzer
                .global_save_variable(function_name.clone(), function_data_type);
        } else {
            self.semantic_analyzer
                .save_variable(function_name.clone(), function_data_type);
        }

        self.semantic_analyzer.enter_function_enviroment();

        let body = self.parse_block_statement()?;

        self.function_context = Context::None;
        self.semantic_analyzer.exit_function_enviroment()?;

        Ok(Statement::Function {
            name: function_name,
            public,
            return_type: function_type,
            parameters,
            body,
        })
    }

    fn parse_for_statement(&mut self) -> io::Result<Statement> {
        let name = self.consume_identifier()?;

        self.consume_token(Token::In)?;
        let expression = self.parse_expression()?;
        let data_type = self.semantic_analyzer.get_data_type(&expression)?;

        self.semantic_analyzer.enter_function_enviroment();

        match data_type {
            DataType::List(child_data_type) => self
                .semantic_analyzer
                .save_variable(name.clone(), *child_data_type),
            DataType::String => self
                .semantic_analyzer
                .save_variable(name.clone(), DataType::String),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "For loop statement mismatch type!",
                ));
            }
        }

        self.consume_token(Token::LeftBrace)?;
        let body = self.parse_block_statement()?;

        self.semantic_analyzer.exit_function_enviroment()?;

        Ok(Statement::ForLoopStatement {
            name: name,
            list: expression,
            body: body,
        })
    }

    fn parse_add_value_statment(
        &mut self,
        segments: &Vec<IdentifierTail>,
    ) -> io::Result<Statement> {
        let identifier_type = self
            .semantic_analyzer
            .get_data_type_from_segments(segments)?;

        let expression = self.parse_expression()?;
        let data_type = self.semantic_analyzer.get_data_type(&expression)?;

        if !(data_type == DataType::Float
            || data_type == DataType::Int
            || data_type == DataType::String)
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid data type for add assignment!",
            ));
        }

        if identifier_type == data_type {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Add assignment value mismatch!",
            ));
        }

        self.consume_token(Token::Semicolon)?;

        Ok(Statement::AddValue {
            segments: segments.clone(),
            value: expression,
        })
    }

    fn parse_remove_value_statement(
        &mut self,
        segments: &Vec<IdentifierTail>,
    ) -> io::Result<Statement> {
        let identifier_type = self
            .semantic_analyzer
            .get_data_type_from_segments(segments)?;

        let expression = self.parse_expression()?;
        let data_type = self.semantic_analyzer.get_data_type(&expression)?;

        if !(data_type == DataType::Float || data_type == DataType::Int) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid data type for add assignment!",
            ));
        }

        if identifier_type == data_type {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Add assignment value mismatch!",
            ));
        }

        self.consume_token(Token::Semicolon)?;

        Ok(Statement::RemoveValue {
            segments: segments.clone(),
            value: expression,
        })
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
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Different data types in expression and actual data type.",
                ));
            }
        }

        if public {
            self.semantic_analyzer
                .global_save_variable(name.clone(), dt.clone());
        } else {
            self.semantic_analyzer
                .save_variable(name.clone(), dt.clone());
        }

        self.consume_token(Token::Semicolon)?;

        Ok(Statement::VariableDeclaration {
            name,
            public,
            data_type,
            value: Some(expression),
        })
    }

    fn parse_return_statement(&mut self) -> io::Result<Statement> {
        if let Context::Function { return_data } = self.function_context.clone() {
            let expression = self.parse_expression()?;
            let data_type = self.semantic_analyzer.get_data_type(&expression)?;

            if let DataType::Function {
                parameters: _,
                return_type,
            } = return_data
            {
                if *return_type != data_type {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Mismatch return and function return types!",
                    ));
                }

                self.consume_token(Token::Semicolon)?;
                return Ok(Statement::ReturnStatement {
                    value: Some(expression),
                });
            }
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "No function context for return!",
        ))
    }

    fn parse_assignment_statement(
        &mut self,
        segments: &Vec<IdentifierTail>,
    ) -> io::Result<Statement> {
        let identifier_type = self
            .semantic_analyzer
            .get_data_type_from_segments(&segments)?;

        let expression = self.parse_expression()?;
        let data_type = self.semantic_analyzer.get_data_type(&expression)?;

        match identifier_type {
            DataType::Void(Some(null_type)) => {
                if *null_type != data_type && !DataType::is_void(&data_type) {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Assignment value mismatch!",
                    ));
                }
            }

            _ => {
                if identifier_type != data_type && !DataType::is_void(&data_type) {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Assignment value mismatch!",
                    ));
                }
            }
        }

        self.consume_token(Token::Semicolon)?;

        Ok(Statement::Assignment {
            segments: segments.clone(),
            value: expression,
        })
    }

    fn parse_function_call_parameters(&mut self) -> io::Result<Vec<Expression>> {
        let mut args: Vec<Expression> = Vec::new();

        loop {
            let expression = self.parse_expression()?;
            args.push(expression);

            if !self.match_token(&Token::Comma) {
                break;
            }
        }

        Ok(args)
    }

    fn check_function_arguments(
        &self,
        args: &[Expression],
        parameters: &[DataType],
    ) -> io::Result<()> {
        if parameters.len() != args.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Argument mismatch!",
            ));
        }

        for (function_arg_data_type, arg_expresiion) in parameters.iter().zip(args) {
            let arg_data_type = self.semantic_analyzer.get_data_type(arg_expresiion)?;

            if function_arg_data_type != &arg_data_type {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Argument mismatch, expected {:?} got {:?}",
                        function_arg_data_type, arg_data_type
                    ),
                ));
            }
        }

        Ok(())
    }

    fn parse_if_statement(&mut self) -> io::Result<Statement> {
        let condition = self.parse_expression()?;

        let statment_data_type = self.semantic_analyzer.get_data_type(&condition)?;
        if statment_data_type != DataType::Bool {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "If statment condition mismatch data_type, expected bool!",
            ));
        }

        self.consume_token(Token::LeftBrace)?;

        self.semantic_analyzer.enter_function_enviroment();
        let if_body = self.parse_block_statement()?;
        self.semantic_analyzer.exit_function_enviroment()?;

        let else_block = if self.match_token(&Token::Else) {
            self.consume_token(Token::LeftBrace)?;

            self.semantic_analyzer.enter_function_enviroment();
            let result = self.parse_block_statement()?;
            self.semantic_analyzer.exit_function_enviroment()?;

            Some(result)
        } else {
            None
        };

        Ok(Statement::IfStatement {
            condition: condition,
            body: if_body,
            else_body: else_block,
        })
    }

    fn parse_while_statement(&mut self) -> io::Result<Statement> {
        let condition = self.parse_expression()?;

        let condition_data_type = self.semantic_analyzer.get_data_type(&condition)?;
        if condition_data_type != DataType::Bool {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "While statment condition mismatch data_type, expected bool!",
            ));
        }

        self.consume_token(Token::LeftBrace)?;

        self.semantic_analyzer.enter_function_enviroment();
        let block = self.parse_block_statement()?;
        self.semantic_analyzer.exit_function_enviroment()?;

        Ok(Statement::WhileStatement {
            condition: condition,
            body: block,
        })
    }

    pub fn parse_expression(&mut self) -> io::Result<Expression> {
        self.parse_logic_or()
    }

    fn parse_logic_or(&mut self) -> io::Result<Expression> {
        let mut expression = self.parse_logic_and()?;

        while self.match_token(&Token::Or) {
            let right = self.parse_logic_and()?;

            expression = Expression::BinaryOp {
                left: Box::new(expression),
                operator: Operator::Or,
                right: Box::new(right),
            }
        }

        Ok(expression)
    }

    fn parse_logic_and(&mut self) -> io::Result<Expression> {
        let mut expression = self.parse_comparison()?;

        while self.match_token(&Token::And) {
            let right = self.parse_comparison()?;

            expression = Expression::BinaryOp {
                left: Box::new(expression),
                operator: Operator::And,
                right: Box::new(right),
            }
        }

        Ok(expression)
    }

    fn parse_comparison(&mut self) -> io::Result<Expression> {
        let mut expression = self.parse_addition()?;

        while self.match_token(&Token::EqualEqual)
            || self.match_token(&Token::NotEqual)
            || self.match_token(&Token::GreaterEqual)
            || self.match_token(&Token::LessEqual)
            || self.match_token(&Token::GreaterThan)
            || self.match_token(&Token::LessThan)
        {
            let operator = match self.previous() {
                Token::EqualEqual => Operator::EqualEqual,
                Token::NotEqual => Operator::NotEqual,
                Token::GreaterEqual => Operator::GreaterEqual,
                Token::GreaterThan => Operator::Greater,
                Token::LessEqual => Operator::LessEqual,
                Token::LessThan => Operator::Less,
                _ => unreachable!(),
            };

            let right = self.parse_addition()?;

            expression = Expression::BinaryOp {
                left: Box::new(expression),
                operator: operator,
                right: Box::new(right),
            }
        }

        Ok(expression)
    }

    fn parse_addition(&mut self) -> io::Result<Expression> {
        let mut expression = self.parse_multiplication()?;

        while self.match_token(&Token::Plus) || self.match_token(&Token::Minus) {
            let operator = match self.previous() {
                Token::Plus => Operator::Plus,
                Token::Minus => Operator::Minus,
                _ => unreachable!(),
            };

            let right = self.parse_multiplication()?;

            expression = Expression::BinaryOp {
                left: Box::new(expression),
                operator: operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    fn parse_multiplication(&mut self) -> io::Result<Expression> {
        let mut expression = self.parse_power()?;

        while self.match_token(&Token::Multiply) || self.match_token(&Token::Divide) {
            let operator = match self.previous() {
                Token::Multiply => Operator::Multiply,
                Token::Divide => Operator::Divide,
                _ => unreachable!(),
            };

            let right = self.parse_power()?;

            expression = Expression::BinaryOp {
                left: Box::new(expression),
                operator: operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    fn parse_power(&mut self) -> io::Result<Expression> {
        let mut expression = self.parse_unary()?;

        while self.match_token(&Token::Power) {
            let right = self.parse_unary()?;

            expression = Expression::BinaryOp {
                left: Box::new(expression),
                operator: Operator::Power,
                right: Box::new(right),
            }
        }

        Ok(expression)
    }

    fn parse_unary(&mut self) -> io::Result<Expression> {
        if self.match_token(&Token::Minus) || self.match_token(&Token::Not) {
            let operator = match self.previous() {
                Token::Minus => Operator::Minus,
                Token::Not => Operator::Not,
                _ => unreachable!(),
            };

            let expression = self.parse_front_unary()?;

            Ok(Expression::UnaryOp {
                expression: Box::new(expression),
                operator: operator,
            })
        } else {
            self.parse_front_unary()
        }
    }

    fn parse_front_unary(&mut self) -> io::Result<Expression> {
        let left = self.parse_primary()?;

        if self.match_token(&Token::PlusPlus)
            || self.match_token(&Token::MinusMinus)
            || self.match_token(&Token::Not)
        {
            let operator = match self.previous() {
                Token::PlusPlus => Operator::PlusPlus,
                Token::MinusMinus => Operator::MinusMinus,
                Token::Not => Operator::Clone,
                _ => unreachable!(),
            };

            Ok(Expression::FrontUnaryOp {
                expression: Box::new(left),
                operator: operator,
            })
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
                    Some(Token::RightParenthesis) => Ok(expression),

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

                    _ => Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Expected closed expression with right parenthesis.",
                    )),
                }
            }
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
            }
            Some(Token::Identifier(_)) => {
                self.back();
                let identifier_tail = self.parse_identifier_tail()?;
                Ok(Expression::Identifier(identifier_tail))
            }
            Some(Token::LeftBrace) => self.parse_module(),
            None => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Expected expression got nothing!",
            )),
            token => {
                if let Some(token) = token {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Expected expression got {}", token),
                    ))
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Expected expression got nothing!",
                    ))
                }
            }
        }
    }

    fn parse_module(&mut self) -> io::Result<Expression> {
        let mut module: HashMap<String, Expression> = HashMap::new();

        loop {
            let field_name: String = self.consume_identifier()?;
            match self.advance() {
                Some(Token::Colon) => {
                    let expression = self.parse_expression()?;
                    module.insert(field_name, expression);
                }
                Some(Token::LeftParenthesis) => {
                    self.semantic_analyzer.enter_function_enviroment();
                    let parameters = self.parse_parameters()?;

                    let return_type = if self.match_token(&Token::Colon) {
                        self.parse_data_type()?
                    } else {
                        DataType::void()
                    };

                    self.consume_token(Token::LeftBrace)?;

                    let function_data_type = DataType::Function {
                        parameters: DataType::from_parameters(&parameters),
                        return_type: Box::new(return_type.clone()),
                    };

                    self.function_context = Context::Function {
                        return_data: function_data_type,
                    };
                    let block = self.parse_block_statement()?;
                    self.function_context = Context::None;

                    self.semantic_analyzer.exit_function_enviroment()?;

                    module.insert(
                        field_name,
                        Expression::FunctionLiteral {
                            parameters,
                            return_type,
                            block,
                        },
                    );
                }
                _ => {}
            }

            if !self.match_token(&Token::Comma) {
                break;
            }
        }

        self.consume_token(Token::RightBrace)?;

        Ok(Expression::Module(module))
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
            return_type: Box::new(return_type.clone()),
        };

        self.function_context = Context::Function {
            return_data: function_data_type.clone(),
        };
        let block = self.parse_block_statement()?;
        self.function_context = Context::None;

        self.semantic_analyzer.exit_function_enviroment()?;

        Ok(Expression::FunctionLiteral {
            parameters,
            return_type,
            block,
        })
    }

    fn check(&self, token: &Token) -> bool {
        if let Some(original_token) = self.tokens.get(self.current_token) {
            original_token == token
        } else {
            false
        }
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

                Ok(DataType::Function {
                    parameters: parameters,
                    return_type: Box::new(return_type),
                })
            }
            Some(Token::LeftSquareBracket) => {
                let data_type = self.parse_data_type()?;
                self.consume_token(Token::RightSquareBracket)?;

                Ok(DataType::List(Box::new(data_type)))
            }
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
            Some(Token::LeftBrace) => {
                let mut module: HashMap<String, DataType> = HashMap::new();
                loop {
                    let field_name = self.consume_identifier()?;
                    self.consume_token(Token::Colon)?;

                    let data_type = self.parse_data_type()?;
                    module.insert(field_name, data_type);

                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                }
                self.consume_token(Token::RightBrace)?;
                Ok(DataType::Module(module))
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Cannot parse the data type!",
            )),
        }
    }

    fn consume_token(&mut self, token: Token) -> io::Result<()> {
        if self.check(&token) {
            self.advance();
            Ok(())
        } else {
            if self.is_end() {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Expected token: {:?} got nothing!", token),
                ))
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Expected token: {:?} got {:?}!", token, self.peek()),
                ))
            }
        }
    }

    fn consume_identifier(&mut self) -> io::Result<String> {
        let token = self.advance();

        if let Some(Token::Identifier(name)) = token {
            Ok(name.to_string())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Expected token identefier!",
            ))
        }
    }

    fn advance(&mut self) -> Option<Token> {
        if let Some(token) = self.tokens.get(self.current_token) {
            let token = token.clone();
            self.current_token += 1;
            return Some(token);
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

    fn peek_pos(&self) -> &TokenPos {
        // Big boss
        if self.is_end() {
            &self.token_pos[self.token_pos.len() - 1]
        } else {
            &self.token_pos[self.current_token]
        }
    }
}
