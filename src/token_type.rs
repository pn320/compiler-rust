#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Period,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Asterisk,
    CarriageReturn,
    Tab,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,

    // Literals.
    Identifier,
    String,
    Integer,

    // Keywords.
    And,
    Else,
    False,
    Function,
    For,
    If,
    None,
    Or,
    Print,
    Return,
    This,
    True,
    Struct,
    Let,
    While,

    EOF,
}
