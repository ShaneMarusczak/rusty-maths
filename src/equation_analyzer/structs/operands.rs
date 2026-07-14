use super::token::{Token, TokenType};

// One Assoc for the whole crate — the catalog owns the definition, the
// parser keeps importing it from here.
pub(crate) use crate::equation_analyzer::catalog::Assoc;

pub(crate) struct Operand {
    pub(crate) prec: usize,
    pub(crate) assoc: Assoc,
    pub(crate) is_func: bool,
    pub(crate) paren_opener: bool,
    pub(crate) token: Token,
}

pub(crate) fn get_operator(operator: Token) -> Result<Operand, String> {
    match operator.token_type {
        TokenType::OpenParen => Ok(get_op(operator, 0, Assoc::Right, false, true)),
        // Postfix operators bind tightest: 2^50% is 2^(0.5), -3! is -(6).
        TokenType::Factorial | TokenType::Percent => {
            Ok(get_op(operator, 5, Assoc::Left, false, false))
        }

        TokenType::Plus | TokenType::Minus => Ok(get_op(operator, 2, Assoc::Left, false, false)),
        TokenType::Star | TokenType::Slash | TokenType::Modulo => {
            Ok(get_op(operator, 3, Assoc::Left, false, false))
        }
        TokenType::UnaryMinus | TokenType::Power => {
            Ok(get_op(operator, 4, Assoc::Right, false, false))
        }
        TokenType::Call | TokenType::Log => Ok(get_op(operator, 0, Assoc::Right, true, true)),
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
