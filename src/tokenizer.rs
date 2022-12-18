use crate::token::Literal::{IntLiteral, StrLiteral};
use crate::token_type::TokenType;
use crate::Token;
use std::collections::HashMap;

use colored::Colorize;
use std::string::String;
use crate::token::Literal;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    line: u32,
    start_pos: u32,
    current_pos: u32,
    // mostly for error reporting
    col_offset: u32,
    keywords_map: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(
        source: String,
        tokens: Vec<Token>,
        line: u32,
        start_pos: u32,
        current_pos: u32,
        pos_x: u32,
        keywords_map: HashMap<String, TokenType>,
    ) -> Scanner {
        Scanner { source, tokens, line, start_pos, current_pos, col_offset: pos_x, keywords_map }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start_pos = self.current_pos;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::EOF, "".to_string(), None, self.line));
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
            '\'' => self.string_literal_scan(),
            '\r' => self.add_token(TokenType::CarriageReturn),
            '\t' => self.add_token(TokenType::Tab),
            '\n' => {
                self.line += 1;
                self.col_offset = 0;
            }
            ' ' => {}
            _ => {
                if c.is_digit(10) {
                    self.int_literal_scan()
                } else if c.is_alphabetic() || c == '_' {
                    self.keyword_scan()
                } else {
                    self.error_report(format!("Unexpected token"))
                }
            }
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current_pos as usize).unwrap();
    }

    fn int_literal_scan(&mut self) {
        loop {
            if self.peek().is_digit(10) {
                self.advance();
            } else {
                break;
            }
        }
        let num =
            self.source[self.start_pos as usize..self.current_pos as usize].parse::<u32>().unwrap();
        self.add_token_with_literal(TokenType::Integer, IntLiteral(num));
    }

    fn keyword_scan(&mut self) {
        loop {
            if self.peek().is_alphanumeric() || self.peek() == '_' {
                self.advance();
            } else {
                break;
            }
        }
        let keyword = self.source[self.start_pos as usize..self.current_pos as usize].to_string();
        let token_type = self.keywords_map.get(&keyword);
        let tkn_type = token_type.cloned();
        match tkn_type {
            Some(tkn_type) => self.add_token_with_literal(tkn_type, StrLiteral(keyword)),
            None => self.add_token(TokenType::Identifier),
        }
    }

    fn multi_line_comment_scan(&mut self) {}

    fn string_literal_scan(&mut self) {
        loop {
            if self.peek() != '"' && self.peek() != '\'' && !self.is_at_end() {
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
            return;
        }

        // to get the closing double quotes
        self.advance();
        self.add_token(TokenType::String);
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
        self.tokens.push(Token::new(
            token_type,
            self.source[self.start_pos as usize..self.current_pos as usize].to_string(),
            None,
            self.line,
        ));
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Literal) {
        self.tokens.push(Token::new(
            token_type,
            self.source[self.start_pos as usize..self.current_pos as usize].to_string(),
            Some(literal),
            self.line,
        ));
    }

    fn advance(&mut self) -> char {
        let advanced_char = self.source.chars().nth(self.current_pos as usize).unwrap();
        self.update_pos();
        return advanced_char;
    }

    fn update_pos(&mut self) -> () {
        self.current_pos += 1;
        self.col_offset += 1;
    }

    fn error_report(&mut self, message: String) -> () {
        self.report_line(message);
    }

    fn report_line(&self, message: String) -> () {
        println!("{}:", "Error".red());
        let error_source = self.source.split("\n").nth((self.line - 1) as usize).unwrap();
        println!("   {} | {}", self.line, error_source);
        let error_arrow_msg = format!("^---").red();
        let shift_amt = 7;
        println!(
            "   {: >width$} {}",
            error_arrow_msg,
            message,
            width = (self.col_offset + shift_amt) as usize
        );
    }
}
