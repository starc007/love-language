use logos::Logos;
use crate::error::LoveError;
pub use token::Token;

mod token;

#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LoveError> {
        let mut tokens = Vec::new();
        let mut lexer = Token::lexer(self.source);
        
        while let Some(token) = lexer.next() {
            // Update position information
            let span = lexer.span();
            let slice = &self.source[..span.start];
            let lines = slice.matches('\n').count();
            self.line += lines;
            if lines > 0 {
                self.column = span.start - slice.rfind('\n').unwrap_or(0);
            } else {
                self.column += span.end - span.start;
            }

            // Handle the token
            match token {
                Token::Error => {
                    return Err(LoveError::Lexer(format!(
                        "Invalid token at line {}, column {}",
                        self.line,
                        self.column
                    )))
                }
                token => tokens.push(token),
            }
        }

        Ok(tokens)
    }

}
