#[derive(Debug, PartialEq)]
pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) literal: String,
}

#[derive(Debug, PartialEq)]
pub(crate) enum TokenType {
    Y,
    Equal,
    Comma,

    _Pi,
    _E,

    Sin,
    Cos,
    Tan,
    Max,
    Abs,
    Sqrt,
    Min,
    Ln,
    Log,

    OpenParen,
    CloseParen,

    Star,
    Slash,
    Plus,
    Minus,
    Power,

    Number,

    X,

    End,
}
