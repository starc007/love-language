use std::fmt;
use crate::parser::ast::Ast;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Text(String),
    Boolean(bool),
     Function {
        name: String,
        params: Vec<String>,
        body: Vec<Ast>,
    },
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,           // cuddle
    Subtract,      // breakup
    Multiply,      // kiss
    Divide,        // split
    Equal,         // soulmate
    NotEqual,      // heartbreak
    Less,          // envies
    Greater,       // admires
    LessEqual,     // yearns
    GreaterEqual,  // adores
    And,           // and
    Or,            // or
    Not,           // not
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    Text,
    Boolean,
    Function,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Number => write!(f, "number"),
            Type::Text => write!(f, "text"),
            Type::Boolean => write!(f, "feeling"),
            Type::Function => write!(f, "devotion"),
        }
    }
}

// Add Display implementation for Value too for better error messages
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(_) => write!(f, "number"),
            Value::Text(_) => write!(f, "text"),
            Value::Boolean(_) => write!(f, "feeling"),
            Value::Function { .. } => write!(f, "devotion"),
            Value::Null => write!(f, "lonely"),
        }
    }
}

// Add method to get type of a value
impl Value {
    pub fn get_type(&self) -> Type {
        match self {
            Value::Number(_) => Type::Number,
            Value::Text(_) => Type::Text,
            Value::Boolean(_) => Type::Boolean,
            Value::Function { .. } => Type::Function,
            Value::Null => Type::Number, // You might want to handle null differently
        }
    }
}