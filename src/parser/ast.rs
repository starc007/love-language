use crate::shared_types::{Value, BinaryOp, Type};

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Program(Vec<Ast>),
    
    VariableDecl {
        name: String,
        is_constant: bool,
        initializer: Box<Ast>,
    },
    
    FunctionDecl {
        name: String,
        params: Vec<(String, Type)>,
        return_type: Option<Type>,
        body: Vec<Ast>,
    },
    
    If {
        condition: Box<Ast>,
        then_branch: Vec<Ast>,
        else_branch: Option<Vec<Ast>>,
    },
    
    While {
        condition: Box<Ast>,
        body: Vec<Ast>,
    },
    
    Block(Vec<Ast>),
    
    ExpressionStmt(Box<Ast>),
    
    PrintStmt(Box<Ast>),
    
    ReturnStmt(Option<Box<Ast>>),
    
    Binary {
        left: Box<Ast>,
        operator: BinaryOp,
        right: Box<Ast>,
    },
    
    Unary {
        operator: BinaryOp,
        operand: Box<Ast>,
    },
    
    
    Assign {
        name: String,
        value: Box<Ast>,
    },
    
    Variable(String),
    
    Literal(Value),
    
    Grouping(Box<Ast>),
}