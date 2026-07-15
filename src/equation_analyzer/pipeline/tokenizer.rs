use crate::equation_analyzer::catalog::{self, SymbolKind};
use crate::equation_analyzer::definitions::{Definitions, Resolved};
use crate::equation_analyzer::errors::{EquationError, Span};
use crate::equation_analyzer::structs::token::{Callee, SpannedToken, Token};
use std::collections::VecDeque;
use std::iter::Peekable;
use std::str::Chars;

/// May `c` continue an identifier? The first character must be alphabetic;
/// continuation characters may also be ASCII digits (`atan2`, `log10`).
fn continues_identifier(c: char) -> bool {
    c.is_alphabetic() || c.is_ascii_digit()
}

/// A streaming tokenizer that implements Iterator, yielding tokens on demand.
///
/// `chars` is the only cursor; `position` counts characters and doubles as
/// the source of token spans. Lookahead beyond one character clones the
/// (cheap) char iterator instead of re-indexing the source, so multi-byte
/// characters like `π` can never desynchronize scanning.
pub(crate) struct StreamingTokenizer<'a> {
    chars: Peekable<Chars<'a>>,
    position: usize,
    start_position: usize,
    previous_token: Option<Token>,
    finished: bool,
    pending_tokens: VecDeque<SpannedToken>,
    /// User definitions consulted for identifiers the catalog doesn't claim.
    defs: Option<&'a Definitions>,
}

impl<'a> StreamingTokenizer<'a> {
    pub(crate) fn new_with(
        eq: &'a str,
        defs: Option<&'a Definitions>,
    ) -> Result<Self, EquationError> {
        if eq.is_empty() {
            return Err(EquationError::new("Invalid equation supplied"));
        }

        Ok(Self {
            chars: eq.chars().peekable(),
            position: 0,
            start_position: 0,
            previous_token: None,
            finished: false,
            pending_tokens: VecDeque::new(),
            defs,
        })
    }

