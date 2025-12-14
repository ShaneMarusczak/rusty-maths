use crate::equation_analyzer::structs::token::Token;

/// Evaluates a parsed equation in Reverse Polish Notation (RPN).
///
/// This is a wrapper around the core evaluator that handles &[Token] input.
///
/// # Arguments
/// * `parsed_eq` - A slice of tokens in RPN format
/// * `x` - Optional value of the variable x (defaults to 0.0 if None)
///
/// # Returns
/// * `Ok(f32)` - The result of the evaluation
/// * `Err(String)` - An error message if evaluation fails
pub(crate) fn evaluate(parsed_eq: &[Token], x: impl Into<Option<f32>>) -> Result<f32, String> {
    // Since Token is Copy, we can iterate over references and dereference cheaply
    crate::equation_analyzer::core::evaluator::evaluate(parsed_eq.iter().copied(), x)
}
