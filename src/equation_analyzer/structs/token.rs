use crate::equation_analyzer::catalog::Symbol;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) numeric_value_1: f32,
    pub(crate) numeric_value_2: f32,
    /// Backing catalog entry when the token corresponds to a named symbol
    /// (function call, constant, operator). `None` for structural tokens
    /// like `OpenParen`, `Number`, `End`, `X`.
    pub(crate) symbol: Option<&'static Symbol>,
}

// Manual PartialEq: symbols are compared by pointer identity because all
// Symbol references originate from a single &'static CATALOG slice, so
// pointer equality precisely means "same catalog entry" — cheaper and
// safer than deriving PartialEq on the Symbol struct (which would compare
// through fn pointers, whose addresses aren't guaranteed unique).
impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token_type == other.token_type
            && self.numeric_value_1 == other.numeric_value_1
            && self.numeric_value_2 == other.numeric_value_2
            && match (self.symbol, other.symbol) {
                (Some(a), Some(b)) => std::ptr::eq(a, b),
                (None, None) => true,
                _ => false,
            }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum TokenType {
    Y,
    Equal,
    Comma,

    /// log_N syntax — the base is baked into `numeric_value_1` on the token.
    /// Structurally distinct from `Call` because its base is parsed lexically.
    Log,

    OpenParen,
    CloseParen,

    Factorial,

    Star,
    Slash,
    Plus,
    Minus,
    UnaryMinus,
    Power,
    Modulo,
    Percent,

    Number,

    X,
    End,

    Pipe,

    /// A unary function call whose behavior comes from `token.symbol`.
    /// The evaluator pops one arg, dispatches through the Symbol.
    Call,

    /// Emitted by the parser as the postfix marker for a comma-separated
    /// call (variadic — includes atan2 / ch). Carries the backing Symbol
    /// on `token.symbol` so the evaluator can pop the frame and dispatch.
    EndCall,

    /// A named mathematical constant (π, e, …). Value comes from
    /// `token.symbol` via `SymbolKind::Constant(v)`.
    Constant,
}
