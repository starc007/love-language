use std::collections::HashMap;
use crate::shared_types::{BinaryOp, Type, Value};
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

      fn check_type(&self, value: &Value, expected: Type) -> Result<(), LoveError> {
        let actual = value.get_type();
        if actual != expected {
            Err(LoveError::Type(format!(
                "Expected {}, but found {}",
                expected,
                value
            )))
        } else {
            Ok(())
        }
    }

    fn check_binary_operands(
        &self,
        left: &Value,
        right: &Value,
        _operator: &BinaryOp,
        expected_type: Type
    ) -> Result<(), LoveError> {
        self.check_type(left, expected_type.clone())?;
        self.check_type(right, expected_type.clone())?;
        Ok(())
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
            Ast::FunctionDecl { name, params, body, .. } => {
                let param_names = params.into_iter().map(|(name, _)| name).collect();
                let function = Value::Function {
                    name: name.clone(),
                    params: param_names,
                    body,
                };
                self.environment.define(name, function.clone());
                Ok(function)
            }
            Ast::Call { callee, arguments } => {
                let function = self.environment.get(&callee)
                    .ok_or_else(|| LoveError::Runtime(format!("Undefined function '{}'", callee)))?
                    .clone();

                match function {
                    Value::Function { params, body, .. } => {
                        // Create new environment for function scope
                        let mut new_env = Environment::new();

                        // Evaluate and bind arguments
                        if params.len() != arguments.len() {
                            return Err(LoveError::Runtime(format!(
                                "Expected {} arguments but got {}.",
                                params.len(),
                                arguments.len()
                            )));
                        }

                        for (param, arg) in params.iter().zip(arguments) {
                            let value = self.interpret(arg)?;
                            new_env.define(param.clone(), value);
                        }

                        // Store current environment and set new one
                        let old_env = std::mem::replace(&mut self.environment, new_env);

                        // Execute function body
                        let mut result = Value::Null;
                        for stmt in body {
                            result = self.interpret(stmt)?;
                        }

                        // Restore old environment
                        self.environment = old_env;

                        Ok(result)
                    }
                    _ => Err(LoveError::Runtime(format!("'{}' is not a function", callee))),
                }
            }
            Ast::ReturnStmt(value) => {
                match value {
                    Some(expr) => self.interpret(*expr),
                    None => Ok(Value::Null),
                }
            }
            Ast::VariableDecl { name, initializer, .. } => {
                let value = self.interpret(*initializer)?;
                self.environment.define(name, value.clone());
                Ok(value)
            }
              Ast::If { condition, then_branch, else_branch } => {
                // Evaluate the condition
                let cond_value = self.interpret(*condition)?;
                
                match cond_value {
                    Value::Boolean(true) => {
                        // Execute then branch
                        let mut last_value = Value::Null;
                        for stmt in then_branch {
                            last_value = self.interpret(stmt)?;
                        }
                        Ok(last_value)
                    },
                    Value::Boolean(false) => {
                        // Execute else branch if it exists
                        if let Some(else_stmts) = else_branch {
                            let mut last_value = Value::Null;
                            for stmt in else_stmts {
                                last_value = self.interpret(stmt)?;
                            }
                            Ok(last_value)
                        } else {
                            Ok(Value::Null)
                        }
                    },
                    _ => Err(LoveError::Runtime(
                        "Condition must evaluate to a feeling (yes/no)".to_string()
                    )),
                }
            },
            Ast::Binary { left, operator, right } => {
                let left_val = self.interpret(*left)?;
                let right_val = self.interpret(*right)?;
                
                 match operator {
                    BinaryOp::Add | BinaryOp::Subtract | 
                    BinaryOp::Multiply | BinaryOp::Divide => {
                        self.check_binary_operands(&left_val, &right_val, &operator, Type::Number)?;
                        
                        match (left_val, operator, right_val) {
                            (Value::Number(a), BinaryOp::Add, Value::Number(b)) => 
                                Ok(Value::Number(a + b)),
                            (Value::Number(a), BinaryOp::Subtract, Value::Number(b)) => 
                                Ok(Value::Number(a - b)),
                            (Value::Number(a), BinaryOp::Multiply, Value::Number(b)) => 
                                Ok(Value::Number(a * b)),
                            (Value::Number(a), BinaryOp::Divide, Value::Number(b)) => {
                                if b == 0.0 {
                                    Err(LoveError::Runtime("Cannot split by zero!".to_string()))
                                } else {
                                    Ok(Value::Number(a / b))
                                }
                            }
                            _ => Err(LoveError::Runtime("Invalid operation".to_string())),
                        }
                    },
                    BinaryOp::Greater | BinaryOp::Less | 
                    BinaryOp::GreaterEqual | BinaryOp::LessEqual => {
                        self.check_binary_operands(&left_val, &right_val, &operator, Type::Number)?;
                        
                        match (left_val, operator, right_val) {
                            (Value::Number(a), BinaryOp::Greater, Value::Number(b)) => 
                                Ok(Value::Boolean(a > b)),
                            (Value::Number(a), BinaryOp::Less, Value::Number(b)) => 
                                Ok(Value::Boolean(a < b)),
                            (Value::Number(a), BinaryOp::GreaterEqual, Value::Number(b)) => 
                                Ok(Value::Boolean(a >= b)),
                            (Value::Number(a), BinaryOp::LessEqual, Value::Number(b)) => 
                                Ok(Value::Boolean(a <= b)),
                            _ => Err(LoveError::Runtime("Invalid comparison".to_string())),
                        }
                    },
                    _ => Err(LoveError::Runtime("Operation not implemented".to_string())),
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
            Ast::Block(statements) => {
                let mut result = Value::Null;
                // Create new environment for block scope
                let new_env = Environment::new();
                let old_env = std::mem::replace(&mut self.environment, new_env);

                for stmt in statements {
                    result = self.interpret(stmt)?;
                }

                // Restore old environment
                self.environment = old_env;
                Ok(result)
            }
            _ => Err(LoveError::Runtime("Not implemented".to_string())),
        }
    }
}