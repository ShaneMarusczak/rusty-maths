use super::token::{Token, TokenType};

pub(crate) struct Operand {
    pub(crate) prec: usize,
    pub(crate) assoc: Assoc,
    pub(crate) is_func: bool,
    pub(crate) paren_opener: bool,
    pub(crate) token: Token,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Assoc {
    Right,
    Left,
}

pub(crate) fn get_operator(operator: Token) -> Result<Operand, String> {
    match operator.token_type {
        TokenType::OpenParen => Ok(get_op(operator, 0, Assoc::Right, false, true)),
        TokenType::Factorial => Ok(get_op(operator, 5, Assoc::Left, false, false)),

        TokenType::Plus | TokenType::Minus => Ok(get_op(operator, 2, Assoc::Left, false, false)),
        TokenType::Star | TokenType::Slash | TokenType::Percent | TokenType::Modulo => {
            Ok(get_op(operator, 3, Assoc::Left, false, false))
        }
        TokenType::UnaryMinus | TokenType::Power => {
            Ok(get_op(operator, 4, Assoc::Right, false, false))
        }
        TokenType::Sqrt
        | TokenType::Ln
        | TokenType::Abs
        | TokenType::Max
        | TokenType::Min
        | TokenType::Sin
        | TokenType::Cos
        | TokenType::Tan
        | TokenType::Asin
        | TokenType::Acos
        | TokenType::Atan
        | TokenType::Log => Ok(get_op(operator, 0, Assoc::Right, true, true)),
        op => Err(format!("Unknown operator: {:?}", op)),
    }
}

fn get_op(token: Token, prec: usize, assoc: Assoc, is_func: bool, paren_opener: bool) -> Operand {
    Operand {
        token,
        prec,
        assoc,
        is_func,
        paren_opener,
    }
}
