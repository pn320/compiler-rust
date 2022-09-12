#[derive(Debug)]
pub(crate) enum TokenType {
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
    Space,
    Asterisk,
    CarriageReturn,
    Tab,
    Newline,

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
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Function,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}
