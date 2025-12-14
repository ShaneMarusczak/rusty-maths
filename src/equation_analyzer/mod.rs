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

// Internal modules (not part of public API)
pub(crate) mod pipeline;
pub(crate) mod structs;
pub(crate) mod utils;
mod tests;
