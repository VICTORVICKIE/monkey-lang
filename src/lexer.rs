#![allow(dead_code)]

use anyhow::Result;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-character tokens
    LParen,
    RParen,
    LCurly,
    RCurly,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    FSlash,
    Asterisk,

    // One or two character tokens
    Bang,
    NotEqual,
    Assign,
    Equal,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,

    // Literals
    Identifier(String),
    Number(String),

    // Keywords
    If,
    Else,
    Elif,
    True,
    False,
    Let,
    Function,
    Return,

    // End of file
    EOF,
    Illegal,
}
#[derive(Debug, Clone)]
pub struct Position {
    line: usize,
    column: usize,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub position: Position,
}

#[derive(Debug)]
pub struct Lexer {
    input: Vec<u8>,
    ch: u8,
    curr_pos: usize,
    peek_pos: usize,
}

impl Into<Vec<Token>> for Lexer {
    fn into(self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut lexer = self;

        while let Ok(token) = lexer.next_token() {
            tokens.push(token.clone());
            if let TokenType::EOF = token.token_type {
                break;
            }
        }
        return tokens;
    }
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lex = Lexer {
            curr_pos: 0,
            peek_pos: 0,
            ch: 0,
            input: input.into_bytes(),
        };
        lex.read_char();

        return lex;
    }

    fn peek_char(&self) -> u8 {
        if self.peek_pos >= self.input.len() {
            return 0;
        } else {
            return self.input[self.peek_pos];
        }
    }

    fn read_char(&mut self) {
        if self.peek_pos >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.peek_pos];
        }

        self.curr_pos = self.peek_pos;
        self.peek_pos += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let start_pos = self.curr_pos;

        while self.ch.is_ascii_alphanumeric() || self.ch == b'_' {
            self.read_char();
        }

        String::from_utf8_lossy(&self.input[start_pos..self.curr_pos]).to_string()
    }

    fn read_number(&mut self) -> String {
        let start_pos = self.curr_pos;

        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        String::from_utf8_lossy(&self.input[start_pos..self.curr_pos]).to_string()
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        let token_type = match self.ch {
            b'{' => TokenType::LCurly,
            b'}' => TokenType::RCurly,
            b'(' => TokenType::LParen,
            b')' => TokenType::RParen,
            b',' => TokenType::Comma,
            b';' => TokenType::Semicolon,
            b'+' => TokenType::Plus,
            b'-' => TokenType::Minus,
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    TokenType::NotEqual
                } else {
                    TokenType::Bang
                }
            }
            b'>' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    TokenType::GreaterThanOrEqual
                } else {
                    TokenType::GreaterThan
                }
            }
            b'<' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    TokenType::LessThanOrEqual
                } else {
                    TokenType::LessThan
                }
            }
            b'*' => TokenType::Asterisk,
            b'/' => TokenType::FSlash,
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    TokenType::Equal
                } else {
                    TokenType::Assign
                }
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let identifier = self.read_identifier();
                let token_type = match identifier.as_str() {
                    "fn" => TokenType::Function,
                    "let" => TokenType::Let,
                    "if" => TokenType::If,
                    "elif" => TokenType::Elif,
                    "else" => TokenType::Else,
                    "false" => TokenType::False,
                    "true" => TokenType::True,
                    "return" => TokenType::Return,
                    _ => TokenType::Identifier(identifier.clone()),
                };
                return Ok(Token {
                    token_type,
                    literal: identifier,
                    position: Position {
                        line: 0,
                        column: self.curr_pos,
                    },
                });
            }
            b'0'..=b'9' => {
                let number = self.read_number();
                return Ok(Token {
                    token_type: TokenType::Number(number.clone()),
                    literal: number,
                    position: Position {
                        line: 0,
                        column: self.curr_pos,
                    },
                });
            }
            0 => TokenType::EOF,
            _ => TokenType::Illegal,
        };
        let token = Token {
            token_type,
            literal: String::from_utf8_lossy(&[self.ch]).to_string(),
            position: Position {
                line: 0,
                column: self.curr_pos,
            },
        };
        self.read_char();

        return Ok(token);
    }
}
