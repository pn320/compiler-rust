use crate::token::Literal;
use crate::token::Literal::{IntLiteral, StrLiteral};
use crate::token_type::TokenType;
use crate::{token, Token};
use std::thread::sleep;

use colored::Colorize;

pub struct Scanner {
    pub(crate) source: String,
    pub(crate) tokens: Vec<Token>,
    pub(crate) line: u32,
    pub(crate) start_pos: u32,
    pub(crate) current_pos: u32,
    // separation of pos_x and current_pos for error reporting
    pub(crate) pos_x: u32,
}

impl Scanner {
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start_pos = self.current_pos;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
        });
        return &self.tokens;
    }

    fn is_at_end(&self) -> bool {
        self.current_pos >= self.source.len() as u32
    }

    fn scan_token(&mut self) -> () {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Period),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '*' => self.add_token(TokenType::Asterisk),
            '!' => {
                if self.match_advance('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_advance('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_advance('=') {
                    self.add_token(TokenType::LessThanEqual)
                } else {
                    self.add_token(TokenType::LessThan)
                }
            }
            '>' => {
                if self.match_advance('=') {
                    self.add_token(TokenType::GreaterThanEqual)
                } else {
                    self.add_token(TokenType::GreaterThan)
                }
            }
            '/' => {
                if self.match_advance('/') {
                    loop {
                        if self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            '"' => self.string_literal_scan(),
            '\r' => self.add_token(TokenType::CarriageReturn),
            '\t' => self.add_token(TokenType::Tab),
            '\n' => {
                self.line += 1;
                self.pos_x = 0;
            }
            ' ' => self.add_token(TokenType::Space),
            _ => self.error_report(format!("Unexpected token")),
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current_pos as usize).unwrap();
    }

    fn string_literal_scan(&mut self) {
        use crate::token::Literal::{IntLiteral, StrLiteral};
        loop {
            if self.peek() != '"' && !self.is_at_end() {
                if self.peek() == '\n' {
                    self.line += 1;
                }
                self.advance();
            } else {
                break;
            }
        }
        if self.is_at_end() {
            self.error_report("Unterminated string".to_string());
        }

        self.advance();
        let literal =
            self.source[(self.start_pos + 1) as usize..(self.current_pos - 1) as usize].to_string();
        self.add_token_with_literal(TokenType::String, StrLiteral(literal));
    }

    fn match_advance(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current_pos as usize).unwrap() != expected {
            return false;
        }
        self.update_pos();
        return true;
    }

    fn add_token(&mut self, token_type: TokenType) -> () {
        self.tokens.push(Token {
            token_type,
            lexeme: self.source[self.start_pos as usize..self.current_pos as usize].to_string(),
            literal: None,
            line: self.line,
        })
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Literal) {
        self.tokens.push(Token {
            token_type,
            lexeme: self.source[self.start_pos as usize..self.current_pos as usize].to_string(),
            literal: Some(literal),
            line: self.line,
        })
    }

    fn advance(&mut self) -> char {
        let advanced_char = self.source.chars().nth(self.current_pos as usize).unwrap();
        self.update_pos();
        return advanced_char;
    }

    fn update_pos(&mut self) -> () {
        self.current_pos += 1;
        self.pos_x += 1;
    }

    fn error_report(&mut self, message: String) -> () {
        self.report_line(message);
    }

    fn report_line(&self, message: String) -> () {
        println!("{}: {}", "Syntax error".red(), message);
        let error_source = self.source.split("\n").nth((self.line - 1) as usize).unwrap();
        println!("   {} | {}", self.line, error_source);
        let ptr_arrows = format!("^---").red();
        let shift_amt = 7;
        println!("   {: >width$}", ptr_arrows, width = (self.pos_x + shift_amt) as usize);
    }
}
