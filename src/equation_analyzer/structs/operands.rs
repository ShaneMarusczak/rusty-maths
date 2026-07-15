use super::token::{SpannedToken, Token};
use crate::equation_analyzer::catalog::{self, SymbolKind};
use crate::equation_analyzer::errors::EquationError;
use std::sync::OnceLock;

// One Assoc for the whole crate — the catalog owns the definition, the
// parser keeps importing it from here.
pub(crate) use crate::equation_analyzer::catalog::Assoc;

pub(crate) struct Operand {
    pub(crate) prec: u8,
    pub(crate) assoc: Assoc,
    pub(crate) is_func: bool,
    pub(crate) paren_opener: bool,
    pub(crate) token: SpannedToken,
}

/// Precedence and associativity for every named operator, resolved from the
/// catalog — the single source of truth for operator metadata — exactly once.
/// `get_operator` runs for every operator token, so it must not re-scan.
struct OperatorTable {
    plus: (u8, Assoc),
    minus: (u8, Assoc),
    star: (u8, Assoc),
    slash: (u8, Assoc),
    modulo: (u8, Assoc),
    power: (u8, Assoc),
    factorial: (u8, Assoc),
    percent: (u8, Assoc),
}

fn operator_table() -> Result<&'static OperatorTable, EquationError> {
    static TABLE: OnceLock<Option<OperatorTable>> = OnceLock::new();
    TABLE
        .get_or_init(|| {
            let meta = |name: &str| match catalog::find(name)?.kind {
                SymbolKind::Operator {
                    precedence, assoc, ..
                } => Some((precedence, assoc)),
                _ => None,
            };
            Some(OperatorTable {
                plus: meta("+")?,
                minus: meta("-")?,
                star: meta("*")?,
                slash: meta("/")?,
                modulo: meta("mod")?,
                power: meta("^")?,
                factorial: meta("!")?,
                percent: meta("%")?,
            })
        })
        .as_ref()
        .ok_or_else(|| EquationError::new("Internal error: operator missing from catalog"))
}

pub(crate) fn get_operator(spanned: SpannedToken) -> Result<Operand, EquationError> {
    let token = spanned.token;
    let (prec, assoc) = match token {
        // Call tokens never reach the operator stack: parenthesized calls
        // become frames, and pipe targets go straight to the output.
        Token::OpenParen | Token::Log { .. } => (0, Assoc::Right),
        // Unary minus is not a catalog symbol (the catalog documents `-` once,
        // as binary subtraction). It mirrors `^` so that -x^2 == -(x^2) while
        // still binding tighter than the binary operators.
        Token::UnaryMinus => (operator_table()?.power.0, Assoc::Right),
        Token::Plus => operator_table()?.plus,
        Token::Minus => operator_table()?.minus,
        Token::Star => operator_table()?.star,
        Token::Slash => operator_table()?.slash,
        Token::Modulo => operator_table()?.modulo,
        Token::Power => operator_table()?.power,
        Token::Factorial => operator_table()?.factorial,
        Token::Percent => operator_table()?.percent,
        op => {
            return Err(EquationError::spanned(
                format!("Unknown operator: {op:?}"),
                spanned.span,
            ))
        }
    };

    Ok(Operand {
        prec,
        assoc,
        is_func: matches!(token, Token::Log { .. }),
        paren_opener: matches!(token, Token::OpenParen | Token::Log { .. }),
        token: spanned,
    })
}
