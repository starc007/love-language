use std::collections::HashMap;
use crate::shared_types::{Value, BinaryOp};
use crate::parser::ast::Ast;
use crate::error::LoveError;

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.values.get(name)
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), LoveError> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            Ok(())
        } else {
            Err(LoveError::Runtime(format!("Undefined variable '{}'.", name)))
        }
    }
}

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, ast: Ast) -> Result<Value, LoveError> {
        match ast {
            Ast::Program(statements) => {
                let mut result = Value::Null;
                for stmt in statements {
                    result = self.interpret(stmt)?;
                }
                Ok(result)
            }

            Ast::VariableDecl { name, initializer, .. } => {
                let value = self.interpret(*initializer)?;
                self.environment.define(name, value.clone());
                Ok(value)
            }

            Ast::Binary { left, operator, right } => {
                let left_val = self.interpret(*left)?;
                let right_val = self.interpret(*right)?;
                
                match (left_val, operator, right_val) {
                    (Value::Number(a), BinaryOp::Add, Value::Number(b)) => Ok(Value::Number(a + b)),
                    (Value::Number(a), BinaryOp::Subtract, Value::Number(b)) => Ok(Value::Number(a - b)),
                    (Value::Number(a), BinaryOp::Multiply, Value::Number(b)) => Ok(Value::Number(a * b)),
                    (Value::Number(a), BinaryOp::Divide, Value::Number(b)) => {
                        if b == 0.0 {
                            Err(LoveError::Runtime("Cannot split by zero!".to_string()))
                        } else {
                            Ok(Value::Number(a / b))
                        }
                    }
                    (Value::Text(a), BinaryOp::Add, Value::Text(b)) => Ok(Value::Text(a + &b)),
                    (Value::Number(a), BinaryOp::Less, Value::Number(b)) => Ok(Value::Boolean(a < b)),
                    (Value::Number(a), BinaryOp::Greater, Value::Number(b)) => Ok(Value::Boolean(a > b)),
                    (Value::Number(a), BinaryOp::Equal, Value::Number(b)) => Ok(Value::Boolean(a == b)),
                    _ => Err(LoveError::Runtime("Invalid operation".to_string())),
                }
            }

            Ast::PrintStmt(expr) => {
                let value = self.interpret(*expr)?;
                println!("{:?}", value);
                Ok(Value::Null)
            }

            Ast::Literal(value) => Ok(value),

            Ast::Variable(name) => {
                self.environment.get(&name)
                    .cloned()
                    .ok_or_else(|| LoveError::Runtime(format!("Undefined variable '{}'.", name)))
            }

            Ast::Assign { name, value } => {
                let evaluated_value = self.interpret(*value)?;
                self.environment.assign(&name, evaluated_value.clone())?;
                Ok(evaluated_value)
            }

            Ast::ExpressionStmt(expr) => self.interpret(*expr),

            Ast::Grouping(expr) => self.interpret(*expr),

            _ => Err(LoveError::Runtime("Not implemented".to_string())),
        }
    }
}