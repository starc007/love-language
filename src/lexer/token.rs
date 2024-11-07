use logos::Logos;
use std::fmt;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {

    #[regex(r"[ \t\n\f]+", logos::skip)]  // Skip whitespace
    #[regex(r"//[^\n]*\n?", logos::skip)]  // Skip line comments

    #[error]
    #[regex(r".", priority = 0)]  // Catch any single char for error reporting
    Error,

    // Keywords with love theme
    #[token("heart")]
    Heart,
    #[token("forever")]
    Forever,
    #[token("devotion")]
    Devotion,
    #[token("crush")]
    Crush,
    #[token("butterflies")]
    Butterflies,
    #[token("lonely")]
    Lonely,
    #[token("dating")]
    Dating,
    #[token("promise")]
    Promise,
    #[token("whisper")]
    Whisper,
    #[token("relationship")]
    Relationship,

    // Types
    #[token("number")]
    TypeNumber,
    #[token("text")]
    TypeText,
    #[token("feeling")]
    TypeFeeling,

    // Literals
    #[regex(r"-?[0-9]+", |lex| lex.slice().parse().ok())]
    Number(i64),
    #[regex(r#""[^"]*""#, |lex| Some(String::from(&lex.slice()[1..lex.slice().len()-1])))]
    Text(String),
    #[token("yes")]
    Yes,
    #[token("no")]
    No,

    // Love-themed operators
    #[token("cuddle")]
    Cuddle,
    #[token("breakup")]
    Breakup,
    #[token("kiss")]
    Kiss,
    #[token("split")]
    Split,
    #[token("match")]
    Match,
    #[token("soulmate")]
    Soulmate,
     #[token("not")]
    Not,
    #[token("and")]
    And,
    #[token("or")]
    Or,

    // Comparison operators with unique love theme
    #[token("admires")]        // Greater than (>)
    GreaterThan,
    #[token("adores")]         // Greater than or equal (>=)
    GreaterThanEqual,
    #[token("envies")]         // Less than (<)
    LessThan,
    #[token("yearns")]         // Less than or equal (<=)
    LessThanEqual,
    #[token("heartbreak")]     // Not equal (!=)
    NotEqual,

    // Delimiters
     #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token("->")]
    Arrow,

    // Identifiers (must come after keywords)
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| Some(String::from(lex.slice())))]
    Identifier(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Error => write!(f, "ERROR"),
            Token::Heart => write!(f, "heart"),
            Token::Forever => write!(f, "forever"),
            Token::Devotion => write!(f, "devotion"),
            Token::Crush => write!(f, "crush"),
            Token::Butterflies => write!(f, "butterflies"),
            Token::Lonely => write!(f, "lonely"),
            Token::Dating => write!(f, "dating"),
            Token::Promise => write!(f, "promise"),
            Token::Whisper => write!(f, "whisper"),
            Token::Relationship => write!(f, "relationship"),
            Token::TypeNumber => write!(f, "number"),
            Token::TypeText => write!(f, "text"),
            Token::TypeFeeling => write!(f, "feeling"),
            Token::Number(n) => write!(f, "{}", n),
            Token::Text(s) => write!(f, "\"{}\"", s),
            Token::Yes => write!(f, "yes"),
            Token::No => write!(f, "no"),
            Token::Cuddle => write!(f, "cuddle"),
            Token::Breakup => write!(f, "breakup"),
            Token::Kiss => write!(f, "kiss"),
            Token::Split => write!(f, "split"),
            Token::Match => write!(f, "match"),
            Token::Soulmate => write!(f, "soulmate"),
            Token::GreaterThan => write!(f, "admires"),
            Token::GreaterThanEqual => write!(f, "adores"),
            Token::LessThan => write!(f, "envies"),
            Token::LessThanEqual => write!(f, "yearns"),
            Token::NotEqual => write!(f, "heartbreak"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::Semicolon => write!(f, ";"),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::Not => write!(f, "not"),
            Token::And => write!(f, "and"),
            Token::Or => write!(f, "or"),
            Token::Arrow => write!(f, "->"),
            Token::Identifier(name) => write!(f, "{}", name),
        }
    }
}
