use rustyline::Editor;
use colored::*;
use std::io::{self};

mod shared_types;
mod lexer;
mod parser;
mod interpreter;
mod error;

use crate::shared_types::Value;
use crate::error::LoveError;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() -> io::Result<()> {
    print_welcome_message();

    let mut rl = Editor::<()>::new();
    let mut interpreter = interpreter::Interpreter::new();
    let mut current_line = String::new();
    let mut brace_count = 0;

    loop {
        let prompt = if brace_count > 0 { "...  " } else { "love> " };
        
        match rl.readline(prompt) {
            Ok(line) => {
                let trimmed_line = line.trim();
                
                // Check for exit command
                if trimmed_line.eq_ignore_ascii_case("breakup;") {
                    println!("{}", "Goodbye! Our love story ends here... 💔".bright_red());
                    break;
                }

                // Update brace count
                brace_count += count_braces(trimmed_line);
                
                // Append current line
                current_line.push_str(&line);
                current_line.push('\n');

                // Only check for semicolon if we're not in a block and not in a function declaration
                if brace_count == 0 && !trimmed_line.is_empty() && 
                   !trimmed_line.ends_with(';') && !trimmed_line.ends_with('{') && 
                   !trimmed_line.ends_with('}') && !current_line.contains("devotion") {
                    println!("{} Syntax error: Missing semicolon at end of statement", "💔".bright_red());
                    current_line.clear();
                    continue;
                }

                // If we have balanced braces and the line ends with semicolon or closing brace, execute
                if brace_count == 0 && (trimmed_line.ends_with(';') || trimmed_line.ends_with('}')) {
                    // Validate syntax before execution
                    match validate_syntax(&current_line) {
                        Ok(_) => {
                            rl.add_history_entry(current_line.as_str());
                            
                            match execute_line(&current_line, &mut interpreter) {
                                Ok(value) => match value {
                                    Value::Null => (),
                                    _ => println!("{} {:?}", "💝".bright_green(), value),
                                },
                                Err(e) => println!("{} {}", "💔".bright_red(), format_error(&e).bright_red()),
                            }
                        }
                        Err(e) => println!("{} {}", "💔".bright_red(), format_error(&e).bright_red()),
                    }
                    
                    current_line.clear();
                } else if brace_count < 0 {
                    println!("{} Syntax error: Unmatched closing brace", "💔".bright_red());
                    current_line.clear();
                    brace_count = 0;
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

fn validate_syntax(input: &str) -> Result<(), LoveError> {
    // First check lexical structure
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;
    
    // Then check parsing
    let mut parser = Parser::new(tokens);
    parser.parse()?;
    
    Ok(())
}

fn count_braces(line: &str) -> i32 {
    let mut count = 0;
    for c in line.chars() {
        match c {
            '{' => count += 1,
            '}' => count -= 1,
            _ => (),
        }
    }
    count
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
    println!("   heart x match 10;");
    println!("   whisper x;");
    println!("   devotion add(x: number, y: number) -> number {{");
    println!("       promise x cuddle y;");
    println!("   }}");
    println!("{}", "\n💔 Type 'breakup;' to exit\n".bright_red());
}

fn execute_line(
    line: &str, 
    interpreter: &mut interpreter::Interpreter
) -> Result<Value, LoveError> {
    // Skip empty lines
    if line.trim().is_empty() {
        return Ok(Value::Null);
    }

    // Lexical analysis
    let mut lexer = lexer::Lexer::new(line);
    let tokens = lexer.tokenize()?;

    // Parsing
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse()?;

    // Interpretation
    interpreter.interpret(ast)
}

fn format_error(error: &LoveError) -> String {
    match error {
        LoveError::Lexer(msg) => format!("Lexer error: {}", msg),
        LoveError::Parser(msg) => format!("Parser error: {}", msg),
        LoveError::Runtime(msg) => format!("Runtime error: {}", msg),
        LoveError::Type(msg) => format!("Type error: {}", msg),
    }
}