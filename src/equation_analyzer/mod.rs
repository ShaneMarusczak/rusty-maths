/// Equation analyzer for mathematical expression parsing, evaluation, and plotting.
///
/// # Quick Start
///
/// ```
/// use rusty_maths::equation_analyzer::calculator;
///
/// // Evaluate an expression
/// let result = calculator::calculate("2 + 3 * 4").unwrap();
/// assert_eq!(result, 14.0);
///
/// // Plot a function
/// let points = calculator::plot("x^2", -5.0, 5.0, 0.5).unwrap();
/// ```
// Public API
pub mod calculator;
pub mod catalog;
pub mod definitions;
pub mod errors;

/// The pipeline's error type and its character-span companion, re-exported
/// for convenience.
///
/// ```
/// use rusty_maths::equation_analyzer::{calculator, EquationError};
///
/// let err: EquationError = calculator::calculate("2 + foo(3)").unwrap_err();
/// assert!(err.span.is_some());
/// ```
pub use errors::{EquationError, Span};

/// User definitions — named values and single-parameter functions —
/// re-exported for convenience.
///
/// ```
/// use rusty_maths::equation_analyzer::{calculator, Definitions};
///
/// let mut defs = Definitions::new();
/// defs.define_value("a", 2.0).unwrap();
/// assert_eq!(calculator::calculate_with("a + 1", &defs).unwrap(), 3.0);
/// ```
pub use definitions::{Definition, Definitions};

/// The plot-point type returned by [`calculator::plot`], re-exported so
/// downstream crates can name it.
///
/// ```
/// use rusty_maths::equation_analyzer::Point;
///
/// let p = Point::new(2.0, 4.0);
/// assert_eq!((p.x, p.y), (2.0, 4.0));
/// ```
pub use utils::Point;

// Internal modules (not part of public API)
pub(crate) mod pipeline;
pub(crate) mod structs;
mod tests;
pub(crate) mod utils;
