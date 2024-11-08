# Love Language 💝

Love Language is a fun, love-themed programming language that brings romance to coding! Express your algorithms with affection using love-themed keywords and operators.

## 🌟 Features

- Love-themed keywords and operators
- Variables and constants
- Functions with parameters and return values
- Control flow statements (if-else, while)
- Basic arithmetic operations
- String operations
- Boolean logic

## 💖 Quick Start

### Installation

1. Make sure you have Rust installed on your system
2. Clone the repository:

```bash
git clone https://github.com/starc007/love-language.git
cd love-language
```

3. Build the project:

```bash
cargo build --release
```

### Running Love Language Programs

You can run Love Language in two modes:

1. Interactive REPL:

```bash
cargo run
```

2. Run a .love file:

```bash
cargo run -- path/to/your/script.love
```

## 💕 Language Guide

### Basic Syntax

```love
// Variables
heart x match 10;              // Variable declaration
forever LOVE match 100;        // Constant declaration

// Output
whisper "Hello Love!";         // Print to console
whisper x;                     // Print variable value

// Arithmetic
heart sum match x cuddle y;    // Addition
heart diff match x breakup y;  // Subtraction
heart prod match x kiss y;     // Multiplication
heart quot match x split y;    // Division

// Functions
devotion add(x: number, y: number) -> number {
    promise x cuddle y;
}

// Control Flow
crush (x admires y) {          // If statement
    whisper "x is greater!";
} butterflies {                // Else statement
    whisper "y is greater!";
}

// Loops
dating (count envies 5) {      // While loop
    whisper count;
    count match count cuddle 1;
}
```

### 💝 Operators

| Love Operator | Traditional | Description    |
| ------------- | ----------- | -------------- |
| `cuddle`      | `+`         | Addition       |
| `breakup`     | `-`         | Subtraction    |
| `kiss`        | `*`         | Multiplication |
| `split`       | `/`         | Division       |
| `admires`     | `>`         | Greater than   |
| `envies`      | `<`         | Less than      |
| `soulmate`    | `==`        | Equal to       |
| `heartbreak`  | `!=`        | Not equal to   |

### 🎯 Keywords

| Love Keyword  | Traditional | Description          |
| ------------- | ----------- | -------------------- |
| `heart`       | `let`       | Variable declaration |
| `forever`     | `const`     | Constant declaration |
| `devotion`    | `function`  | Function declaration |
| `whisper`     | `print`     | Output statement     |
| `crush`       | `if`        | If statement         |
| `butterflies` | `else`      | Else statement       |
| `dating`      | `while`     | While loop           |
| `promise`     | `return`    | Return statement     |

## 📝 Example Programs

### Simple Calculator

```love
devotion add(x: number, y: number) -> number {
    promise x cuddle y;
}

heart a match 10;
heart b match 5;
heart result match add(a, b);
whisper "The sum is:";
whisper result;
```

### Love Counter

```love
heart count match 0;
whisper "Counting with love:";

dating (count envies 5) {
    whisper "Love count:";
    whisper count;
    count match count cuddle 1;
}
```

## 🤝 Contributing

Contributions are welcome! Feel free to:

- Report bugs
- Suggest new features
- Add new love-themed operators
- Improve documentation
- Create example programs

## 💌 Contact

For questions, suggestions, or love letters about the language, please open an issue on GitHub.

Remember: Code with love! ❤️
