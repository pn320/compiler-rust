use crate::token_type::TokenType;
use std::any::Any;

#[derive(Debug)]
pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: Box<dyn Any>,
    pub(crate) line: u8,
}
