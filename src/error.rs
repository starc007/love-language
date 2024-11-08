use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoveError {
    #[error("Lexer error: {0}")]
    Lexer(String),
    
    #[error("Parser error: {0}")]
    Parser(String),
    
    #[error("Runtime error: {0}")]
    Runtime(String),

    #[error("Type error: {0}")]
    Type(String),
}