use crate::token_type::TokenType;

#[derive(Debug)]
pub(crate) enum Literal {
    StrLiteral(String),
    IntLiteral(u32),
}

#[derive(Debug)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: Option<Literal>,
    pub(crate) line: u32,
}
