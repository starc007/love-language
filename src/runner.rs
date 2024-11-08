use std::fs;
use std::path::Path;
use colored::*;

use crate::error::LoveError;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::fun::*;

pub struct Runner {
    interpreter: Interpreter,
}

impl Runner {
    pub fn new() -> Self {
        Runner {
            interpreter: Interpreter::new(),
        }
    }

    pub fn run_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), LoveError> {
        let path = path.as_ref();
        
        // Check file extension
        if let Some(extension) = path.extension() {
            if extension != "love" {
                return Err(LoveError::Runtime(
                    "Only .love files can contain our love story! 💝".to_string()
                ));
            }
        } else {
            return Err(LoveError::Runtime(
                "File must have a .love extension! 💝".to_string()
            ));
        }

        // Read file content
        let content = fs::read_to_string(path)
            .map_err(|e| LoveError::Runtime(format!("Failed to read love letter: {}", e)))?;

        println!("{}", create_love_border(
            &format!("💌 Reading love story from: {}", path.display())
        ).bright_cyan());

        // Print the content being executed
        println!("{}", "💝 Love story output:".bright_yellow());
        println!();

        // Execute the code
        let mut lexer = Lexer::new(&content);
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                println!("{}", create_love_border(
                    &format!("💔 Lexer error:\n{}", format_error(&e))
                ).bright_red());
                return Err(e);
            }
        };

        let mut parser = Parser::new(tokens);
        let ast = match parser.parse() {
            Ok(a) => a,
            Err(e) => {
                println!("{}", create_love_border(
                    &format!("💔 Parser error:\n{}", format_error(&e))
                ).bright_red());
                return Err(e);
            }
        };

        match self.interpreter.interpret(ast) {
            Ok(_) => {
                println!("{}", create_love_border(
                    &format!("{} Love story executed successfully!", get_random_emoji())
                ).bright_green());
                Ok(())
            },
            Err(e) => {
                println!("{}", create_love_border(
                    &format!("💔 Runtime error:\n{}", format_error(&e))
                ).bright_red());
                Err(e)
            }
        }
    }
}

fn format_error(error: &LoveError) -> String {
    match error {
        LoveError::Lexer(msg) => format!("{}\n{}", get_random_error_message(), msg),
        LoveError::Parser(msg) => format!("{}\n{}", get_random_error_message(), msg),
        LoveError::Runtime(msg) => format!("{}\n{}", get_random_error_message(), msg),
        LoveError::Type(msg) => format!("{}\n{}", get_random_error_message(), msg),
    }
}