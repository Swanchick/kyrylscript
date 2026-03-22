use super::data_type::DataType;
use crate::lexer::token::Token;
use crate::lexer::token_pos::TokenPos;

use ks_global::utils::ks_error::KsError;
use ks_global::utils::ks_result::KsResult;

use super::context::Context;
use super::expression::Expression;
use super::identifier_tail::IdentifierTail;
use super::operator::Operator;
use super::parameter::Parameter;
use super::semantic_analyzer::SemanticAnalyzer;
use super::statement::Statement;

use std::collections::BTreeMap;

pub struct Parser {
    tokens: Vec<Token>,
    token_pos: Vec<TokenPos>,
    semantic_analyzer: SemanticAnalyzer,
    current_token: usize,
    function_context: Vec<Context>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            tokens: Vec::new(),
            token_pos: Vec::new(),
            semantic_analyzer: SemanticAnalyzer::new(),
            current_token: 0,
            function_context: vec![Context::process()],
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
            function_context: vec![Context::process()],
        }
    }

    pub fn set_tokens(&mut self, tokens: Vec<Token>, token_pos: Vec<TokenPos>) {
        self.tokens = tokens;
        self.token_pos = token_pos;
    }

    pub fn function_context(&self) -> &[Context] {
        &self.function_context
    }

    fn last_function_context(&self) -> KsResult<&Context> {
        if let Some(context) = self.function_context.last() {
            Ok(context)
        } else {
            Err(KsError::parse("Cannot get the last function context"))
        }
    }

    fn last_function_context_mut(&mut self) -> KsResult<&mut Context> {
        if let Some(context) = self.function_context.last_mut() {
            Ok(context)
        } else {
            Err(KsError::parse("Cannot get the last function context"))
        }
    }

    pub fn start(&mut self) -> KsResult<Vec<Statement>> {
        let result = self.parse_block_statement();

        let statements = match result {
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
                    e.message(),
                );

                Err(KsError::parse(&error))
            }
        }?;

        Ok(statements)
    }

    pub fn register_variable(&mut self, name: &str, data_type: DataType, public: bool) {
        let semantic_analyzer = &mut self.semantic_analyzer;

        if public {
            semantic_analyzer.global_save_variable(name.to_string(), data_type);
        } else {
            semantic_analyzer.save_variable(name.to_string(), data_type);
        }
    }

    pub fn get_semantic_analyzer(&self) -> &SemanticAnalyzer {
        &self.semantic_analyzer
    }

    pub fn parse_block_statement(&mut self) -> KsResult<Vec<Statement>> {
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

    fn parse_parameters(&mut self) -> KsResult<Vec<Parameter>> {
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

    fn parse_parameter(&mut self) -> KsResult<Parameter> {
        let name = self.consume_identifier()?;
        let data_type = self.parse_data_type()?;

        let parameter = Parameter {
            name: name,
            data_type: data_type,
        };

        Ok(parameter)
    }

    pub fn parse_statement(&mut self) -> KsResult<Option<Statement>> {
        let public = self.match_token(&Token::Pub);

        if self.function_context.len() > 1 && public {
            return Err(KsError::parse("Invalid context for public visibility!"));
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
    ) -> KsResult<Option<Statement>> {
        match self.advance() {
            Some(Token::Equal) => {
                let statement = self.parse_assignment_statement(&segments)?;
                Ok(Some(statement))
            }
            Some(Token::PlusEqual) => {
                let statement = self.parse_add_value_statement(&segments)?;
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

    fn parse_identifier_tail(&mut self) -> KsResult<Vec<IdentifierTail>> {
        let mut segments: Vec<IdentifierTail> = Vec::new();
        let mut segment_data_type: Option<DataType> = None;

        loop {
            match self.advance() {
                Some(Token::Identifier(name)) => {
                    if segments.is_empty() {
                        let current_data_type = self.semantic_analyzer.get_variable(&name)?;
                        segment_data_type = Some(current_data_type);
                        segments.push(IdentifierTail::Name(name.clone()));

                        let context = self.last_function_context_mut()?;

                        if !context.variables.contains(&name) {
                            context.captured_variables.push(name);
                        }

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
                            return Err(KsError::parse(&format!(
                                "No field in module with name {}!",
                                name
                            )));
                        }
                    } else {
                        return Err(KsError::parse("This is not a module!"));
                    }
                }
                Some(Token::LeftSquareBracket) => {
                    let index = self.parse_expression()?;
                    let index_data_type = self.semantic_analyzer.get_data_type(&index)?;

                    if !(matches!(segment_data_type, Some(DataType::List(_)))
                        || matches!(segment_data_type, Some(DataType::String)))
                    {
                        return Err(KsError::parse("Cannot access element from non list value!"));
                    }

                    if index_data_type != DataType::Int {
                        return Err(KsError::parse(
                            "Expected type integer in indexing the value in list!",
                        ));
                    }

                    self.consume_token(Token::RightSquareBracket)?;
                    segments.push(IdentifierTail::Index(index));
                }
                Some(Token::Arrow) => {
                    if !matches!(segment_data_type, Some(DataType::Tuple(_))) {
                        return Err(KsError::parse(
                            "Cannot access element from non tuple value!",
                        ));
                    }

                    if let Some(Token::IntegerLiteral(int)) = self.advance() {
                        segments.push(IdentifierTail::TupleIndex(int));
                    } else {
                        return Err(KsError::parse("Invalid tuple indexing!"));
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
                            return Err(KsError::parse(&format!(
                                "Expected a function to call, got {:?}",
                                data_type
                            )));
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

    fn enter_function(&mut self, return_type: DataType) {
        self.function_context.push(Context::new(return_type));
    }

    fn exit_function(&mut self) -> KsResult<Context> {
        if let Some(context) = self.function_context.pop() {
            Ok(context)
        } else {
            Err(KsError::parse("No Function to exit!"))
        }
    }

    pub fn parse_function(&mut self, public: bool) -> KsResult<Statement> {
        let function_name = self.consume_identifier()?;

        self.consume_token(Token::LeftParenthesis)?;

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

        self.enter_function(return_type.clone());

        if public {
            self.semantic_analyzer
                .global_save_variable(function_name.clone(), function_data_type);
        } else {
            self.semantic_analyzer
                .save_variable(function_name.clone(), function_data_type);
        }

        self.semantic_analyzer.enter_function_environment();

        for parameter in &parameters {
            self.semantic_analyzer
                .save_variable(parameter.name.clone(), parameter.data_type.clone());

            let context = self.last_function_context_mut()?;
            context.variables.push(parameter.name.clone());
        }

        let body = self.parse_block_statement()?;
        let mut context = self.exit_function()?;
        let last_context = self.last_function_context_mut()?;
        last_context.variables.push(function_name.clone());

        for capture in &mut context.captured_variables {
            if last_context.variables.contains(&capture) {
                continue;
            }

            last_context.captured_variables.push(capture.clone());
        }

        self.semantic_analyzer.exit_function_environment()?;

        Ok(Statement::Function {
            name: function_name,
            public,
            return_type,
            parameters,
            body,
            captured: context.captured_variables,
        })
    }

    fn parse_for_statement(&mut self) -> KsResult<Statement> {
        let name = self.consume_identifier()?;

        self.consume_token(Token::In)?;
        let expression = self.parse_expression()?;
        let data_type = self.semantic_analyzer.get_data_type(&expression)?;

        self.semantic_analyzer.enter_function_environment();

        match data_type {
            DataType::List(child_data_type) => self
                .semantic_analyzer
                .save_variable(name.clone(), *child_data_type),
            DataType::String => self
                .semantic_analyzer
                .save_variable(name.clone(), DataType::String),
            _ => {
                return Err(KsError::parse("For loop statement mismatch type!"));
            }
        }

        self.consume_token(Token::LeftBrace)?;
        let body = self.parse_block_statement()?;

        self.semantic_analyzer.exit_function_environment()?;

        Ok(Statement::ForLoopStatement {
            name: name,
            list: expression,
            body: body,
        })
    }

    fn parse_add_value_statement(&mut self, segments: &Vec<IdentifierTail>) -> KsResult<Statement> {
        let identifier_type = self
            .semantic_analyzer
            .get_data_type_from_segments(segments)?;

        let expression = self.parse_expression()?;
        let data_type = self.semantic_analyzer.get_data_type(&expression)?;

        if !(data_type == DataType::Float
            || data_type == DataType::Int
            || data_type == DataType::String)
        {
            return Err(KsError::parse("Invalid data type for add assignment!"));
        }

        if identifier_type == data_type {
            return Err(KsError::parse("Add assignment value mismatch!"));
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
    ) -> KsResult<Statement> {
        let identifier_type = self
            .semantic_analyzer
            .get_data_type_from_segments(segments)?;

        let expression = self.parse_expression()?;
        let data_type = self.semantic_analyzer.get_data_type(&expression)?;

        if !(data_type == DataType::Float || data_type == DataType::Int) {
            return Err(KsError::parse("Invalid data type for add assignment!"));
        }

        if identifier_type == data_type {
            return Err(KsError::parse("Add assignment value mismatch!"));
        }

        self.consume_token(Token::Semicolon)?;

        Ok(Statement::RemoveValue {
            segments: segments.clone(),
            value: expression,
        })
    }

    fn parse_variable_declaration_statement(&mut self, public: bool) -> KsResult<Statement> {
        let name = self.consume_identifier()?;

        let context = self.last_function_context_mut()?;
        context.variables.push(name.clone());

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
                return Err(KsError::parse(
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
            data_type: Some(dt),
            value: Some(expression),
        })
    }

    fn parse_return_statement(&mut self) -> KsResult<Statement> {
        if self.function_context.len() == 1 {
            return Err(KsError::parse("No function context to return!"));
        }

        let last = self.last_function_context()?;
        let return_type = last.return_type.clone();

        let expression = self.parse_expression()?;
        let data_type = self.semantic_analyzer.get_data_type(&expression)?;

        if return_type != data_type {
            return Err(KsError::parse("Mismatch return and function return types!"));
        }

        self.consume_token(Token::Semicolon)?;
        Ok(Statement::ReturnStatement {
            value: Some(expression),
        })
    }

    fn parse_assignment_statement(
        &mut self,
        segments: &Vec<IdentifierTail>,
    ) -> KsResult<Statement> {
        let identifier_type = self
            .semantic_analyzer
            .get_data_type_from_segments(&segments)?;

        let expression = self.parse_expression()?;
        let data_type = self.semantic_analyzer.get_data_type(&expression)?;

        match identifier_type {
            DataType::Void(Some(null_type)) => {
                if *null_type != data_type && !DataType::is_void(&data_type) {
                    return Err(KsError::parse("Assignment value mismatch!"));
                }
            }

            _ => {
                if identifier_type != data_type && !DataType::is_void(&data_type) {
                    return Err(KsError::parse("Assignment value mismatch!"));
                }
            }
        }

        self.consume_token(Token::Semicolon)?;

        Ok(Statement::Assignment {
            segments: segments.clone(),
            value: expression,
        })
    }

    fn parse_function_call_parameters(&mut self) -> KsResult<Vec<Expression>> {
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
    ) -> KsResult<()> {
        if parameters.len() != args.len() {
            return Err(KsError::parse("Argument mismatch!"));
        }

        for (function_arg_data_type, arg_expresiion) in parameters.iter().zip(args) {
            let arg_data_type = self.semantic_analyzer.get_data_type(arg_expresiion)?;

            if function_arg_data_type != &arg_data_type {
                return Err(KsError::parse(&format!(
                    "Argument mismatch, expected {:?} got {:?}",
                    function_arg_data_type, arg_data_type
                )));
            }
        }

        Ok(())
    }

    fn parse_if_statement(&mut self) -> KsResult<Statement> {
        let condition = self.parse_expression()?;

        let statement_data_type = self.semantic_analyzer.get_data_type(&condition)?;
        if statement_data_type != DataType::Bool {
            return Err(KsError::parse(
                "If statement condition mismatch data_type, expected bool!",
            ));
        }

        self.consume_token(Token::LeftBrace)?;

        self.semantic_analyzer.enter_function_environment();
        let if_body = self.parse_block_statement()?;
        self.semantic_analyzer.exit_function_environment()?;

        let else_block = if self.match_token(&Token::Else) {
            self.consume_token(Token::LeftBrace)?;

            self.semantic_analyzer.enter_function_environment();
            let result = self.parse_block_statement()?;
            self.semantic_analyzer.exit_function_environment()?;

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

    fn parse_while_statement(&mut self) -> KsResult<Statement> {
        let condition = self.parse_expression()?;

        let condition_data_type = self.semantic_analyzer.get_data_type(&condition)?;
        if condition_data_type != DataType::Bool {
            return Err(KsError::parse(
                "While statement condition mismatch data_type, expected bool!",
            ));
        }

        self.consume_token(Token::LeftBrace)?;

        self.semantic_analyzer.enter_function_environment();
        let block = self.parse_block_statement()?;
        self.semantic_analyzer.exit_function_environment()?;

        Ok(Statement::WhileStatement {
            condition: condition,
            body: block,
        })
    }

    pub fn parse_expression(&mut self) -> KsResult<Expression> {
        self.parse_logic_or()
    }

    fn parse_logic_or(&mut self) -> KsResult<Expression> {
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

    fn parse_logic_and(&mut self) -> KsResult<Expression> {
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

    fn parse_comparison(&mut self) -> KsResult<Expression> {
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

    fn parse_addition(&mut self) -> KsResult<Expression> {
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

    fn parse_multiplication(&mut self) -> KsResult<Expression> {
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

    fn parse_power(&mut self) -> KsResult<Expression> {
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

    fn parse_unary(&mut self) -> KsResult<Expression> {
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

    fn parse_front_unary(&mut self) -> KsResult<Expression> {
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

    fn parse_primary(&mut self) -> KsResult<Expression> {
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

                    _ => Err(KsError::parse(
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
            None => Err(KsError::parse("Expected expression got nothing!")),
            token => {
                if let Some(token) = token {
                    Err(KsError::parse(&format!(
                        "Expected expression got {}",
                        token
                    )))
                } else {
                    Err(KsError::parse("Expected expression got nothing!"))
                }
            }
        }
    }

    fn parse_module(&mut self) -> KsResult<Expression> {
        let mut module: BTreeMap<String, Expression> = BTreeMap::new();

        loop {
            let field_name: String = self.consume_identifier()?;
            match self.advance() {
                Some(Token::Colon) => {
                    let expression = self.parse_expression()?;
                    module.insert(field_name, expression);
                }
                Some(Token::LeftParenthesis) => {
                    self.semantic_analyzer.enter_function_environment();
                    let parameters = self.parse_parameters()?;

                    let return_type = if self.match_token(&Token::Colon) {
                        self.parse_data_type()?
                    } else {
                        DataType::void()
                    };

                    self.consume_token(Token::LeftBrace)?;
                    self.enter_function(return_type.clone());

                    for parameter in &parameters {
                        self.semantic_analyzer
                            .save_variable(parameter.name.clone(), parameter.data_type.clone());

                        let context = self.last_function_context_mut()?;
                        context.variables.push(parameter.name.clone());
                    }

                    let block = self.parse_block_statement()?;

                    let mut context = self.exit_function()?;
                    let last_context = self.last_function_context_mut()?;

                    for capture in &mut context.captured_variables {
                        if last_context.variables.contains(&capture) {
                            continue;
                        }

                        last_context.captured_variables.push(capture.clone());
                    }

                    self.semantic_analyzer.exit_function_environment()?;

                    module.insert(
                        field_name,
                        Expression::FunctionLiteral {
                            parameters,
                            return_type,
                            block,
                            captured: context.captured_variables,
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

    fn parse_expression_function(&mut self) -> KsResult<Expression> {
        self.consume_token(Token::LeftParenthesis)?;
        self.semantic_analyzer.enter_function_environment();

        let parameters = self.parse_parameters()?;

        let return_type = if self.match_token(&Token::Colon) {
            let data_type = self.parse_data_type()?;

            data_type
        } else {
            DataType::void()
        };

        self.consume_token(Token::LeftBrace)?;
        self.enter_function(return_type.clone());

        for parameter in &parameters {
            self.semantic_analyzer
                .save_variable(parameter.name.clone(), parameter.data_type.clone());

            let context = self.last_function_context_mut()?;
            context.variables.push(parameter.name.clone());
        }

        let block = self.parse_block_statement()?;

        let mut context = self.exit_function()?;
        let last_context = self.last_function_context_mut()?;

        for capture in &mut context.captured_variables {
            if last_context.variables.contains(&capture) {
                continue;
            }

            last_context.captured_variables.push(capture.clone());
        }

        self.semantic_analyzer.exit_function_environment()?;

        Ok(Expression::FunctionLiteral {
            parameters,
            return_type,
            block,
            captured: context.captured_variables,
        })
    }

    fn check(&self, token: &Token) -> bool {
        if let Some(original_token) = self.tokens.get(self.current_token) {
            original_token == token
        } else {
            false
        }
    }

    fn parse_data_type(&mut self) -> KsResult<DataType> {
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
                let mut module: BTreeMap<String, DataType> = BTreeMap::new();
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
            _ => Err(KsError::parse("Cannot parse the data type!")),
        }
    }

    fn consume_token(&mut self, token: Token) -> KsResult<()> {
        if self.check(&token) {
            self.advance();
            Ok(())
        } else {
            if self.is_end() {
                Err(KsError::parse(&format!(
                    "Expected token: {:?} got nothing!",
                    token
                )))
            } else {
                Err(KsError::parse(&format!(
                    "Expected token: {:?} got {:?}!",
                    token,
                    self.peek()
                )))
            }
        }
    }

    fn consume_identifier(&mut self) -> KsResult<String> {
        let token = self.advance();

        if let Some(Token::Identifier(name)) = token {
            Ok(name.to_string())
        } else {
            Err(KsError::parse("Expected token identifier!"))
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
