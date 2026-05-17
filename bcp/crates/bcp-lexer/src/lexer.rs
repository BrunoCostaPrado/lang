use crate::token::{Token, TokenType};
use std::collections::HashMap;

pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("let".to_string(), TokenType::Let);
        keywords.insert("const".to_string(), TokenType::Const);
        keywords.insert("fn".to_string(), TokenType::Fn);
        keywords.insert("return".to_string(), TokenType::Return);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("while".to_string(), TokenType::While);
        keywords.insert("for".to_string(), TokenType::For);
        keywords.insert("struct".to_string(), TokenType::Struct);
        keywords.insert("enum".to_string(), TokenType::Enum);
        keywords.insert("true".to_string(), TokenType::True);
        keywords.insert("false".to_string(), TokenType::False);
        keywords.insert("null".to_string(), TokenType::Null);
        keywords.insert("mut".to_string(), TokenType::Mut);
        keywords.insert("pub".to_string(), TokenType::Pub);
        keywords.insert("import".to_string(), TokenType::Import);

        Lexer {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
            keywords,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while self.pos < self.source.len() {
            let c = self.peek();

            if c.is_whitespace() && c != '\n' {
                self.advance();
                continue;
            }

            if c == '\n' {
                self.advance();
                tokens.push(Token::new(TokenType::Newline, "\n".to_string(), self.line - 1, 0));
                continue;
            }

            match c {
                '(' => {
                    self.advance();
                    tokens.push(self.make_token(TokenType::OpenParen, "("));
                }
                ')' => {
                    self.advance();
                    tokens.push(self.make_token(TokenType::CloseParen, ")"));
                }
                '{' => {
                    self.advance();
                    tokens.push(self.make_token(TokenType::OpenBrace, "{"));
                }
                '}' => {
                    self.advance();
                    tokens.push(self.make_token(TokenType::CloseBrace, "}"));
                }
                '[' => {
                    self.advance();
                    tokens.push(self.make_token(TokenType::OpenBracket, "["));
                }
                ']' => {
                    self.advance();
                    tokens.push(self.make_token(TokenType::CloseBracket, "]"));
                }
                ',' => {
                    self.advance();
                    tokens.push(self.make_token(TokenType::Comma, ","));
                }
                ':' => {
                    self.advance();
                    tokens.push(self.make_token(TokenType::Colon, ":"));
                }
                '.' => {
                    self.advance();
                    tokens.push(self.make_token(TokenType::Dot, "."));
                }
                '+' => {
                    self.advance();
                    tokens.push(self.make_token(TokenType::Plus, "+"));
                }
                '-' => {
                    self.advance();
                    if self.matches('>') {
                        tokens.push(self.make_token(TokenType::Arrow, "->"));
                    } else {
                        tokens.push(self.make_token(TokenType::Minus, "-"));
                    }
                }
                '*' => {
                    self.advance();
                    tokens.push(self.make_token(TokenType::Star, "*"));
                }
                '/' => {
                    self.advance();
                    if self.matches('/') {
                        while self.pos < self.source.len() && self.peek() != '\n' {
                            self.advance();
                        }
                    } else {
                        tokens.push(self.make_token(TokenType::Slash, "/"));
                    }
                }
                '%' => {
                    self.advance();
                    tokens.push(self.make_token(TokenType::Percent, "%"));
                }
                '!' => {
                    self.advance();
                    if self.matches('=') {
                        tokens.push(self.make_token(TokenType::BangEqual, "!="));
                    } else {
                        tokens.push(self.make_token(TokenType::Bang, "!"));
                    }
                }
                '=' => {
                    self.advance();
                    if self.matches('=') {
                        tokens.push(self.make_token(TokenType::EqualEqual, "=="));
                    } else {
                        tokens.push(self.make_token(TokenType::Equal, "="));
                    }
                }
                '<' => {
                    self.advance();
                    if self.matches('=') {
                        tokens.push(self.make_token(TokenType::LessEqual, "<="));
                    } else {
                        tokens.push(self.make_token(TokenType::Less, "<"));
                    }
                }
                '>' => {
                    self.advance();
                    if self.matches('=') {
                        tokens.push(self.make_token(TokenType::GreaterEqual, ">="));
                    } else {
                        tokens.push(self.make_token(TokenType::Greater, ">"));
                    }
                }
                '&' => {
                    self.advance();
                    if self.matches('&') {
                        tokens.push(self.make_token(TokenType::And, "&&"));
                    } else {
                        return Err(format!("Unexpected '&' at {}:{}", self.line, self.col));
                    }
                }
                '|' => {
                    self.advance();
                    if self.matches('|') {
                        tokens.push(self.make_token(TokenType::Or, "||"));
                    } else {
                        tokens.push(self.make_token(TokenType::Pipe, "|"));
                    }
                }
                '"' => {
                    self.advance();
                    let start_col = self.col;
                    let mut s = String::new();
                    while self.pos < self.source.len() && self.peek() != '"' {
                        if self.peek() == '\n' {
                            return Err(format!(
                                "Unterminated string at {}:{}",
                                self.line, start_col
                            ));
                        }
                        s.push(self.advance());
                    }
                    if self.pos >= self.source.len() {
                        return Err(format!(
                            "Unterminated string at {}:{}",
                            self.line, start_col
                        ));
                    }
                    self.advance(); // closing "
                    tokens.push(Token::new(TokenType::String, s, self.line, start_col));
                }
                _ if c.is_ascii_digit() => {
                    let start_col = self.col;
                    let mut num = String::new();
                    while self.pos < self.source.len() && self.peek().is_ascii_digit() {
                        num.push(self.advance());
                    }
                    tokens.push(Token::new(TokenType::Number, num, self.line, start_col));
                }
                _ if c.is_ascii_alphabetic() || c == '_' => {
                    let start_col = self.col;
                    let mut ident = String::new();
                    while self.pos < self.source.len()
                        && (self.peek().is_ascii_alphanumeric() || self.peek() == '_')
                    {
                        ident.push(self.advance());
                    }
                    let kind = self
                        .keywords
                        .get(&ident)
                        .copied()
                        .unwrap_or(TokenType::Identifier);
                    tokens.push(Token::new(kind, ident, self.line, start_col));
                }
                _ => {
                    return Err(format!(
                        "Unexpected character '{}' at {}:{}",
                        c, self.line, self.col
                    ));
                }
            }
        }

        tokens.push(Token::new(TokenType::Eof, "EOF".to_string(), self.line, self.col));
        Ok(tokens)
    }

    fn peek(&self) -> char {
        self.source[self.pos]
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.pos];
        self.pos += 1;
        if c == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        c
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.pos < self.source.len() && self.source[self.pos] == expected {
            self.advance();
            true
        } else {
            false
        }
    }

    fn make_token(&self, kind: TokenType, value: &str) -> Token {
        let col = if self.col >= value.len() {
            self.col - value.len()
        } else {
            1
        };
        Token::new(kind, value.to_string(), self.line, col)
    }
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Lexer::new(source);
    lexer.tokenize()
}
