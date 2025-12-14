/// Equation analyzer with three pipeline implementations
///
/// This module provides mathematical expression parsing, evaluation, and plotting.
///
/// # Pipeline Implementations
///
/// - **vec_pipeline**: Traditional fully-buffered implementation (baseline)
/// - **hybrid_pipeline**: Streaming tokenizer + buffered parser (recommended, 1.6x faster)
/// - **full_pipeline**: Fully streaming with minimal buffers (1.4x faster)
///
/// # Quick Start
///
/// ```
/// use rusty_maths::equation_analyzer::hybrid_pipeline::calculator;
///
/// // Evaluate an expression
/// let result = calculator::calculate("2 + 3 * 4").unwrap();
/// assert_eq!(result, 14.0);
///
/// // Plot a function
/// let points = calculator::plot("x^2", -5.0, 5.0, 0.5).unwrap();
/// ```
///
/// # Default Export
///
/// For convenience, the hybrid_pipeline (recommended) is re-exported as `calculator`:
///
/// ```
/// use rusty_maths::equation_analyzer::calculator;
///
/// let result = calculator::calculate("sin(π/2)").unwrap();
/// ```
// Recommended default: hybrid_pipeline (fastest and most practical)
pub mod calculator {
    pub use super::hybrid_pipeline::calculator::*;
}

// Three pipeline implementations - choose based on your needs
pub mod vec_pipeline;
pub mod hybrid_pipeline;
pub mod full_pipeline;

// Internal modules (not part of public API)
pub(crate) mod core;
pub(crate) mod structs;
pub(crate) mod utils;
mod tests;
