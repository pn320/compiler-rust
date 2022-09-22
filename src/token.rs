use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub enum Literal {
    StrLiteral(String),
    IntLiteral(u32),
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: u32,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: u32,
    ) -> Token {
        Token { token_type, lexeme, literal, line }
    }
}
