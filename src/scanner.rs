use crate::token::{keyword, Token, TokenType};

/// Cut `source` into tokens. Returns everything it managed to scan alongside every
/// error it found; the caller decides whether to go on.
pub fn scan(source: &str) -> (Vec<Token>, Vec<String>) {
    let mut s = Scanner {
        src: source.chars().collect(),
        start: 0,
        current: 0,
        line: 1,
        tokens: Vec::new(),
        errors: Vec::new(),
    };
    s.run();
    (s.tokens, s.errors)
}

struct Scanner {
    src: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
    errors: Vec<String>,
}

impl Scanner {
    fn run(&mut self) {
        // Spec 6.1: --tokens prints one token per line in order, ending with EOF, and
        // EOF carries the line of the last real token, not the last line of the file.
        // self.line has kept counting past trailing blank lines and comment lines, so
        // the EOF line is taken from the last token actually scanned; a file with no
        // tokens at all -- empty or all comments -- reports EOF at line 1.
        while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.start = self.current;
        self.line = self.tokens.last().map(|t| t.line).unwrap_or(1);
        self.add(TokenType::Eof);
    }

    fn scan_token(&mut self) {
        // Spec 1.1: whitespace separates tokens and is otherwise ignored; newlines move
        // the line counter; a comment runs from // to end of line and is discarded.
        // Spec 1.2: one character of lookahead decides ! = < > / between their one- and
        // two-character forms. Spec 1.2/1.3/1.4/1.5: everything else is dispatched.
        // Spec 5.1: an unrecognised character is 'Character is not part of any token.'
        match self.advance() {
            '(' => self.add(TokenType::LParen),
            ')' => self.add(TokenType::RParen),
            '{' => self.add(TokenType::LBrace),
            '}' => self.add(TokenType::RBrace),
            ',' => self.add(TokenType::Comma),
            ';' => self.add(TokenType::Semicolon),
            '+' => self.add(TokenType::Plus),
            '-' => self.add(TokenType::Minus),
            '*' => self.add(TokenType::Star),
            '/' => {
                if self.matches('/') {
                    while !self.at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add(TokenType::Slash);
                }
            }
            '!' => {
                if self.matches('=') {
                    self.add(TokenType::BangEqual);
                } else {
                    self.add(TokenType::Bang);
                }
            }
            '=' => {
                if self.matches('=') {
                    self.add(TokenType::EqualEqual);
                } else {
                    self.add(TokenType::Equal);
                }
            }
            '<' => {
                if self.matches('=') {
                    self.add(TokenType::LessEqual);
                } else {
                    self.add(TokenType::Less);
                }
            }
            '>' => {
                if self.matches('=') {
                    self.add(TokenType::GreaterEqual);
                } else {
                    self.add(TokenType::Greater);
                }
            }
            '"' => self.string(),
            '0'..='9' => self.number(),
            c if c.is_ascii_alphabetic() || c == '_' => self.identifier(),
            ' ' | '\t' | '\r' => {}
            '\n' => self.line += 1,
            _ => self.error(self.line, "Character is not part of any token."),
        }
    }

    fn string(&mut self) {
        // Spec 1.5: a string is any run of characters between two quotes and may span
        // lines, so every newline inside it moves the line counter. An unterminated
        // string is reported at the line it opened on. The closing quote is part of
        // the lexeme (6.1 prints it) and does not itself move the line counter.
        let open_line = self.line;
        while !self.at_end() && self.peek() != '"' {
            let c = self.advance();
            if c == '\n' {
                self.line += 1;
            }
        }
        if self.at_end() {
            self.error(open_line, "String is never closed.");
            return;
        }
        self.advance(); // the closing quote
        self.add(TokenType::Str);
    }

    fn number(&mut self) {
        // Spec 1.4: a number is one or more digits, optionally followed by '.' and one
        // or more digits. The fractional part exists only when a digit follows the dot,
        // so 5. scans as NUMBER 5 and then '.' is reported by scan_token (5.1), and .5
        // never reaches number() at all. No exponent notation: 'e' stops the scan and
        // is read as the start of an identifier, so 1e308 is NUMBER 1, IDENTIFIER e308.
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // the dot
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        self.add(TokenType::Number);
    }

    fn identifier(&mut self) {
        // Spec 1.3: an identifier starts with a letter or '_' and continues with
        // letters, digits or '_'. The whole word is read first, then keyword() decides
        // whether it scans as one of the twelve keyword types or as IDENTIFIER.
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let word: String = self.src[self.start..self.current].iter().collect();
        self.add(keyword(&word).unwrap_or(TokenType::Identifier));
    }

    // --- primitives ---------------------------------------------------------------

    fn at_end(&self) -> bool {
        self.current >= self.src.len()
    }

    fn advance(&mut self) -> char {
        let c = self.src[self.current];
        self.current += 1;
        c
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.at_end() || self.src[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.src[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.src.len() {
            '\0'
        } else {
            self.src[self.current + 1]
        }
    }

    fn add(&mut self, kind: TokenType) {
        self.tokens.push(Token {
            kind,
            lexeme: self.src[self.start..self.current].iter().collect(),
            line: self.line,
        });
    }

    fn error(&mut self, line: usize, message: &str) {
        self.errors.push(format!("[line {}] Error: {}", line, message));
    }
}
