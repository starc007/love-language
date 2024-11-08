use colored::*;
use rand::Rng;

// Love-themed emojis and messages
pub const LOVE_EMOJIS: &[&str] = &["💝", "💖", "💗", "💓", "💕", "💞", "💘", "💟", "❤️", "🫀", "😍", "🥰"];
pub const SUCCESS_MESSAGES: &[&str] = &[
    "Love is in the air!",
    "Your code is beautiful!",
    "What a lovely expression!",
    "You're coding with passion!",
    "Your heart flows through the code!",
    "Such romantic programming!",
    "Love at first byte!",
    "Your code makes my heart skip a beat!",
];

pub const ERROR_MESSAGES: &[&str] = &[
    "Oh no, heartbreak! 💔",
    "Love hurts sometimes... 😢",
    "Not all love stories work out... 💔",
    "Even Romeo and Juliet had bugs... 🥀",
    "Time to mend this broken heart... 💝",
    "Love is patient, love is kind, but this code needs fixing! 🔧",
];

pub fn get_random_emoji() -> String {
    let mut rng = rand::thread_rng();
    LOVE_EMOJIS[rng.gen_range(0..LOVE_EMOJIS.len())].to_string()
}

pub fn get_random_success_message() -> String {
    let mut rng = rand::thread_rng();
    SUCCESS_MESSAGES[rng.gen_range(0..SUCCESS_MESSAGES.len())].to_string()
}

pub fn get_random_error_message() -> String {
    let mut rng = rand::thread_rng();
    ERROR_MESSAGES[rng.gen_range(0..ERROR_MESSAGES.len())].to_string()
}

pub fn create_love_border(message: &str) -> String {
    let width = message.len() + 4;
    let border: String = "♥".repeat(width);
    format!("{}\n♥ {} ♥\n{}", border, message, border)
}

pub fn print_love_help() {
    println!("{}", create_love_border("💝 Love Language Quick Guide 💝").bright_cyan());
    println!("
🌟 Special Commands:
   love help   - Show this guide
   love story  - Tell a random love story
   love quote  - Share a love quote

🎨 Basic Syntax:
   heart      -> declare variables
   forever    -> declare constants
   devotion   -> define functions
   whisper    -> print values
   
❤️ Operators:
   cuddle     -> addition (+)
   kiss       -> multiply (*)
   breakup    -> subtract (-)
   split      -> divide (/)
");
}

pub fn print_random_love_quote() {
    let quotes = [
        "In code as in love, simplicity is beautiful.",
        "Every function call is a love letter to the CPU.",
        "Bug-free code is true love.",
        "Variables may be undefined, but our love for coding isn't.",
        "The best code is written with love."
    ];
    let mut rng = rand::thread_rng();
    let quote = quotes[rng.gen_range(0..quotes.len())];
    println!("{}", create_love_border(quote).bright_magenta());
}

pub fn handle_special_commands(line: &str) -> bool {
    match line.trim() {
        "love help" => {
            print_love_help();
            true
        },
        "love quote" => {
            print_random_love_quote();
            true
        },
        _ => false
    }
}