use std::io::{self, Write};
use rustyline::Editor;
use colored::*;

mod lexer;
mod parser;
mod interpreter;
mod error;
mod shared_types;

use crate::shared_types::Value;

fn main() -> io::Result<()> {
    print_welcome_message();
    
    let mut rl = Editor::<()>::new();
    let mut interpreter = interpreter::Interpreter::new();

    loop {
        print!("💕 love> ");
        io::stdout().flush()?;

        let readline = rl.readline("");
        match readline {
            Ok(line) => {
                if line.trim().is_empty() {
                    continue;
                }

                // Check for exit command
                if line.trim().eq_ignore_ascii_case("breakup;") {
                    println!("{}", "Goodbye! Our love story ends here... 💔".bright_red());
                    break;
                }

                // Add line to history
                rl.add_history_entry(line.as_str());

                // Execute the line
                match execute_line(&line, &mut interpreter) {
                    Ok(value) => match value {
                        Value::Null => (), // Don't print anything for null values
                        _ => println!("{} {:?}", "💝".bright_green(), value),
                    },
                    Err(e) => println!("{} {}", "💔".bright_red(), e.to_string().bright_red()),
                }
            }
            Err(err) => {
                println!("{} Error: {}", "💔".bright_red(), err.to_string().bright_red());
                break;
            }
        }
    }

    Ok(())
}

fn print_welcome_message() {
    println!("{}", r#"
    ╭──────────────────────────────────╮
    │     Welcome to Love Language     │
    │      Let's write with love!      │
    ╰──────────────────────────────────╯
    "#.bright_magenta());
    
    println!("{}", "💕 Type your love expressions...\n".bright_cyan());
    println!("{}", "💘 Example commands:".bright_yellow());
    println!("   heart x match 10;          // Declare variable");
    println!("   forever y match 20;        // Declare constant");
    println!("   whisper (x cuddle y);      // Print x + y");
    println!("   x match y kiss 2;          // Multiply\n");
    println!("{}", "💔 Type 'breakup;' to exit\n".bright_red());
}

fn execute_line(
    line: &str, 
    interpreter: &mut interpreter::Interpreter
) -> Result<Value, error::LoveError> {
    // Skip empty lines
    if line.trim().is_empty() {
        return Ok(Value::Null);
    }

    // Lexical analysis
    let mut lexer = lexer::Lexer::new(line);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => {
            return Err(error::LoveError::Lexer(format!("Lexer error: {}", e)));
        }
    };

    // Parsing
    let mut parser = parser::Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            return Err(error::LoveError::Parser(format!("Parser error: {}", e)));
        }
    };

    // Interpretation
    match interpreter.interpret(ast) {
        Ok(value) => Ok(value),
        Err(e) => Err(error::LoveError::Runtime(format!("Runtime error: {}", e))),
    }
}

// Helper function to format error messages
fn format_error(error: &error::LoveError) -> String {
    match error {
        error::LoveError::Lexer(msg) => format!("Lexical Error: {}", msg),
        error::LoveError::Parser(msg) => format!("Syntax Error: {}", msg),
        error::LoveError::Runtime(msg) => format!("Runtime Error: {}", msg),
        error::LoveError::Type(msg) => format!("Type Error: {}", msg),
    }
}
