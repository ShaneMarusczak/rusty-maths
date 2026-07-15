use crate::equation_analyzer::catalog::Symbol;
use crate::equation_analyzer::errors::Span;

/// What a call token dispatches to: a built-in catalog symbol, or a
/// user-defined function referenced by its index into the `Definitions`
/// that were in scope when the equation was tokenized. Index-based so the
/// token stays `Copy`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Callee {
    Catalog(&'static Symbol),
    User(usize),
}

/// A token plus the character range of the source equation it came from.
/// Synthetic tokens (the `2x` expansion, the parser's `EndCall`) carry the
/// span of the source construct that produced them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct SpannedToken {
    pub(crate) token: Token,
    pub(crate) span: Span,
}

impl SpannedToken {
    pub(crate) fn new(token: Token, span: Span) -> Self {
        SpannedToken { token, span }
    }
}

/// A lexical/RPN token. Payload-carrying variants hold everything the parser
/// and evaluator need for that token — there are no out-of-band value fields.
///
/// Symbol-carrying variants compare by catalog identity (see `Symbol`'s
/// `PartialEq`), so two tokens are equal only when they reference the same
/// catalog entry.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Token {
    Y,
    Equal,
    Comma,

    /// `log_N(` syntax — the base is parsed lexically from the `_N` suffix.
    /// Structurally distinct from `Call` because of that surface syntax.
    Log {
        base: f32,
    },

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

    Number(f32),

    /// The variable `x` — `plot()` substitutes each sample value,
    /// `calculate()` evaluates it as 0.
    X,
    End,

    Pipe,

    /// A function call — catalog or user-defined — dispatched through its
    /// `Callee`. In RPN this appears only as a pipe target (`x |> sin`): the
    /// evaluator pops one argument off the stack. Parenthesized calls are
    /// rewritten by the parser into a `CallStart`…`EndCall` frame instead.
    Call(Callee),

    /// Parser-synthesized frame opener for a parenthesized call — unary and
    /// variadic alike. Arguments collect until the matching `EndCall`, where
    /// the callee's arity is enforced. Never produced by the tokenizer.
    CallStart(Callee),

    /// Parser-synthesized frame closer; its span covers the whole call.
    /// Never produced by the tokenizer.
    EndCall(Callee),

    /// A named constant (π, e, …) — the value comes from the Symbol.
    Constant(&'static Symbol),
}
