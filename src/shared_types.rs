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