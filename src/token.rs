// GIVEN, do not edit. This file is part of the compiler you are handed, not the
// compiler you write. Read it anyway: what you write has to fit it.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,

    Plus,
    Minus,
    Star,
    Slash,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    Str,
    Number,

    And,
    Else,
    False,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    True,
    Var,
    While,

    Eof,
}

impl TokenType {
    /// The name the `--tokens` dump prints. These are the thirty-four names in
    /// docs/language-spec.md section 1.2, and the golden tests match them character for
    /// character, so they are part of the contract and not a debugging convenience.
    pub fn name(self) -> &'static str {
        match self {
            TokenType::LParen => "LPAREN",
            TokenType::RParen => "RPAREN",
            TokenType::LBrace => "LBRACE",
            TokenType::RBrace => "RBRACE",
            TokenType::Comma => "COMMA",
            TokenType::Semicolon => "SEMICOLON",

            TokenType::Plus => "PLUS",
            TokenType::Minus => "MINUS",
            TokenType::Star => "STAR",
            TokenType::Slash => "SLASH",

            TokenType::Bang => "BANG",
            TokenType::BangEqual => "BANG_EQUAL",
            TokenType::Equal => "EQUAL",
            TokenType::EqualEqual => "EQUAL_EQUAL",
            TokenType::Greater => "GREATER",
            TokenType::GreaterEqual => "GREATER_EQUAL",
            TokenType::Less => "LESS",
            TokenType::LessEqual => "LESS_EQUAL",

            TokenType::Identifier => "IDENTIFIER",
            TokenType::Str => "STRING",
            TokenType::Number => "NUMBER",

            TokenType::And => "AND",
            TokenType::Else => "ELSE",
            TokenType::False => "FALSE",
            TokenType::Fun => "FUN",
            TokenType::If => "IF",
            TokenType::Nil => "NIL",
            TokenType::Or => "OR",
            TokenType::Print => "PRINT",
            TokenType::Return => "RETURN",
            TokenType::True => "TRUE",
            TokenType::Var => "VAR",
            TokenType::While => "WHILE",

            TokenType::Eof => "EOF",
        }
    }
}

pub fn keyword(word: &str) -> Option<TokenType> {
    match word {
        "and" => Some(TokenType::And),
        "else" => Some(TokenType::Else),
        "false" => Some(TokenType::False),
        "fun" => Some(TokenType::Fun),
        "if" => Some(TokenType::If),
        "nil" => Some(TokenType::Nil),
        "or" => Some(TokenType::Or),
        "print" => Some(TokenType::Print),
        "return" => Some(TokenType::Return),
        "true" => Some(TokenType::True),
        "var" => Some(TokenType::Var),
        "while" => Some(TokenType::While),
        _ => None,
    }
}

/// Render source text so it occupies exactly one line. A Kobo string may span lines
/// (spec section 1.5), and both `--tokens` and `--ast` promise one record per line, so the raw
/// newline has to be shown rather than printed.
pub fn one_line(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    /// Exactly the source text the token was cut from, quotes included for strings.
    pub lexeme: String,
    pub line: usize,
}
