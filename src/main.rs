use rustyline::Editor;
use colored::*;
use std::io::{self};
use std::env;

mod shared_types;
mod lexer;
mod parser;
mod interpreter;
mod error;
mod fun;
mod runner;

use crate::runner::Runner;
use crate::shared_types::Value;
use crate::error::LoveError;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::fun::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        1 => run_repl(),
        
        2 => run_file(&args[1]),
    
        _ => {
            println!("{}", create_love_border(
                "Usage: love-language [script.love]"
            ).bright_red());
            Ok(())
        }
    }
}

fn run_repl() -> io::Result<()> {
    print_welcome_message();

    let mut rl = Editor::<()>::new();
    let mut interpreter = interpreter::Interpreter::new();
    let mut current_line = String::new();
    let mut brace_count = 0;

    loop {
        let prompt = if brace_count > 0 { 
            format!("{}  ", get_random_emoji()) 
        } else { 
            format!("{}> ", get_random_emoji())
        };
        
        match rl.readline(&prompt) {
            Ok(line) => {
                let trimmed_line = line.trim();
                
                if handle_special_commands(trimmed_line) {
                    continue;
                }

                if trimmed_line.eq_ignore_ascii_case("breakup;") {
                    println!("{}", create_love_border("Goodbye! Our love story ends here...").bright_red());
                    break;
                }

                brace_count += count_braces(trimmed_line);
                
                current_line.push_str(&line);
                current_line.push('\n');

                if brace_count == 0 && !trimmed_line.is_empty() && 
                   !trimmed_line.ends_with(';') && !trimmed_line.ends_with('{') && 
                   !trimmed_line.ends_with('}') && !current_line.contains("devotion") {
                    println!("{} {}", "💔".bright_red(), format!("{}\nMissing semicolon at end of statement", get_random_error_message()));
                    current_line.clear();
                    continue;
                }

                if brace_count == 0 && (trimmed_line.ends_with(';') || trimmed_line.ends_with('}')) {
                    match validate_syntax(&current_line) {
                        Ok(_) => {
                            rl.add_history_entry(current_line.as_str());
                            
                            match execute_line(&current_line, &mut interpreter) {
                                Ok(value) => {
                                    match value {
                                        Value::Null => (),
                                        _ => {
                                            println!("{} {}", get_random_emoji(), get_random_success_message().bright_green());
                                            println!("{} {:?}", get_random_emoji(), value)
                                        },
                                    }
                                },
                                Err(e) => println!("{} {}", "💔".bright_red(), format_error(&e).bright_red()),
                            }
                        }
                        Err(e) => println!("{} {}", "💔".bright_red(), format_error(&e).bright_red()),
                    }
                    
                    current_line.clear();
                } else if brace_count < 0 {
                    println!("{} {}", "💔".bright_red(), 
                        format!("{}\nUnmatched closing brace", get_random_error_message()));
                    current_line.clear();
                    brace_count = 0;
                }
            }
            Err(err) => {
                println!("{} {}", "💔".bright_red(), format!("{}\n{}", 
                    get_random_error_message(), err.to_string()).bright_red());
                break;
            }
        }
    }

    Ok(())
}

fn run_file(path: &str) -> io::Result<()> {
    let mut runner = Runner::new();
    
    if let Err(e) = runner.run_file(path) {
        println!("{}", create_love_border(
            &format!("💔 Our love story encountered an error:\n{}", e)
        ).bright_red());
    }
    
    Ok(())
}

fn print_welcome_message() {
    println!("{}", r#"
╭━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━╮
│     Welcome to Love Language! 💝     │
│  Where Code and Romance Unite! 💕    │
╰━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━╯
"#.bright_magenta());
    
    println!("{}", "💌 Dear programmer, let's write a love story in code...".bright_cyan());
    println!("{}", "\n💘 Quick Reference:".bright_yellow());
    println!("   heart x match 10;         // Variables are matters of the heart");
    println!("   forever LOVE match 100;   // Some values last forever");
    println!("   whisper \"Hello Love!\";    // Share your feelings");
    println!("   x cuddle y                // Addition spreads love");
    println!("   x kiss y                  // Multiplication is magical");
    println!("\n💔 Type 'breakup;' when it's time to part ways...");
    println!("💡 Type 'love help' for more information\n");
}

fn execute_line(line: &str, interpreter: &mut interpreter::Interpreter) -> Result<Value, LoveError> {
    if line.trim().is_empty() {
        return Ok(Value::Null);
    }

    let mut lexer = lexer::Lexer::new(line);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;

    let result = interpreter.interpret(ast)?;
    
    // Add fun messages based on the code being executed
    if line.contains("devotion") {
        println!("{}", create_love_border("A new love function is born! 💘").bright_cyan());
    } else if line.contains("heart") {
        println!("{} {}", get_random_emoji(), "A new variable finds love!".bright_green());
    }
    
    Ok(result)
}

fn validate_syntax(input: &str) -> Result<(), LoveError> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;
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

fn format_error(error: &LoveError) -> String {
    match error {
        LoveError::Lexer(msg) => format!("{}\n{}", get_random_error_message(), msg),
        LoveError::Parser(msg) => format!("{}\n{}", get_random_error_message(), msg),
        LoveError::Runtime(msg) => format!("{}\n{}", get_random_error_message(), msg),
        LoveError::Type(msg) => format!("{}\n{}", get_random_error_message(), msg),
    }
}