#[derive(Debug, PartialEq)]
pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) numeric_value_1: f32,
    pub(crate) numeric_value_2: f32,
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

    Med,
    EndMed,

    Mode,
    EndMode,

    Max,
    EndMax,

    Abs,
    Sqrt,

    Min,
    EndMin,

    Ln,
    Log,

    Avg,
    EndAvg,

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

#[derive(Debug, PartialEq)]
pub(crate) enum ParamToken {
    None,
    Avg,
    Min,
    Max,
    Med,
    Mode,
}
