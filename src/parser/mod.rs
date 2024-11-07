// parser/mod.rs
use crate::error::LoveError;
use crate::lexer::Token;
use crate::shared_types::{BinaryOp, Type, Value};
use ast::Ast;

pub mod ast;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Ast, LoveError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(Ast::Program(statements))
    }

    fn declaration(&mut self) -> Result<Ast, LoveError> {
        match self.peek() {
            Some(Token::Heart) | Some(Token::Forever) => self.var_declaration(),
            Some(Token::Devotion) => self.function_declaration(),
            _ => self.statement(),
        }
    }

    fn var_declaration(&mut self) -> Result<Ast, LoveError> {
        let is_constant = matches!(self.peek(), Some(Token::Forever));
        self.advance(); // consume heart/forever

        let name = match self.advance() {
            Some(Token::Identifier(name)) => name.clone(),
            _ => return Err(LoveError::Parser("Expected variable name".to_string())),
        };

        // Handle optional type annotation
        let var_type = if matches!(self.peek(), Some(Token::Colon)) {
            self.advance(); // consume :
            Some(self.parse_type()?)
        } else {
            None
        };

        self.consume(&Token::Match, "Expected 'match' after variable name")?;
        let initializer = self.expression()?;
        self.consume(&Token::Semicolon, "Expected ';' after variable declaration")?;

        Ok(Ast::VariableDecl {
            name,
            is_constant,
            initializer: Box::new(initializer),
        })
    }

    fn statement(&mut self) -> Result<Ast, LoveError> {
        match self.peek() {
            Some(Token::Whisper) => self.print_statement(),
            Some(Token::Crush) => self.if_statement(),
            Some(Token::Dating) => self.while_statement(),
            Some(Token::Promise) => self.return_statement(),
            Some(Token::LBrace) => Ok(Ast::Block(self.block()?)),
            _ => self.expression_statement(),
        }
    }

    fn print_statement(&mut self) -> Result<Ast, LoveError> {
        self.advance(); // consume 'whisper'
        let value = self.expression()?;
        self.consume(&Token::Semicolon, "Expected ';' after value")?;
        Ok(Ast::PrintStmt(Box::new(value)))
    }

    fn if_statement(&mut self) -> Result<Ast, LoveError> {
        self.advance(); // consume 'crush'
        self.consume(&Token::LParen, "Expected '(' after 'crush'")?;
        let condition = self.expression()?;
        self.consume(&Token::RParen, "Expected ')' after condition")?;

        let then_branch = self.block()?;
        
        let else_branch = if matches!(self.peek(), Some(Token::Butterflies)) {
            self.advance(); // consume 'butterflies'
            Some(self.block()?)
        } else {
            None
        };

        Ok(Ast::If {
            condition: Box::new(condition),
            then_branch,
            else_branch,
        })
    }

    fn while_statement(&mut self) -> Result<Ast, LoveError> {
        self.advance(); // consume 'dating'
        self.consume(&Token::LParen, "Expected '(' after 'dating'")?;
        let condition = self.expression()?;
        self.consume(&Token::RParen, "Expected ')' after condition")?;

        let body = self.block()?;

        Ok(Ast::While {
            condition: Box::new(condition),
            body,
        })
    }

    fn expression_statement(&mut self) -> Result<Ast, LoveError> {
        let expr = self.expression()?;
        self.consume(&Token::Semicolon, "Expected ';' after expression")?;
        Ok(Ast::ExpressionStmt(Box::new(expr)))
    }

    fn expression(&mut self) -> Result<Ast, LoveError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Ast, LoveError> {
        let expr = self.or()?;

        if matches!(self.peek(), Some(Token::Match)) {
            self.advance(); // consume 'match'
            let value = self.assignment()?;

            match expr {
                Ast::Variable(name) => {
                    return Ok(Ast::Assign {
                        name,
                        value: Box::new(value),
                    });
                }
                _ => return Err(LoveError::Parser("Invalid assignment target".to_string())),
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Ast, LoveError> {
        let mut expr = self.and()?;

        while matches!(self.peek(), Some(Token::Or)) {
            self.advance(); // consume 'or'
            let right = self.and()?;
            expr = Ast::Binary {
                left: Box::new(expr),
                operator: BinaryOp::Or,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Ast, LoveError> {
        let mut expr = self.equality()?;

        while matches!(self.peek(), Some(Token::And)) {
            self.advance(); // consume 'and'
            let right = self.equality()?;
            expr = Ast::Binary {
                left: Box::new(expr),
                operator: BinaryOp::And,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Ast, LoveError> {
        let mut expr = self.comparison()?;

        while let Some(token) = self.peek() {
            let op = match token {
                Token::Soulmate => BinaryOp::Equal,
                Token::NotEqual => BinaryOp::NotEqual,
                _ => break,
            };
            self.advance();
            let right = self.comparison()?;
            expr = Ast::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Ast, LoveError> {
        let mut expr = self.term()?;

        while let Some(token) = self.peek() {
            let op = match token {
                Token::LessThan => BinaryOp::Less,
                Token::GreaterThan => BinaryOp::Greater,
                Token::LessThanEqual => BinaryOp::LessEqual,
                Token::GreaterThanEqual => BinaryOp::GreaterEqual,
                _ => break,
            };
            self.advance();
            let right = self.term()?;
            expr = Ast::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Ast, LoveError> {
        let mut expr = self.factor()?;

        while let Some(token) = self.peek() {
            let op = match token {
                Token::Cuddle => BinaryOp::Add,
                Token::Breakup => BinaryOp::Subtract,
                _ => break,
            };
            self.advance();
            let right = self.factor()?;
            expr = Ast::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Ast, LoveError> {
        let mut expr = self.unary()?;

        while let Some(token) = self.peek() {
            let op = match token {
                Token::Kiss => BinaryOp::Multiply,
                Token::Split => BinaryOp::Divide,
                _ => break,
            };
            self.advance();
            let right = self.unary()?;
            expr = Ast::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Ast, LoveError> {
        if matches!(self.peek(), Some(Token::Not)) {
            self.advance();
            let right = self.unary()?;
            return Ok(Ast::Unary {
                operator: BinaryOp::Not,
                operand: Box::new(right),
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Ast, LoveError> {
        match self.advance() {
            Some(Token::Number(n)) => Ok(Ast::Literal(Value::Number(*n as f64))),
            Some(Token::Text(s)) => Ok(Ast::Literal(Value::Text(s.clone()))),
            Some(Token::Yes) => Ok(Ast::Literal(Value::Boolean(true))),
            Some(Token::No) => Ok(Ast::Literal(Value::Boolean(false))),
            Some(Token::LParen) => {
                let expr = self.expression()?;
                self.consume(&Token::RParen, "Expected ')' after expression")?;
                Ok(Ast::Grouping(Box::new(expr)))
            }
            Some(Token::Identifier(name)) => Ok(Ast::Variable(name.clone())),
            _ => Err(LoveError::Parser("Expected expression".to_string())),
        }
    }

    fn block(&mut self) -> Result<Vec<Ast>, LoveError> {
        self.consume(&Token::LBrace, "Expected '{' before block")?;
        let mut statements = Vec::new();

        while !matches!(self.peek(), Some(Token::RBrace) | None) {
            statements.push(self.declaration()?);
        }

        self.consume(&Token::RBrace, "Expected '}' after block")?;
        Ok(statements)
    }

    fn parse_type(&mut self) -> Result<Type, LoveError> {
        match self.advance() {
            Some(Token::TypeNumber) => Ok(Type::Number),
            Some(Token::TypeText) => Ok(Type::Text),
            Some(Token::TypeFeeling) => Ok(Type::Boolean),
            _ => Err(LoveError::Parser("Expected type".to_string())),
        }
    }

    // Helper methods
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.tokens.get(self.current - 1)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn consume(&mut self, expected: &Token, message: &str) -> Result<&Token, LoveError> {
        if let Some(token) = self.peek() {
            if token == expected {
                Ok(self.advance().unwrap())
            } else {
                Err(LoveError::Parser(message.to_string()))
            }
        } else {
            Err(LoveError::Parser(message.to_string()))
        }
    }

    fn return_statement(&mut self) -> Result<Ast, LoveError> {
        self.advance(); // consume 'promise'
        
        // Check if there's a return value
        let value = if !matches!(self.peek(), Some(Token::Semicolon)) {
            Some(Box::new(self.expression()?))
        } else {
            None
        };
        
        self.consume(&Token::Semicolon, "Expected ';' after return value")?;
        Ok(Ast::ReturnStmt(value))
    }

    fn function_declaration(&mut self) -> Result<Ast, LoveError> {
        self.advance(); // consume 'devotion'
        let name = match self.advance() {
            Some(Token::Identifier(name)) => name.clone(),
            _ => return Err(LoveError::Parser("Expected function name".to_string())),
        };

        self.consume(&Token::LParen, "Expected '(' after function name")?;
        
        // Parse parameters
        let mut params = Vec::new();
        if !matches!(self.peek(), Some(Token::RParen)) {
            loop {
                let param_name = match self.advance() {
                    Some(Token::Identifier(name)) => name.clone(),
                    _ => return Err(LoveError::Parser("Expected parameter name".to_string())),
                };

                self.consume(&Token::Colon, "Expected ':' after parameter name")?;
                let param_type = self.parse_type()?;
                params.push((param_name, param_type));

                if !matches!(self.peek(), Some(Token::Comma)) {
                    break;
                }
                self.advance(); // consume comma
            }
        }
        
        self.consume(&Token::RParen, "Expected ')' after parameters")?;

        // Parse optional return type
        let return_type = if matches!(self.peek(), Some(Token::Arrow)) {
            self.advance(); // consume ->
            Some(self.parse_type()?)
        } else {
            None
        };

        // Parse function body
        let body = self.block()?;

        Ok(Ast::FunctionDecl {
            name,
            params,
            return_type,
            body,
        })
    }
}