    /// Looks `name` up in the user definitions. Only called after the
    /// catalog has declined the name — catalog resolution always wins.
    fn resolve_user(&self, name: &str) -> Option<Resolved> {
        self.defs.and_then(|d| d.resolve(name))
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.chars.next();
        if ch.is_some() {
            self.position += 1;
        }
        ch
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    /// The character `n` places past the current peek position.
    fn peek_nth(&self, n: usize) -> Option<char> {
        self.chars.clone().nth(n)
    }

    /// The span of the lexeme currently being scanned.
    fn lexeme_span(&self) -> Span {
        Span::new(self.start_position, self.position)
    }

    fn emit(&mut self, token: Token) -> SpannedToken {
        self.previous_token = Some(token);
        SpannedToken::new(token, self.lexeme_span())
    }

    /// An error pointing at the lexeme currently being scanned.
    fn err_here(&self, message: impl Into<String>) -> EquationError {
        EquationError::spanned(message, self.lexeme_span())
    }

    fn scan_digits(&mut self) -> String {
        let mut literal = String::new();

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '_' {
                literal.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if self.peek() == Some('.') && self.peek_nth(1).is_some_and(|c| c.is_ascii_digit()) {
            literal.push('.');
            self.advance();

            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    literal.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        literal
    }

    fn scan_number(&mut self) -> Result<SpannedToken, EquationError> {
        let literal = self.scan_digits();
        // Underscores are digit-grouping sugar (`1_000`); the float parser
        // doesn't know them. Commas can't serve this role — they separate
        // function arguments.
        let val: f32 = literal
            .replace('_', "")
            .parse()
            .map_err(|_| self.err_here(format!("Invalid number: {}", literal)))?;

        // A bare `x` right after a number literal is a juxtaposed coefficient
        // (`2x`). A longer identifier is not (`2xor` stays `2`, `xor`).
        if self.peek() == Some('x') && !self.peek_nth(1).is_some_and(continues_identifier) {
            self.advance();
            return Ok(self.coefficient_x(val));
        }

        Ok(self.emit(Token::Number(val)))
    }

    /// Emit the token stream for a juxtaposed coefficient (`2x`).
    ///
    /// Juxtaposition binds tighter than a *preceding* high-precedence operator
    /// (`1/2x` is 1/(2x), `x^2x` is x^(2x)) but looser than an exponent on the
    /// x itself (`2x^3` is 2(x³)). No single operator precedence can express
    /// both, so the pair is parenthesized exactly when the previous token
    /// binds at least as tightly as multiplication. Pinned by
    /// plot_test_linear/exp_2/exp_3.
    ///
    /// Every synthetic token carries the span of the whole `2x` lexeme.
    fn coefficient_x(&mut self, coef: f32) -> SpannedToken {
        if coef == 1.0 {
            return self.emit(Token::X);
        }

        let needs_parens = matches!(
            self.previous_token,
            Some(Token::Power | Token::Star | Token::Slash | Token::Modulo | Token::Percent)
        );

        let span = self.lexeme_span();
        if needs_parens {
            self.pending_tokens
                .push_back(SpannedToken::new(Token::OpenParen, span));
        }
        self.pending_tokens.extend([
            SpannedToken::new(Token::Number(coef), span),
            SpannedToken::new(Token::Star, span),
            SpannedToken::new(Token::X, span),
        ]);
        if needs_parens {
            self.pending_tokens
                .push_back(SpannedToken::new(Token::CloseParen, span));
        }

        // The queue was populated just above, so the front always exists;
        // fall back to a bare X only to keep this branch panic-free.
        let first = self
            .pending_tokens
            .pop_front()
            .unwrap_or(SpannedToken::new(Token::X, span));
        self.previous_token = Some(first.token);
        first
    }

    fn scan_word(&mut self) -> Result<SpannedToken, EquationError> {
        // The two reserved single letters — the variable and the equation
        // marker — are by far the most common identifiers; resolving them
        // here avoids building a String. A longer word starting with x/y
        // falls through to the general scan and can never resolve to them.
        if let Some(c @ ('x' | 'y')) = self.peek() {
            if !self.peek_nth(1).is_some_and(continues_identifier) {
                self.advance();
                let token = if c == 'x' { Token::X } else { Token::Y };
                return Ok(self.emit(token));
            }
        }

        let mut name = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphabetic() || (!name.is_empty() && ch.is_ascii_digit()) {
                name.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Pipe target: name must be a unary function — catalog or
        // user-defined (user functions are always unary) — no parens follow.
        if matches!(self.previous_token, Some(Token::Pipe)) {
            let callee = match catalog::find(&name).filter(|s| s.kind.is_unary()) {
                Some(sym) => Callee::Catalog(sym),
                None => match self.resolve_user(&name) {
                    Some(Resolved::Function(i)) => Callee::User(i),
                    _ => {
                        return Err(self.err_here(format!(
                            "'{}' cannot be used after '|>'; only unary functions are allowed",
                            name
                        )))
                    }
                },
            };

            if self.peek() == Some('(') {
                return Err(self.err_here(format!(
                    "Function '{}' after '|>' must not be called with parentheses; the piped value is its argument",
                    name
                )));
            }

            return Ok(self.emit(Token::Call(callee)));
        }

        // Handle log base
        if self.peek() == Some('_') {
            if name != "log" {
                // Point at the unexpected underscore itself.
                let underscore = Span::new(self.position, self.position + 1);
                return Err(EquationError::spanned("Invalid input", underscore));
            }
            self.advance(); // consume '_'

            if !self.peek().is_some_and(|c| c.is_ascii_digit()) {
                return Err(self.err_here("Invalid use of log"));
            }

            let base_literal = self.scan_digits();
            let base: f32 = base_literal
                .parse()
                .map_err(|_| self.err_here("Invalid log base"))?;

            if self.peek() != Some('(') {
                return Err(self.err_here("Invalid use of log: expected '(' after the base"));
            }
            self.advance(); // consume '('

            return Ok(self.emit(Token::Log { base }));
        }

        // Named constants (π, e, and multi-char aliases like `pi`).
        if let Some(sym) =
            catalog::find(&name).filter(|s| matches!(s.kind, SymbolKind::Constant(_)))
        {
            return Ok(self.emit(Token::Constant(sym)));
        }

        // Word operators (`17 mod 5`). The catalog documents them;
        // each one maps to its structural Token here.
        if catalog::find(&name).is_some_and(|s| matches!(s.kind, SymbolKind::Operator { .. })) {
            return match name.as_str() {
                "mod" => Ok(self.emit(Token::Modulo)),
                _ => Err(self.err_here(format!(
                    "Operator '{}' cannot be written as a word here",
                    name
                ))),
            };
        }

        // User definitions resolve after every catalog form: values become
        // number literals carrying the identifier's span, functions become
        // calls. The value is read at tokenize time, which *is* call time
        // for function bodies — they're compiled fresh each evaluation pass.
        let called_with_parens = self.peek() == Some('(');
        match self.resolve_user(&name) {
            Some(Resolved::Value(v)) if !called_with_parens => {
                return Ok(self.emit(Token::Number(v)));
            }
            Some(Resolved::Value(_)) => {
                return Err(self.err_here(format!("'{}' is a value, not a function", name)));
            }
            Some(Resolved::Function(i)) if called_with_parens => {
                self.advance(); // consume '('
                return Ok(self.emit(Token::Call(Callee::User(i))));
            }
            Some(Resolved::Function(_)) => {
                return Err(self.err_here(format!("Function '{}' requires parentheses", name)));
            }
            None => {}
        }

        if !called_with_parens {
            // A known function used bare gets a pointer at the fix; a name
            // nothing claims gets called what it is: unknown.
            return Err(if catalog::find(&name).is_some() {
                self.err_here(format!("Function '{}' requires parentheses", name))
            } else {
                self.err_here(format!("Unknown name '{}'", name))
            });
        }

        let sym = catalog::find(&name)
            .filter(|s| s.kind.is_unary() || s.kind.is_variadic())
            .ok_or_else(|| self.err_here(format!("Invalid function name {}", name)))?;

        self.advance(); // consume '('
        Ok(self.emit(Token::Call(Callee::Catalog(sym))))
    }

    fn scan_token(&mut self) -> Result<Option<SpannedToken>, EquationError> {
        // Skip whitespace
        while matches!(self.peek(), Some(' ' | '\r' | '\t')) {
            self.advance();
        }

        // Check if we're done
        let Some(c) = self.peek() else {
            if !self.finished {
                self.finished = true;
                self.start_position = self.position;
                return Ok(Some(self.emit(Token::End)));
            }
            return Ok(None);
        };

        self.start_position = self.position;

        if c.is_ascii_digit() {
            return self.scan_number().map(Some);
        }

        // Identifiers are scanned in full before any single-character
        // meaning applies, so catalog names may start with x/y/e/π.
        if c.is_alphabetic() {
            return self.scan_word().map(Some);
        }

        self.advance();
        let token = match c {
            '=' => self.emit(Token::Equal),
            ',' => self.emit(Token::Comma),
            '*' => self.emit(Token::Star),
            '/' => self.emit(Token::Slash),
            '+' => self.emit(Token::Plus),
            '!' => self.emit(Token::Factorial),
            '%' => {
                if self.peek() == Some('%') {
                    self.advance();
                    self.emit(Token::Modulo)
                } else {
                    self.emit(Token::Percent)
                }
            }
            '-' => {
                // Binary subtraction after an operand, unary negation
                // otherwise. Percent is in the operand list because it's
                // postfix: `50% - 3`.
                if matches!(
                    self.previous_token,
                    Some(
                        Token::Constant(_)
                            | Token::Number(_)
                            | Token::CloseParen
                            | Token::X
                            | Token::Factorial
                            | Token::Percent
                    )
                ) {
                    self.emit(Token::Minus)
                } else {
                    self.emit(Token::UnaryMinus)
                }
            }
            '|' => {
                // Accept either `|` or `|>` as the pipe operator.
                if self.peek() == Some('>') {
                    self.advance();
                }
                self.emit(Token::Pipe)
            }
            '(' => self.emit(Token::OpenParen),
            ')' => self.emit(Token::CloseParen),
            '^' => self.emit(Token::Power),
            _ => return Err(self.err_here("Invalid input")),
        };

        Ok(Some(token))
    }
}

impl<'a> Iterator for StreamingTokenizer<'a> {
    type Item = Result<SpannedToken, EquationError>;

    fn next(&mut self) -> Option<Self::Item> {
        // First drain any pending tokens from multi-token expansions
        if let Some(spanned) = self.pending_tokens.pop_front() {
            self.previous_token = Some(spanned.token);
            return Some(Ok(spanned));
        }

        // Otherwise scan the next token
        match self.scan_token() {
            Ok(Some(token)) => Some(Ok(token)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}
