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
    NegPi,
    NegE,

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

    Factorial,

    Star,
    Slash,
    Plus,
    Minus,
    Power,
    Modulo,
    Percent,

    Number,

    X,

    End,
}
