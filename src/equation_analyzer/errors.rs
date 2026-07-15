//! The equation analyzer's error type: a message plus an optional character
//! span into the source equation, so consumers (like rm-repl) can point at
//! the offending input instead of only echoing a sentence.

use std::fmt;

/// A character range into the source equation, half-open (`[start, end)`).
///
/// Units are **characters, not bytes** — `π` counts as one. Indexing the
/// source with `&eq[span.start..span.end]` would be wrong for multi-byte
/// input; use `eq.chars().skip(start).take(end - start)` instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }

    /// Number of characters covered; a caret line should draw at least one.
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// An error from tokenizing, parsing, or evaluating an equation.
///
/// `span` locates the offending region when one exists; errors about the
/// expression as a whole (empty input, leftover operands) carry `None`.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct EquationError {
    pub message: String,
    pub span: Option<Span>,
}

impl EquationError {
    /// An error with no meaningful location.
    pub fn new(message: impl Into<String>) -> Self {
        EquationError {
            message: message.into(),
            span: None,
        }
    }

    /// An error pointing at a character range of the source equation.
    pub fn spanned(message: impl Into<String>, span: Span) -> Self {
        EquationError {
            message: message.into(),
            span: Some(span),
        }
    }

    /// Shifts the span right by `delta` characters. For embedders that
    /// evaluate a substring of a larger input (rm-repl's `|`-separated
    /// multi-equation graphs), this maps the span back onto the full text.
    pub fn offset(mut self, delta: usize) -> Self {
        if let Some(span) = self.span.as_mut() {
            span.start += delta;
            span.end += delta;
        }
        self
    }
}

impl fmt::Display for EquationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.span {
            // 1-based: "character 1" is the first character a human counts.
            Some(span) => write!(f, "{} at character {}", self.message, span.start + 1),
            None => write!(f, "{}", self.message),
        }
    }
}

impl std::error::Error for EquationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_appends_one_based_position() {
        let err = EquationError::spanned("Invalid input", Span::new(2, 3));
        assert_eq!(err.to_string(), "Invalid input at character 3");

        let err = EquationError::new("Invalid equation supplied");
        assert_eq!(err.to_string(), "Invalid equation supplied");
    }

    #[test]
    fn offset_shifts_span_only_when_present() {
        let err = EquationError::spanned("x", Span::new(1, 3)).offset(4);
        assert_eq!(err.span, Some(Span::new(5, 7)));

        let err = EquationError::new("x").offset(4);
        assert_eq!(err.span, None);
    }

    #[test]
    fn span_len_saturates() {
        assert_eq!(Span::new(3, 7).len(), 4);
        assert_eq!(Span::new(3, 3).len(), 0);
        assert!(Span::new(3, 3).is_empty());
    }
}
